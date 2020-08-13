use std::io::Write;

use crate::asm::registers::{RegisterIndex, Registers};

pub mod registers;

pub fn cgadd<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    writeln!(out, "\taddq\t{}, {}", r1.name(), r2.name())?;
    registers.free_register(r1);
    return Ok(r2);
}

pub fn cgmul<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    writeln!(out, "\timulq\t{}, {}", r1.name(), r2.name())?;
    registers.free_register(r1);
    return Ok(r2);
}

pub fn cgsub<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    writeln!(out, "\tsubq\t{}, {}", r2.name(), r1.name())?;
    registers.free_register(r2);
    return Ok(r1);
}

pub fn cgdiv<W: Write>(r1: RegisterIndex, r2: RegisterIndex, registers: &mut Registers, mut out: W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    writeln!(out, "\tmovq\t{}, {}", r1.name(), "%rax")?;
    writeln!(out, "\tcqo")?;
    writeln!(out, "\tidivq\t{}", r2.name())?;
    writeln!(out, "\tmovq\t{}, {}", "%rax", r1.name())?;
    registers.free_register(r2);
    return Ok(r1);
}


pub fn cgcomment<W: Write>(mut out: W, comment: &str) -> core::result::Result<(), Box<dyn std::error::Error>> {
    writeln!(out, "\t# {}", comment)?;
    Ok(())
}

pub fn cgpreamble<W: Write>(mut out: W) -> core::result::Result<(), Box<dyn std::error::Error>> {
    writeln!(out, "\t# Start of preamble")?;
    writeln!(out, "\t.text")?;
    writeln!(out, ".LC0:")?;
    writeln!(out, "\t.string\t\"%d\\n\"\n")?;
    writeln!(out, "printint:")?;
    writeln!(out, "\tpushq\t%rbp")?;
    writeln!(out, "\tmovq\t%rsp, %rbp")?;
    writeln!(out, "\tsubq\t$16, %rsp")?;
    writeln!(out, "\tmovl\t%edi, -4(%rbp)")?;
    writeln!(out, "\tmovl\t-4(%rbp), %eax")?;
    writeln!(out, "\tmovl\t%eax, %esi")?;
    writeln!(out, "\tleaq\t.LC0(%rip), %rdi")?;
    writeln!(out, "\tmovl\t$0, %eax")?;
    writeln!(out, "\tcall\t_printf")?;
    writeln!(out, "\tnop")?;
    writeln!(out, "\tleave")?;
    writeln!(out, "\tret")?;
    writeln!(out, "\t.globl\t_main")?;
    writeln!(out, "_main:")?;
    writeln!(out, "\tpushq\t%rbp")?;
    writeln!(out, "\tmovq\t%rsp, %rbp")?;
    writeln!(out, "\t# End of preamble")?;
    Ok(())
}

pub fn cgpostamble<W: Write>(mut out: W) -> core::result::Result<(), Box<dyn std::error::Error>> {
    writeln!(out, "\tmovl	$0, %eax             # Start of postamble")?;
    writeln!(out, "\tpopq	%rbp")?;
    writeln!(out, "\tret                         # End of postamble")?;
    Ok(())
}

pub fn cgload<W: Write>(value: u32, registers: &mut Registers, mut out: W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    let r = registers.allocate_register();
    writeln!(out, "\tmovq\t${}, {}", value, r.name())?;
    return Ok(r);
}

pub fn cgprintint<W: Write>(r: RegisterIndex, mut out: W) -> core::result::Result<(), Box<dyn std::error::Error>> {
    writeln!(out, "\tmovq\t{},%rdi", r.name())?;
    writeln!(out, "\tcall\tprintint")?;
    Ok(())
}

pub fn cgglobsym<W: Write>(sym: &str, out: &mut W) -> core::result::Result<(), Box<dyn std::error::Error>> {
    writeln!(out, "\t.comm\t{},8,8", sym)?;
    Ok(())
}

// Similarly, we need a function to save a register into a variable:
pub fn cgstorglob<W: Write>(sym: &str, r: RegisterIndex, out: &mut W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    writeln!(out, "\tmovq\t{}, {}(%rip)\n", r.name(), sym)?;
    return Ok(r);
}

// You would have noticed that I changed the name of the old cgload() function to cgloadint().
// This is more specific. We now have a function to load the value out of a global variable (in cg.c):
pub fn cgloadglob<W: Write>(sym: &str, registers: &mut Registers, mut out: W) -> core::result::Result<RegisterIndex, Box<dyn std::error::Error>> {
    // Get a new register
    let r = registers.allocate_register();

    // Print out the code to initialise it
    writeln!(out, "\tmovq\t{}(%rip), {}\n", sym, r.name())?;
    return Ok(r);
}
