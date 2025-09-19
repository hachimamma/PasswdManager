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
    #[clap(long, short = 'l')]
    pub login: bool,

    #[clap(long = "new-vault", short = 'n')]
    pub new_vault: Option<String>,
    
    #[clap(long, short = 'o')]
    pub open: Option<String>,

    #[clap(long = "add-entry", short = 'a')]
    pub add_entry: bool,

    #[clap(long = "patch-entry", value_name = "SERVICE")]
    pub patch_entry: Option<String>,

    #[clap(long = "delete-entry", value_name = "SERVICE")]
    pub delete_entry: Option<String>,

    #[clap(long = "patch-vault", value_name = "VAULT")]
    pub patch_vault: Option<String>,

    #[clap(long = "delete-vault", value_name = "VAULT")]
    pub delete_vault: Option<String>,

    #[clap(long = "list", short = 's', value_name = "VAULT")]
    pub list: Option<String>,

    #[clap(long = "view-entry", value_name = "SERVICE")]
    pub view_entry: Option<String>,

    #[clap(long = "patch-master-key")]
    pub patch_master_key: bool,

    #[clap(long, short = 'f')]
    pub force: bool,

    #[clap(value_name = "SERVICE", required = false)]
    pub service: Option<String>,

    #[clap(value_name = "USERNAME", required = false)]
    pub username: Option<String>,

    #[clap(value_name = "PASSWORD", required = false)]
    pub password: Option<String>,
}

