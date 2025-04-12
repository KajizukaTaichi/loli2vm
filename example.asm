section .text
	global _start

_start:
	mov r8, 1
	mov r9, 2
	mov r10, 3
	mov r11, 4
	mov r12, 5
	add r11, r12
	add r10, r11
	add r9, r10
	add r8, r9

	mov rax, 0x2000001
	mov rdi, r8
	syscall
