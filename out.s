	.text
.LC0:
	.string	"%d\n"

printint:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, %esi
	leaq	.LC0(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	ret

	.globl	main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	2, r7d
	movq	2, r8d
	movq	1, r9d
	imulq	r8d, r9d
	addq	r7d, r9d
	movl	$0, %eax
	popq	%rbp
	ret
