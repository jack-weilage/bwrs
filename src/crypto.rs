use argon2::{
    Algorithm as Argon2Algorithm, Argon2, Params as Argon2Params, Version as Argon2Version,
};
use base64::Engine;
use eyre::Result;
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use sha2::{Digest, Sha256};

use crate::api::{KdfConfig, KdfKind};

struct SymmetricKey([u8; 64]);

// AKA makeMasterKey
fn deriveKeyFromPassword(
    password: String,
    salt: String,
    kdf_config: KdfConfig,
) -> Result<SymmetricKey> {
    let mut final_key = [0; 64];
    let enc_key = &mut final_key.as_mut_slice()[0..32];

    match kdf_config.kind {
        KdfKind::Pbkdf2 => pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            salt.as_bytes(),
            kdf_config.iterations,
            enc_key,
        ),
        KdfKind::Argon2id => {
            let hash = Sha256::digest(salt);
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
            .hash_password_into(password.as_bytes(), &hash, enc_key)
            .unwrap();
        }
    };

    Ok(SymmetricKey(final_key))
}

#[repr(u8)]
enum HashPurpose {
    ServerAuthorization = 1,
    LocalAuthorization = 2,
}
impl HashPurpose {
    fn iterations(self) -> u32 {
        self as u32
    }
}
fn hashMasterKey(password: &[u8], key: &[u8], purpose: HashPurpose) -> [u8; 32] {
    pbkdf2_hmac_array::<Sha256, 32>(password, key, purpose.iterations())
}

pub struct Base64;
impl Base64 {
    pub fn encode_url_safe<T: AsRef<[u8]>>(input: T) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
    }
    pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
        base64::engine::general_purpose::STANDARD.encode(input)
    }
}
