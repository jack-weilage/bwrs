use eyre::Result;

mod completion;
pub use completion::Completion;

pub trait CliCommand {
    type Args: clap::Args;
    fn handle(args: Self::Args) -> Result<()>;
}
