//! Ikedi binary.

use anyhow::Result;
use ikedi::command::Cli;
use ikedi::parser;
use ikedi::processor;
use tokio::sync::mpsc;
use tokio::{io, task};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::new();
    let Cli { path, ratelimit } = args;

    let (tx, rx) = mpsc::channel(ratelimit);

    for file in parser::read_directory(path) {
        task::spawn(processor::process_file(file.into_path(), tx.clone()));
    }
    drop(tx);

    let mut stdout = io::stdout();
    processor::write(&mut stdout, rx).await?;
    Ok(())
}
