use super::crypto::{self, NONCE_LEN, SALT_LEN};
use crate::util;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::*;
use base64::{engine::general_purpose, Engine as _};
use anyhow::{bail, Result};

use super::Vault;

#[derive(Serialize, Deserialize)]
struct Envelope {
    salt: String,
    nonce: String,
    cipher: String,
}

pub fn vault_path(name: &str) -> PathBuf {
    dirs::config_dir().unwrap().join(format!("{name}.vault"))
}

pub fn init_vault(master: &str, name: &str, force: bool) -> Result<()> {
    let path = vault_path(name);
    if path.exists() && !force {
        bail!("Vault already exists (use --force/-f to overwrite)");
    }

    save_encrypted(master, &path, &Vault::default())?;
    info!("Vault created at {:?}", path);
    Ok(())
}

pub fn open_vault(master: &str, name: &str) -> Result<Vault> {
    let path = vault_path(name);
    if !path.exists() {
        bail!("No vault found with the name '{}'", name);
    }

    load_encrypted(master, &path)
}

pub fn save_vault(master: &str, name: &str, v: &Vault) -> Result<()> {
    let path = vault_path(name);
    save_encrypted(master, &path, v)
}

fn save_encrypted(master: &str, path: &PathBuf, v: &Vault) -> Result<()> {
    let salt = random_bytes(SALT_LEN);
    let key = crypto::derive_key(master, &salt);

    let nonce_vec = random_bytes(NONCE_LEN);
    let nonce: [u8; NONCE_LEN] = nonce_vec
        .as_slice()
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid nonce length"))?;

    let ct = crypto::encrypt(&key, &nonce, serde_json::to_vec(v)?)?;

    let env = Envelope {
        salt: general_purpose::STANDARD.encode(&salt),
        nonce: general_purpose::STANDARD.encode(&nonce_vec),
        cipher: general_purpose::STANDARD.encode(&ct),
    };
    fs::write(path, serde_json::to_string_pretty(&env)?)?;
    Ok(())
}

pub fn load_encrypted(master: &str, path: &PathBuf) -> Result<Vault> {
    let env: Envelope = serde_json::from_str(&fs::read_to_string(path)?)?;

    let salt = general_purpose::STANDARD.decode(env.salt)?;
    let nonce_vec = general_purpose::STANDARD.decode(env.nonce)?;
    let nonce: [u8; NONCE_LEN] = nonce_vec
        .as_slice()
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid nonce length"))?;

    let ct = general_purpose::STANDARD.decode(env.cipher)?;

    let key = crypto::derive_key(master, &salt);
    let pt = crypto::decrypt(&key, &nonce, ct)?;
    Ok(serde_json::from_slice(&pt)?)
}

fn random_bytes(n: usize) -> Vec<u8> {
    let mut v = vec![0u8; n];
    OsRng.fill_bytes(&mut v);
    v
}
