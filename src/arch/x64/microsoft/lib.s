// Microsoft x64 calling convention
// args:         rcx, rdx, r8, r9
// non-volatile: rbx, rbp, rdi, rsi, rsp, r12-r15

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
    mov [rsp - 8 * 8], rbx
    mov [rsp - 7 * 8], rbp
    mov [rsp - 6 * 8], rdi
    mov [rsp - 5 * 8], rsi
    mov [rsp - 4 * 8], r12
    mov [rsp - 3 * 8], r13
    mov [rsp - 2 * 8], r14
    mov [rsp - 1 * 8], r15
    // save fiber
    mov rbx, rcx
    // swap stacks
    mov r9, rsp
    mov rsp, [rbx]
    mov [rbx], r9
    // call func
    mov r9, rdx
    mov rdx, r8
    call r9
    // swap stacks
    mov r9, rsp
    mov rsp, [rbx]
    mov [rbx], r9
    // restore registers
    mov rbx, [rsp - 8 * 8]
    mov rbp, [rsp - 7 * 8]
    mov rdi, [rsp - 6 * 8]
    mov rsi, [rsp - 5 * 8]
    mov r12, [rsp - 4 * 8]
    mov r13, [rsp - 3 * 8]
    mov r14, [rsp - 2 * 8]
    mov r15, [rsp - 1 * 8]
    ret

// swaps the fibers
fiber_yield: // (fiber: Fiber)
    // save registers
    mov [rsp - 8 * 8], rbx
    mov [rsp - 7 * 8], rbp
    mov [rsp - 6 * 8], rdi
    mov [rsp - 5 * 8], rsi
    mov [rsp - 4 * 8], r12
    mov [rsp - 3 * 8], r13
    mov [rsp - 2 * 8], r14
    mov [rsp - 1 * 8], r15
    // swap stacks
    mov rdx, rsp
    mov rsp, [rcx]
    mov [rcx], rdx
    // restore registers
    mov rbx, [rsp - 8 * 8]
    mov rbp, [rsp - 7 * 8]
    mov rdi, [rsp - 6 * 8]
    mov rsi, [rsp - 5 * 8]
    mov r12, [rsp - 4 * 8]
    mov r13, [rsp - 3 * 8]
    mov r14, [rsp - 2 * 8]
    mov r15, [rsp - 1 * 8]
    ret

