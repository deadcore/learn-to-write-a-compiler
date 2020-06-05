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
	call	_printf
	nop
	leave
	ret

	.globl	_main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$20, %r8
	movq	$10, %r9
	imulq	%r8, %r9
	movq	$10, %r8
	addq	%r9, %r8
	movq	%r8,%rdi
	call	printint
	movq	$5, %r8
	movq	$5, %r9
	movq	$10, %r10
	imulq	%r9, %r10
	addq	%r8, %r10
	movq	%r10,%rdi
	call	printint
	movl	$0, %eax
	popq	%rbp
	ret
