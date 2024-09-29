use clap::{Args, CommandFactory};
use clap_complete::Shell;
use eyre::OptionExt;

use crate::Cli;

use super::CliCommand;

#[derive(Args)]
pub struct CompletionArgs {
    /// Shell to generate completions for.
    shell: Option<Shell>,
}

pub struct Completion;
impl CliCommand for Completion {
    type Args = CompletionArgs;
    fn handle(args: Self::Args) -> eyre::Result<()> {
        clap_complete::generate(
            args.shell
                .or(Shell::from_env())
                .ok_or_eyre("Unknown shell detected")?,
            &mut Cli::command(),
            env!("CARGO_BIN_NAME"),
            &mut std::io::stdout(),
        );

        Ok(())
    }
}
