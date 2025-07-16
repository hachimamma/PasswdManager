use clap::Parser;

mod cli;
mod session;
mod util;
mod vault;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    if cli.login {
        session::login()?;

    } else if let Some(name) = cli.new_vault {
        vault::create_vault(&name, cli.force)?;

    } else if cli.open.is_some() {
        println!("Vault opened (noop, session used instead).");

    } else if cli.add_entry {
        if let (Some(service), Some(username), Some(password)) =
            (&cli.service, &cli.username, &cli.password)
        {
            vault::add_entry_direct(service, username, password)?;
        } else {
            vault::add_entry_interactive()?;
        }

    } else if let Some(service) = cli.patch_entry {
        vault::patch_entry(&service)?;

    } else if let Some(service) = cli.delete_entry {
        vault::delete_entry(&service)?;

    } else if let Some(vname) = cli.patch_vault {
        vault::patch_vault(&vname)?;

    } else if let Some(vname) = cli.delete_vault {
        vault::delete_vault(&vname)?;

    } else if cli.list.is_some() {
        // Ensure session is loaded first
        session::load_session()?; // Will error out if not logged in
        vault::list_entries()?;

    } else if cli.patch_master_key {
        session::patch_master_key()?;

    } else if let Some(service) = cli.view_entry.as_deref() {
        session::load_session()?; // Ensure session exists
        vault::view_entry(service)?;

    } else {
        println!("Run --help for usage");
    }

    Ok(())
}
