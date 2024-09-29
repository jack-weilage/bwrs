use dialoguer::{theme::ColorfulTheme, Input};
use eyre::{eyre, Result};
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::crypto::Base64;

#[derive(Serialize)]
struct PreloginRequest {
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct KdfConfig {
    #[serde(rename = "kdf")]
    pub kind: KdfKind,
    #[serde(rename = "kdfIterations")]
    pub iterations: u32,
    #[serde(rename = "kdfMemory")]
    pub memory: Option<u32>,
    #[serde(rename = "kdfParallelism")]
    pub parallelism: Option<u32>,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum KdfKind {
    Pbkdf2 = 0,
    Argon2id = 1,
}

#[derive(Serialize, Debug)]
pub struct ConnectRequest {
    scope: String,
    grant_type: GrantKind,
    client_id: ClientKind,
    #[serde(flatten)]
    device: Option<ConnectDevice>,
    #[serde(flatten)]
    auth: ConnectAuth,
    #[serde(flatten)]
    two_factor: Option<TwoFactorVerification>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantKind {
    RefreshToken,
    Password,
    ClientCredentials,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientKind {
    Web,
    Browser,
    Desktop,
    // Mobile,
    Cli,
    // [serde(rename = "connector")]
    // DirectoryConnector,
}

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum DeviceKind {
    Android = 0,
    iOS = 1,
    ChromeExtension = 2,
    FirefoxExtension = 3,
    OperaExtension = 4,
    EdgeExtension = 5,
    WindowsDesktop = 6,
    MacOsDesktop = 7,
    LinuxDesktop = 8,
    ChromeBrowser = 9,
    FirefoxBrowser = 10,
    OperaBrowser = 11,
    EdgeBrowser = 12,
    IEBrowser = 13,
    UnknownBrowser = 14,
    AndroidAmazon = 15,
    UWP = 16,
    SafariBrowser = 17,
    VivaldiBrowser = 18,
    VivaldiExtension = 19,
    SafariExtension = 20,
    SDK = 21,
    Server = 22,
    WindowsCLI = 23,
    MacOsCLI = 24,
    LinuxCLI = 25,
}

#[derive(Serialize, Debug)]
pub struct ConnectDevice {
    #[serde(rename = "deviceType")]
    kind: DeviceKind,
    #[serde(rename = "deviceIdentifier")]
    id: Uuid,
    #[serde(rename = "deviceName")]
    name: String,
    // Push tokens don't seem to be implemented
    // #[serde(rename = "devicePushToken")]
    // push_token: String
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ConnectAuth {
    Password { username: String, password: String },
}

#[derive(Deserialize, Debug)]
pub struct Session {
    access_token: String,
}

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum TwoFactorProvider {
    Authenticator,
    Email,
    Duo,
    Yubikey,
    U2f,
    Remember,
    OrganizationDuo,
    WebAuthn,
}

impl std::convert::TryFrom<u64> for TwoFactorProvider {
    type Error = eyre::Error;

    fn try_from(v: u64) -> std::result::Result<Self, Self::Error> {
        Ok(match v {
            0 => Self::Authenticator,
            1 => Self::Email,
            2 => Self::Duo,
            3 => Self::Yubikey,
            4 => Self::U2f,
            5 => Self::Remember,
            6 => Self::OrganizationDuo,
            7 => Self::WebAuthn,
            n => return Err(eyre!("Invalid two factor provider: {n}")),
        })
    }
}
impl std::str::FromStr for TwoFactorProvider {
    type Err = eyre::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Authenticator,
            "1" => Self::Email,
            "2" => Self::Duo,
            "3" => Self::Yubikey,
            "4" => Self::U2f,
            "5" => Self::Remember,
            "6" => Self::OrganizationDuo,
            "7" => Self::WebAuthn,
            n => return Err(eyre!("Invalid two factor provider: {n}")),
        })
    }
}
impl<'de> Deserialize<'de> for TwoFactorProvider {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        use serde::de;
        use std::fmt;

        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = TwoFactorProvider;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string containing a number within quotation marks")
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                std::convert::TryFrom::try_from(v).map_err(serde::de::Error::custom)
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                v.parse().map_err(de::Error::custom)
            }
        }

        d.deserialize_str(Visitor)
    }
}
impl TwoFactorProvider {
    #[must_use]
    pub fn instructions(&self) -> &str {
        match *self {
            Self::Authenticator => {
                "Enter the 6 digit verification code from your authenticator app"
            }
            Self::Email => "Enter the 6 digit verification code you recieved via email",
            // TODO: Implement more providers
            _ => todo!(),
        }
    }

    pub async fn prepare_provider(&self, client: ApiClient) -> Result<()> {
        match *self {
            Self::Authenticator => {}
            // Self::Email => client.send_two_factor_email(identity).await?,
            Self::Email => unimplemented!(),
            Self::Duo => unimplemented!(),
            Self::Yubikey => unimplemented!(),
            Self::U2f => unimplemented!(),
            Self::Remember => unimplemented!(),
            Self::OrganizationDuo => unimplemented!(),
            Self::WebAuthn => unimplemented!(),
        }

        Ok(())
    }

    pub fn prompt_for_token(&self) -> Result<String> {
        match *self {
            Self::Authenticator | Self::Email => Ok(Input::with_theme(&ColorfulTheme::default())
                .with_prompt(self.instructions())
                .validate_with(|code: &String| {
                    if code.len() == 6 {
                        Ok(())
                    } else {
                        Err("2FA code must be 6 digits")
                    }
                })
                .interact()
                .unwrap()),

            // TODO: Implement more providers
            _ => todo!(),
        }
    }

    #[must_use]
    pub const fn name(&self) -> &str {
        match *self {
            Self::Authenticator => "Authenticator app",
            Self::Email => "Email verification",
            Self::Duo => "Duo",
            Self::Yubikey => "Yubikey",
            Self::U2f => "U2f",
            Self::Remember => "Remember",
            Self::OrganizationDuo => "Organization Duo",
            Self::WebAuthn => "Web authentication",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TwoFactorRequiredResponse {
    #[serde(rename = "TwoFactorProviders")]
    providers: Vec<TwoFactorProvider>,
    // TODO: This response also includes some information about the providers, but the Option here
    // messes things up
    // providers: HashMap<TwoFactorProvider, Option<TwoFactorProviderInfo>>,
}

#[derive(Serialize, Debug)]
pub struct TwoFactorVerification {
    #[serde(rename = "twoFactorToken")]
    pub token: String,
    #[serde(rename = "twoFactorProvider")]
    pub provider: TwoFactorProvider,
}

#[derive(Debug)]
pub enum LoginResult {
    NeedsTwoFactor(Vec<TwoFactorProvider>),
    Success(Session),
}

pub struct ApiClient {
    base_url: String,
    identity_url: String,
    agent: Client,
}
impl ApiClient {
    pub fn new(base_url: String, identity_url: String) -> Self {
        Self {
            base_url,
            identity_url,
            agent: Client::builder()
                .user_agent(format!(
                    "{}/{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .unwrap(),
        }
    }
    pub fn prelogin(&self, email: &str) -> Result<KdfConfig> {
        let config: KdfConfig = self
            .agent
            .post(format!("{}{}", self.identity_url, "/accounts/prelogin"))
            .json(&PreloginRequest {
                email: email.to_string(),
            })
            .send()?
            .json()?;

        Ok(config)
    }
    pub fn login(
        &self,
        email: &str,
        password_hash: [u8; 32],
        two_factor: Option<TwoFactorVerification>,
    ) -> Result<LoginResult> {
        let res = self
            .agent
            .post(format!("{}{}", self.identity_url, "/connect/token"))
            .header("Auth-Email", &Base64::encode_url_safe(email))
            .form(&ConnectRequest {
                scope: "api offline_access".to_string(),
                grant_type: GrantKind::Password,
                client_id: ClientKind::Cli,
                device: Some(ConnectDevice {
                    #[cfg(target_os = "windows")]
                    kind: DeviceKind::WindowsCLI,
                    #[cfg(target_os = "macos")]
                    kind: DeviceKind::MacOsCLI,
                    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                    kind: DeviceKind::LinuxCLI,
                    id: Uuid::new_v4(),
                    name: env!("CARGO_PKG_NAME").to_string(),
                }),
                auth: ConnectAuth::Password {
                    username: email.to_string(),
                    password: Base64::encode(password_hash),
                },
                two_factor,
            })
            .send()?;

        match res.status() {
            StatusCode::OK => Ok(LoginResult::Success(res.json::<Session>()?)),
            StatusCode::BAD_REQUEST => {
                let text = res.text()?;

                if text.contains("Two factor required") {
                    let error = serde_json::from_str::<TwoFactorRequiredResponse>(&text)?;

                    Ok(LoginResult::NeedsTwoFactor(error.providers))
                } else if text.contains("Two-step token is invalid") {
                    Err(eyre!("Two-factor token is invalid"))
                } else if text.contains("Username or password is incorrect") {
                    Err(eyre!("Username or password is incorrect"))
                } else {
                    todo!("handle other errors")
                }
            }
            _ => todo!(),
        }
    }
}
