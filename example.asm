section .text
	global _start

_start:
	mov r8, 1
	mov r9, 2
	add r8, r9
	mov r9, 10
	mov r10, 3
	mov r11, 4
	sub r10, r11
	add r9, r10
	mov r10, 5
	sub r9, r10
	add r8, r9

	mov rax, 0x2000001
	mov rdi, r8
	syscall
