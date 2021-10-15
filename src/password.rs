use crate::config::Config;
use sha3::{Digest, Sha3_512};

pub fn encrypted_password(config: &Config) -> Vec<u8> {
    let mut hasher: Sha3_512 = Sha3_512::new();
    hasher.update(config.password());
    let digest: Vec<u8> = hasher.finalize().to_vec();

    digest
}
