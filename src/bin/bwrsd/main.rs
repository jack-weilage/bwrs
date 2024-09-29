use std::{env, process::Command};

use clap::Parser;
use eyre::{eyre, Context, Result};

#[derive(Parser)]
struct Cli {
    /// Start bwrsd in the background.
    #[arg(short, long)]
    daemonize: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.daemonize {
        Command::new(if let Ok(exe) = env::current_exe() {
            exe.to_string_lossy().to_string()
        } else {
            env!("CARGO_BIN_NAME").to_string()
        })
        .args(env::args().skip(1).filter(|a| a != "--daemonize"))
        .spawn()
        .wrap_err(eyre!("Failed to fork process"))?;

        println!("Successfully daemonized!");

        return Ok(());
    }

    loop {}
}
