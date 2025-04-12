section .text
	global _start

_start:
	mov r8, 5
	mov r9, 1
	mov r10, 2
	add r9, r10
	imul r8, r9
	mov r9, 3
	sub r8, r9

	mov rax, 0x2000001
	mov rdi, r8
	syscall
