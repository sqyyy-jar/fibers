use std::{
    alloc,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    panic::{catch_unwind, AssertUnwindSafe},
    ptr,
};

extern "C" {
    // Both of these functions must return the same type as either of them can return from a fiber.
    //
    // The returned boolean states if the fiber ended.

    fn fiber_enter(
        sp: *mut *mut u8,
        func: extern "C" fn(fiber: *mut u8, data: *const u8),
        data: *mut u8,
    );

    fn fiber_yield(sp: *mut *mut u8);
}

#[repr(C)]
pub struct Fiber<F> {
    /// This value must not be moved
    inner: Box<InnerFiber<F>>,
}

impl<F> Fiber<F>
where
    F: FnOnce(ReturnFiber),
{
    pub fn spawn(stack: FiberStack, f: F) -> Self {
        unsafe {
            Fiber {
                inner: Box::new(InnerFiber {
                    sp: stack.base.add(stack.layout.size()),
                    ended: false,
                    stack,
                    f: MaybeDrop::Drop(f),
                }),
            }
        }
    }

    pub fn is_alive(&self) -> bool {
        !self.inner.ended
    }

    pub fn yield_to(&mut self) {
        self.inner.yield_to()
    }
}

#[repr(C)]
struct InnerFiber<F> {
    /// Stack pointer of the opposite/inactive side
    sp: *mut u8,
    ended: bool,
    stack: FiberStack,
    f: MaybeDrop<F>,
}

impl<F> InnerFiber<F>
where
    F: FnOnce(ReturnFiber),
{
    fn yield_to(&mut self) {
        extern "C" fn exec<F>(fiber: *mut u8, f: *const u8)
        where
            F: FnOnce(ReturnFiber),
        {
            let fiber = fiber as *mut InnerFiber<F>;
            let f = unsafe { ptr::read(f as *const F) };
            let return_fiber = ReturnFiber::new(fiber);
            match catch_unwind(AssertUnwindSafe(|| (f)(return_fiber))) {
                Ok(()) => (),
                Err(err) => eprintln!("Unhandled panic in fiber: {err:?}"),
            }
            unsafe { (*fiber).ended = true };
        }

        if self.ended {
            panic!("The fiber has already ended");
        }
        if self.f.is_drop() {
            unsafe {
                self.f.to_no_drop();
                fiber_enter(
                    &mut self.sp,
                    exec::<F>,
                    self.f.as_mut() as *mut F as *mut u8,
                );
            }
        } else {
            unsafe { fiber_yield(&mut self.sp) };
        }
    }
}

/// This type is used by a fiber to yield to the caller.
#[repr(C)]
pub struct ReturnFiber<'a> {
    sp: *mut *mut u8,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> ReturnFiber<'a> {
    fn new<F>(fiber: *mut InnerFiber<F>) -> Self {
        unsafe {
            Self {
                sp: &mut (*fiber).sp,
                _lifetime: PhantomData,
            }
        }
    }

    pub fn yield_to(&self) {
        unsafe { fiber_yield(self.sp) };
    }
}

pub struct FiberStack {
    base: *mut u8,
    layout: alloc::Layout,
}

impl FiberStack {
    /// # Safety
    ///
    /// This function panics if the size is not aligned to 16 or zero. If the allocation
    /// was not successful, [None] is returned.
    pub fn new(size: usize) -> Option<Self> {
        unsafe {
            assert_eq!(size % 16, 0, "Stack size must be aligned to 16");
            assert!(size > 0, "Stack must not be empty");
            let layout = alloc::Layout::array::<u8>(size).ok()?;
            let base = alloc::alloc(layout);
            if base.is_null() {
                return None;
            }
            Some(Self { base, layout })
        }
    }

    pub fn bytes(&mut self) -> (*mut u8, *mut u8) {
        unsafe { (self.base.add(self.layout.size()), self.base) }
    }
}

impl Drop for FiberStack {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.base, self.layout) };
    }
}

enum MaybeDrop<T> {
    Drop(T),
    NoDrop(ManuallyDrop<T>),
    None,
}

impl<T> MaybeDrop<T> {
    fn is_drop(&self) -> bool {
        matches!(self, Self::Drop(_))
    }

    fn to_no_drop(&mut self) {
        let Self::Drop(_) = self else {
            return;
        };
        let Self::Drop(value) = mem::replace(self, Self::None) else {
            unreachable!();
        };
        *self = Self::NoDrop(ManuallyDrop::new(value));
    }

    fn as_mut(&mut self) -> &mut T {
        match self {
            MaybeDrop::Drop(value) => value,
            MaybeDrop::NoDrop(value) => value,
            _ => unreachable!(),
        }
    }
}
