#[macro_use]
extern crate clap;

use learn_to_write_a_compiler::compiler::Result;
use learn_to_write_a_compiler::compiler::Compiler;

fn main() -> Result<()> {
    env_logger::init();

    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Jack L")
        (about: "Does awesome things")
        (@arg INPUT: +required "Sets the main file to compile")
    ).get_matches();

    let compiler = Compiler::from_arg_matches(&matches);

    let file = matches.value_of("INPUT").unwrap();

    return compiler.compile(file);
}