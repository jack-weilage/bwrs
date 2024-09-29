use eyre::Result;

mod login;
pub use login::Login;

mod encode;
pub use encode::Encode;

mod completion;
pub use completion::Completion;

pub trait CliCommand {
    type Args: clap::Args;
    fn handle(args: Self::Args) -> Result<()>;
}
