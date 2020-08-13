	# Start of preamble
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
	# End of preamble
	# Starting users code
	.comm	fred,8,8
	movq	$5, %r8
	movq	%r8, fred(%rip)

	movq	jim(%rip), %r8

	.comm	jim,8,8
	movq	$12, %r8
	movq	%r8, jim(%rip)

	movq	fred(%rip), %r8

	movq	jim(%rip), %r9

	addq	%r8, %r9
	movq	%r9,%rdi
	call	printint
	# Ending users code
	movl	$0, %eax             # Start of postamble
	popq	%rbp
	ret                         # End of postamble
