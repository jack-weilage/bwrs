use clap::{Parser, Subcommand};
use eyre::Result;

mod commands;
use commands::{CliCommand, Completion, Login};
mod api;
mod crypto;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Log into a user account.
    Login(<Login as CliCommand>::Args),
    Logout,
    Lock,
    Unlock,
    Sync,
    Generate,
    Encode,
    Config,
    Update,
    /// Generate shell completions.
    Completion(<Completion as CliCommand>::Args),
    Status,
    List,
    Get,
    Create,
    Edit,
    Delete,
    Restore,
    Move,
    Confirm,
    Import,
    Export,
    Share,
    Send,
    Receive,
    DeviceApproval,
    Serve,
}
impl Command {
    pub fn handle(self) -> Result<()> {
        match self {
            Command::Login(args) => Login::handle(args),
            Command::Logout => todo!(),
            Command::Lock => todo!(),
            Command::Unlock => todo!(),
            Command::Sync => todo!(),
            Command::Generate => todo!(),
            Command::Encode => todo!(),
            Command::Config => todo!(),
            Command::Update => todo!(),
            Command::Completion(args) => Completion::handle(args),
            Command::Status => todo!(),
            Command::List => todo!(),
            Command::Get => todo!(),
            Command::Create => todo!(),
            Command::Edit => todo!(),
            Command::Delete => todo!(),
            Command::Restore => todo!(),
            Command::Move => todo!(),
            Command::Confirm => todo!(),
            Command::Import => todo!(),
            Command::Export => todo!(),
            Command::Share => todo!(),
            Command::Send => todo!(),
            Command::Receive => todo!(),
            Command::DeviceApproval => todo!(),
            Command::Serve => todo!(),
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    args.command.handle()
}
