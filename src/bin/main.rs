use clap::Clap;

use learn_to_write_a_compiler::compiler::Result;
use learn_to_write_a_compiler::compiler::Compiler;
use log::LevelFilter;
use log::info;

/// A NES Emulator written in rust.
#[derive(Clap)]
#[clap(version = "1.0", author = "Jack L. <admin@deadcore.co.uk>")]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0", author = "Jack L. <admin@deadcore.co.uk>")]
    Compile(Compile),
}

/// A subcommand for controlling compiler
#[derive(Clap)]
pub struct Compile {
    /// The main file to compile
    #[clap(short, long)]
    file: String,
}

fn main() -> Result<()> {
    env_logger::builder().init();

    let opts: Opts = Opts::parse();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => info!("No verbose info"),
        1 => info!("Some verbose info"),
        2 => info!("Tons of verbose info"),
        3 | _ => info!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Compile(t) => return compile(t)
    }
}

fn compile(c: Compile) -> Result<()> {
    let compiler = Compiler::from_arg_matches();

    let file = c.file.as_ref();

    return compiler.compile(file);
}