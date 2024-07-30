//! Ikedi binary.

use ikedi::command::Cli;
use ikedi::error::Result;
use ikedi::parser;

fn main() -> Result<()> {
    let args = Cli::new();
    let Cli { path } = &args;
    for file in parser::read_directory(path) {
        println!("{}", file.path().display())
    }
    Ok(())
}
