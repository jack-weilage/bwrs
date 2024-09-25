use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Login,
    Logout,
    Lock,
    Unlock,
    Sync,
    Generate,
    Encode,
    Config,
    Update,
    Completion,
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

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Login => todo!(),
        Command::Logout => todo!(),
        Command::Lock => todo!(),
        Command::Unlock => todo!(),
        Command::Sync => todo!(),
        Command::Generate => todo!(),
        Command::Encode => todo!(),
        Command::Config => todo!(),
        Command::Update => todo!(),
        Command::Completion => todo!(),
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
