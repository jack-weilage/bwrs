use crate::api::ApiClient;

use super::CliCommand;
use clap::{Args, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Input, Password};

#[derive(ValueEnum, Clone, Copy)]
enum TwoStepMethod {
    Authenticator = 0,
    Email = 1,
    YubiKey = 3,
}

#[derive(Args)]
pub struct LoginArgs {
    // method: TwoStepMethod,
    // code: String,
    // sso: bool,
    // apikey: String,
    // passwordenv: String,
    // check: bool,
}

pub struct Login;
impl CliCommand for Login {
    type Args = LoginArgs;

    fn handle(args: Self::Args) -> eyre::Result<()> {
        let client = ApiClient::new(
            "https://api.bitwarden.com".to_string(),
            "https://identity.bitwarden.com".to_string(),
        );
        let email: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Email address")
            .validate_with(|input: &String| {
                if input.contains('@') && input.contains('.') && !input.trim().is_empty() {
                    Ok(())
                } else {
                    Err("Email address is invalid.")
                }
            })
            .interact_text()
            .unwrap();

        let kdf_config = client.prelogin(email)?;

        let password: String = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Master password")
            .validate_with(|input: &String| {
                if !input.trim().is_empty() {
                    Ok(())
                } else {
                    Err("Master password is required.")
                }
            })
            .interact()
            .unwrap();

        todo!()
    }
}
