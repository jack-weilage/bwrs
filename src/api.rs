use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use ureq::{Agent, AgentBuilder};

#[derive(Serialize)]
struct PreloginRequest {
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct KdfConfig {
    #[serde(rename = "kdf")]
    kind: KdfKind,
    #[serde(rename = "kdfIterations")]
    iterations: u64,
    #[serde(rename = "kdfMemory")]
    memory: Option<u64>,
    #[serde(rename = "kdfParallelism")]
    parallelism: Option<u64>,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum KdfKind {
    Pbkdf2 = 0,
    Argon2id = 1,
}

pub struct ApiClient {
    base_url: String,
    identity_url: String,
    agent: Agent,
}
impl ApiClient {
    pub fn new(base_url: String, identity_url: String) -> Self {
        Self {
            base_url,
            identity_url,
            agent: AgentBuilder::new()
                .user_agent(&format!(
                    "{}/{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ))
                .build(),
        }
    }
    pub fn prelogin(&self, email: String) -> Result<KdfConfig> {
        let config: KdfConfig = self
            .agent
            .post(&format!("{}{}", self.identity_url, "/accounts/prelogin"))
            .send_json(PreloginRequest { email })?
            .into_json()?;

        Ok(config)
    }
}
