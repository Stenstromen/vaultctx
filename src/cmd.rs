use clap::{command, Parser};
use serde::{Serialize, Deserialize};

const ABOUT: &str = "Context switching for Hashicorp Vault";
const LONG_ABOUT: &str = "Context switching for Hashicorp Vault with support for multiple vaults and namespaces";
const VERSION: &str = "1.0";
const AUTHOR: &str = "Stenstromen";

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author = AUTHOR, version = VERSION, about = ABOUT, long_about = LONG_ABOUT)]
pub struct Args {
    #[arg(short, long)]
    pub switchcontext: bool,

    #[arg(short, long)]
    pub currentcontext: bool,

    #[arg(short, long)]
    pub delete: Option<String>,

    #[arg(required = false)]
    pub vault_context: Option<String>,
}