use crate::api::{
    ApiClient, KdfConfig, KdfKind, LoginResult, TwoFactorProvider, TwoFactorVerification,
};

use super::CliCommand;
use argon2::{
    Algorithm as Argon2Algorithm, Argon2, Params as Argon2Params, Version as Argon2Version,
};
use clap::{Args, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use hkdf::Hkdf;
use pbkdf2::pbkdf2_hmac_array;
use sha2::{Digest, Sha256};

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

        let kdf_config = client.prelogin(&email)?;

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

        let mut enc_key = match kdf_config.kind {
            KdfKind::Pbkdf2 => pbkdf2_hmac_array::<Sha256, 32>(
                password.as_bytes(),
                email.as_bytes(),
                kdf_config.iterations,
            ),
            KdfKind::Argon2id => {
                let salt = Sha256::digest(email.as_bytes());
                let mut buf = [0; 32];

                Argon2::new(
                    Argon2Algorithm::Argon2id,
                    Argon2Version::V0x13,
                    Argon2Params::new(
                        kdf_config.memory.unwrap() * 1024,
                        kdf_config.iterations,
                        kdf_config.parallelism.unwrap(),
                        Some(32),
                    )
                    .unwrap(),
                )
                .hash_password_into(password.as_bytes(), &salt, &mut buf)
                .unwrap();

                buf
            }
        };

        let server_master_key_hash =
            pbkdf2_hmac_array::<Sha256, 32>(&enc_key, password.as_bytes(), 1);
        let local_master_key_hash =
            pbkdf2_hmac_array::<Sha256, 32>(&enc_key, password.as_bytes(), 2);

        let hkdf = Hkdf::<Sha256>::from_prk(&enc_key).unwrap();
        hkdf.expand(b"enc", &mut enc_key).unwrap();

        let mut mac_key = [0; 32];
        hkdf.expand(b"mac", &mut mac_key).unwrap();

        let session = match client.login(&email, server_master_key_hash, None)? {
            LoginResult::Success(session) => session,
            LoginResult::NeedsTwoFactor(providers) => {
                let provider = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Two-step login method")
                    .default(0)
                    .items(
                        &providers
                            .iter()
                            .map(|p| p.name().to_string())
                            .collect::<Vec<_>>(),
                    )
                    .interact()
                    .unwrap() as u64;

                let provider: TwoFactorProvider = provider.try_into().unwrap();
                let token = provider.prompt_for_token()?;

                let res = client.login(
                    &email,
                    server_master_key_hash,
                    Some(TwoFactorVerification { token, provider }),
                )?;

                println!("{res:?}");

                todo!()
            }
        };
        println!("{session:?}");

        Ok(())
    }
}
