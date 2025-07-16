use std::{fs, path::PathBuf};

use crate::util;
use crate::vault::store;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub vault_name: String,
    pub master_key: String,
}

/// Get the path to the session file
pub fn session_file() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".passwd_session")
}

/// Save the current session
fn save_session(session: &Session) -> Result<()> {
    fs::write(session_file(), serde_json::to_string(session)?)?;
    Ok(())
}

/// Load current session (errors if not logged in)
pub fn load_session() -> Result<Session> {
    let path = session_file();
    let content = fs::read_to_string(&path)
        .with_context(|| "No active session. Run `--login` first.")?;

    serde_json::from_str(&content)
        .with_context(|| "Failed to deserialize session file")
}

/// Perform login and store session if successful
pub fn login() -> Result<()> {
    println!("[*] Vault name:");
    let vault_name = util::prompt("> ");

    println!("[*] Master password:");
    let master_key = util::prompt("> ");

    let vault_path = store::vault_path(&vault_name);
    if !vault_path.exists() {
        bail!("Vault '{}' does not exist.", vault_name);
    }

    // Validate credentials by trying to decrypt the vault
    store::load_encrypted(&master_key, &vault_path)
        .with_context(|| "Invalid master password or corrupt vault.")?;

    let session = Session {
        vault_name,
        master_key,
    };

    save_session(&session)?;
    println!("[+] Logged in successfully.");
    Ok(())
}

/// Patch master key
pub fn patch_master_key() -> Result<()> {
    let mut session = load_session()?;

    println!("[*] Enter old master key:");
    let old = util::prompt("> ");
    if old != session.master_key {
        bail!("Old master key incorrect");
    }

    println!("[*] Enter new master key:");
    let new = util::prompt("> ");
    println!("[*] Confirm new master key:");
    let confirm = util::prompt("> ");
    if new != confirm {
        bail!("New master key mismatch");
    }

    // Load and save vault with new key
    let path = store::vault_path(&session.vault_name);
    let vault = store::load_encrypted(&old, &path)?;
    store::save_vault(&new, &session.vault_name, &vault)?;

    session.master_key = new;
    save_session(&session)?;
    println!("[+] Master key updated.");
    Ok(())
}
