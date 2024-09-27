use std::io::{IsTerminal, Read};

use clap::Args;
use eyre::eyre;

use crate::crypto::Base64;

use super::CliCommand;

#[derive(Args)]
pub struct EncodeArgs {}

pub struct Encode;
impl CliCommand for Encode {
    type Args = EncodeArgs;
    fn handle(_: Self::Args) -> eyre::Result<()> {
        let mut stdin = std::io::stdin();
        if stdin.is_terminal() {
            return Err(eyre!("No stdin was piped in."));
        }

        let mut buf = String::new();
        stdin.read_to_string(&mut buf)?;

        println!("{}", Base64::encode(buf));

        Ok(())
    }
}
