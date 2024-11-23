use anyhow::anyhow;
use sha2::{Digest, Sha256};

use crate::{Error, Result};

pub fn sha256_with_key(data: &str, key: &str) -> Result<String> {
    let data = format!("{}{}", data, key);
    let mut hasher = Sha256::new();
    hasher.update(data);

    let hash = hasher.finalize();
    let mut buf = [0u8; 64];
    let hash = base16ct::lower::encode_str(hash.as_slice(), &mut buf)
        .map_err(|e| Error::from(anyhow!("{}", e)))?;
    Ok(hash.to_string())
}

pub fn sha256(data: &str) -> Result<String> {
    sha256_with_key(data, "")
}
