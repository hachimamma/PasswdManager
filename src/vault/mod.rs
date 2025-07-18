pub mod crypto;
pub mod store;

use crate::session;
use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{bail, Result};
use tracing::*;

pub type Vault = HashMap<String, Credential>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

/// `passwd --new-vault <name>`
pub fn create_vault(name: &str, force: bool) -> Result<()> {
    println!("Set master password:");
    let pw1 = util::prompt("> ");
    println!("Confirm:");
    let pw2 = util::prompt("> ");
    if pw1 != pw2 {
        bail!("mismatch");
    }

    store::init_vault(&pw1, name, force)?;
    println!("[+] Vault '{name}' created successfully.");
    Ok(())
}

/// `passwd --view-entry <service>` — View username and password for a service
pub fn view_entry(service: &str) -> Result<()> {
    let sess = session::load_session()?;
    let vault = store::open_vault(&sess.master_key, &sess.vault_name)?;

    let key = service.to_lowercase();
    if let Some(entry) = vault.get(&key) {
        println!("Service: {}", service);
        println!("Username: {}", entry.username);
        println!("Password: {}", entry.password);
    } else {
        bail!("No entry named '{}'", service);
    }
    Ok(())
}

/// interactive add: `passwd --add-entry`
pub fn add_entry_interactive() -> Result<()> {
    let sess = session::load_session()?;
    let mut vault = store::open_vault(&sess.master_key, &sess.vault_name)?;

    println!("[+] Service:");
    let service = util::prompt("> ");
    println!("[+] Username:");
    let user = util::prompt("> ");
    println!("[+] Password (leave blank to generate):");
    let mut pass = util::prompt("> ");
    if pass.is_empty() {
        pass = util::random_pw();
    }

    vault.insert(
        service.to_lowercase(),
        Credential { username: user, password: pass },
    );
    store::save_vault(&sess.master_key, &sess.vault_name, &vault)?;
    info!("saved {service}");
    Ok(())
}

/// direct add: `passwd --add-entry <service> <username> <password>`
pub fn add_entry_direct(service: &str, username: &str, password: &str) -> Result<()> {
    let sess = session::load_session()?;
    let mut vault = store::open_vault(&sess.master_key, &sess.vault_name)?;

    vault.insert(
        service.to_lowercase(),
        Credential {
            username: username.to_string(),
            password: password.to_string(),
        },
    );

    store::save_vault(&sess.master_key, &sess.vault_name, &vault)?;
    info!("saved {service}");
    Ok(())
}

/// `passwd --patch <service>`
pub fn patch_entry(service: &str) -> Result<()> {
    let sess = session::load_session()?;
    let mut vault = store::open_vault(&sess.master_key, &sess.vault_name)?;

    let key = service.to_lowercase();
    let entry = vault
        .get_mut(&key)
        .ok_or_else(|| anyhow::anyhow!("no entry named '{}'", service))?;

    println!("[+] New username (enter to keep '{}'):", entry.username);
    let new_user = util::prompt("> ");
    if !new_user.is_empty() {
        entry.username = new_user;
    }

    println!("[+] Confirm old password:");
    if util::prompt("> ") != entry.password {
        bail!("wrong old password");
    }

    println!("[+] New password:");
    let new_pass = util::prompt("> ");
    println!("[+] Confirm new password:");
    if new_pass != util::prompt("> ") {
        bail!("mismatch");
    }
    entry.password = new_pass;

    store::save_vault(&sess.master_key, &sess.vault_name, &vault)?;
    Ok(())
}

/// `passwd --delete <service>`
pub fn delete_entry(service: &str) -> Result<()> {
    let sess = session::load_session()?;
    let mut vault = store::open_vault(&sess.master_key, &sess.vault_name)?;

    println!("[+] Confirm master key:");
    if util::prompt("> ") != sess.master_key {
        bail!("wrong master key");
    }
    vault.remove(&service.to_lowercase());
    store::save_vault(&sess.master_key, &sess.vault_name, &vault)?;
    Ok(())
}

pub fn patch_vault(_name: &str) -> Result<()> {
    bail!("patch‑vault not implemented yet")
}

pub fn delete_vault(_name: &str) -> Result<()> {
    bail!("delete‑vault not implemented yet")
}

/// `passwd --list`
pub fn list_entries() -> Result<()> {
    let sess = session::load_session()?;
    let vault = store::open_vault(&sess.master_key, &sess.vault_name)?;

    for (srv, cred) in vault {
        println!("{srv}  ->  {}", cred.username);
    }
    Ok(())
}
