// System V AMD64 ABI
// args:         rdi, rsi, rdx, rcx, r8, r9
// non-volatile: rbx, rsp, rbp, r12-r15

.intel_syntax noprefix
.section .text
.global fiber_enter
.global fiber_yield

// union Fiber {
//   fiber: *mut InnerFiber,
//   sp: *mut *mut u8,
// }

// enters a fiber
fiber_enter: // (fiber: Fiber, func: fn, data: *const fn)
    // save registers
    mov [rsp - 6 * 8], rbx
    mov [rsp - 5 * 8], rbp
    mov [rsp - 4 * 8], r12
    mov [rsp - 3 * 8], r13
    mov [rsp - 2 * 8], r14
    mov [rsp - 1 * 8], r15
    // save fiber
    mov rbx, rdi
    // swap stacks
    mov rcx, rsp
    mov rsp, [rbx]
    mov [rbx], rcx
    // call func
    mov rcx, rsi
    mov rsi, rdx
    call rcx
    // swap stacks
    mov rcx, rsp
    mov rsp, [rbx]
    mov [rbx], rcx
    // restore registers
    mov rbx, [rsp - 6 * 8]
    mov rbp, [rsp - 5 * 8]
    mov r12, [rsp - 4 * 8]
    mov r13, [rsp - 3 * 8]
    mov r14, [rsp - 2 * 8]
    mov r15, [rsp - 1 * 8]
    ret

// swaps the fibers
fiber_yield: // (fiber: Fiber)
    // save registers
    mov [rsp - 6 * 8], rbx
    mov [rsp - 5 * 8], rbp
    mov [rsp - 4 * 8], r12
    mov [rsp - 3 * 8], r13
    mov [rsp - 2 * 8], r14
    mov [rsp - 1 * 8], r15
    // swap stacks
    mov rsi, rsp
    mov rsp, [rdi]
    mov [rdi], rsi
    // restore registers
    mov rbx, [rsp - 6 * 8]
    mov rbp, [rsp - 5 * 8]
    mov r12, [rsp - 4 * 8]
    mov r13, [rsp - 3 * 8]
    mov r14, [rsp - 2 * 8]
    mov r15, [rsp - 1 * 8]
    ret
