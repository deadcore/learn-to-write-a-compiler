use std::fs;
use std::fs::File;
use std::io::Write;

use clap::Clap;
use log::debug;
use log::LevelFilter;

use learn_to_write_a_compiler::compiler::Compiler;
use learn_to_write_a_compiler::scanner::{Token, TokenIterator};

/// A language compiler written in rust.
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
    #[clap(version = "1.0", author = "Jack L. <admin@deadcore.co.uk>")]
    Print(Print),
}

/// A subcommand for controlling compiler
#[derive(Clap)]
pub struct Compile {
    /// The main file to compile
    #[clap(short, long)]
    file: String,
}

#[derive(Clap)]
pub struct Print {
    /// The main file to compile
    #[clap(short, long)]
    file: String,
}

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let mut logger = env_logger::builder();
    let opts: Opts = Opts::parse();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    if option_env!("RUST_LOG").is_none() {
        logger.filter_level(match opts.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            2 | _ => LevelFilter::Trace,
        });
    }

    logger.init();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Compile(t) => return compile(t),
        SubCommand::Print(t) => return print(t),
    }
}

fn print(p: Print) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let filename = p.file;
    debug!("Compiling file: {}", filename);

    let content = fs::read_to_string(&filename).unwrap(); // FIXME
    let chars = content.chars();

    let tokens = TokenIterator::new_iterator(chars)
        .filter(|x| (*x != Token::Space) && (*x != Token::NewLine));

    let mut out = File::create(format!("{}.tks", &filename))?;

    for token in tokens {
        writeln!(out, "{:?}", token)?;
    }

    Ok(())
}

fn compile(c: Compile) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let compiler = Compiler::new();

    let file = c.file.as_ref();

    return compiler.compile(file);
}