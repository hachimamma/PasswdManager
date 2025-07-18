use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("main")
        .required(true)
        .args(&[
            "login",
            "new_vault",
            "open",
            "add_entry",
            "patch_entry",
            "delete_entry",
            "patch_vault",
            "delete_vault",
            "list",
            "patch_master_key",
            "view_entry"
        ])
))]
pub struct Cli {
    /// Log in to unlock the vault
    #[clap(long, short = 'l')]
    pub login: bool,

    /// Create a new vault with name
    #[clap(long = "new-vault", short = 'n')]
    pub new_vault: Option<String>,

    /// Open an existing vault
    #[clap(long, short = 'o')]
    pub open: Option<String>,

    /// Add a new entry
    #[clap(long = "add-entry", short = 'a')]
    pub add_entry: bool,

    /// Patch an existing entry
    #[clap(long = "patch-entry", value_name = "SERVICE")]
    pub patch_entry: Option<String>,

    /// Delete an entry
    #[clap(long = "delete-entry", value_name = "SERVICE")]
    pub delete_entry: Option<String>,

    /// Patch vault name
    #[clap(long = "patch-vault", value_name = "VAULT")]
    pub patch_vault: Option<String>,

    /// Delete a vault
    #[clap(long = "delete-vault", value_name = "VAULT")]
    pub delete_vault: Option<String>,

    /// List all services in a vault
    #[clap(long = "list", short = 's', value_name = "VAULT")]
    pub list: Option<String>,

    /// View an existing entry
    #[clap(long = "view-entry", value_name = "SERVICE")]
    pub view_entry: Option<String>,

    /// Change the master password
    #[clap(long = "patch-master-key")]
    pub patch_master_key: bool,

    /// Force overwrite (use with --new-vault)
    #[clap(long, short = 'f')]
    pub force: bool,

    /// Service name (used with --add-entry)
    #[clap(value_name = "SERVICE", required = false)]
    pub service: Option<String>,

    /// Username (used with --add-entry)
    #[clap(value_name = "USERNAME", required = false)]
    pub username: Option<String>,

    /// Password (used with --add-entry)
    #[clap(value_name = "PASSWORD", required = false)]
    pub password: Option<String>,
}
