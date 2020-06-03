use std::fmt;
use std::io::{BufWriter, Write};

use crate::asm::registers::{RegisterIndex, Registers};

pub mod registers;

pub fn cgadd<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> RegisterIndex {
    writeln!(out, "\taddq\t{}, {}", r1.name(), r2.name());
    registers.free_register(r1);
    return r2;
}

pub fn cgmul<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> RegisterIndex {
    writeln!(out, "\timulq\t{}, {}", r1.name(), r2.name());
    registers.free_register(r1);
    return r2;
}

pub fn cgsub<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> RegisterIndex {
    writeln!(out, "\tsubq\t{}, {}", r2.name(), r1.name());
    registers.free_register(r2);
    return r1;
}

pub fn cgdiv<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> RegisterIndex {
    writeln!(out, "\tmovq\t{}, {}", r1.name(), "%rax");
    writeln!(out, "\tcqo");
    writeln!(out, "\tidivq\t{}", r2.name());
    writeln!(out, "\tmovq\t{}, {}", "%rax", r1.name());
    registers.free_register(r2);
    return r1;
}

// Print out the assembly preamble
pub fn cgpreamble<W: Write>(mut out: W) {
    writeln!(out, "\t.text");
    writeln!(out, ".LC0:");
    writeln!(out, "\t.string\t\"%d\\n\"\n");
    writeln!(out, "printint:");
    writeln!(out, "\tpushq\t%rbp");
    writeln!(out, "\tmovq\t%rsp, %rbp");
    writeln!(out, "\tsubq\t$16, %rsp");
    writeln!(out, "\tmovl\t%edi, -4(%rbp)");
    writeln!(out, "\tmovl\t-4(%rbp), %eax");
    writeln!(out, "\tmovl\t%eax, %esi");
    writeln!(out, "\tleaq\t.LC0(%rip), %rdi");
    writeln!(out, "\tmovl\t$0, %eax");
    writeln!(out, "\tcall\t_printf");
    writeln!(out, "\tnop");
    writeln!(out, "\tleave");
    writeln!(out, "\tret");
    writeln!(out, "");
    writeln!(out, "\t.globl\t_main");
    writeln!(out, "_main:");
    writeln!(out, "\tpushq\t%rbp");
    writeln!(out, "\tmovq\t%rsp, %rbp");
}

pub fn cgpostamble<W: Write>(mut out: W) {
    writeln!(out, "\tmovl	$0, %eax");
    writeln!(out, "\tpopq	%rbp");
    writeln!(out, "\tret");
}

pub fn cgload<W: Write>(value: u32, registers: &mut Registers, mut out: W) -> RegisterIndex {
    let r = registers.allocate_register();
    writeln!(out, "\tmovq\t${}, {}", value, r.name());
    return r;
}

pub fn cgprintint<W: Write>(r: RegisterIndex, mut out: W) {
    writeln!(out, "\tmovq\t{},%rdi", r.name());
    writeln!(out, "\tcall\tprintint");
}

// Call printint() with the given register
// void cgprintint(int r) {
// fprintf(Outfile, "\tmovq\t%s, %%rdi\n", reglist[r]);
// fprintf(Outfile, "\tcall\tprintint\n");
// free_register(r);
// }