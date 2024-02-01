# gcc -nostdlib -nostartfiles -nodefaultlibs -static hello_lsp.s -o hello_lsp

.data

.equ    SYSCALL_WRITE, 1
.equ    SYSCALL_EXIT, 60
.equ    STDOUT, 1
.equ    EXIT_SUCCESS, 0 

message:
    .asciz "Hello, LSP!\n" # Our message to print

.equ MSG_LEN, . - message # The length of our message

.global _start

.text

_start:
    # write (1, msg, len(msg))
    mov $SYSCALL_WRITE, %rax
    mov $STDOUT, %rdi
    mov $message, %rsi
    mov $MSG_LEN, %rdx
    syscall

    # exit(0)
    mov $SYSCALL_EXIT, %rax
    xor %rdi, %rdi
    syscall
