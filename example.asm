section .text
	global _start

_start:
	mov r8, 2
	mov r9, 3
	add r8, r9
	mov r9, 1
	sub r8, r9

	mov rax, 0x2000001
	mov rdi, r8
	syscall
