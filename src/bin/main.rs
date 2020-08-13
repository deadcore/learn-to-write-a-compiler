use clap::Clap;

use learn_to_write_a_compiler::compiler::Compiler;
use log::LevelFilter;

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
        SubCommand::Compile(t) => return compile(t)
    }
}

fn compile(c: Compile) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let compiler = Compiler::from_arg_matches();

    let file = c.file.as_ref();

    return compiler.compile(file);
}