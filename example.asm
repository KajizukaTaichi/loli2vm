section .text
	global _start

_start:
	mov r8, 1
	mov r9, 2
	mov r10, 3
	add r9, r10
	sub r8, r9
	mov r9, 10
	add r8, r9

	mov rax, 0x2000001
	mov rdi, r8
	syscall
