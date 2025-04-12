section .text
	global _start

_start:
	mov r8, 5
	mov r9, 5
	imul r8, r9

	mov rax, 0x2000001
	mov rdi, r8
	syscall
