use std::str::FromStr;

use clap::{ arg, command, Parser };
use serde::{ Serialize, Deserialize };
use crate::model::RenameArgs;

const ABOUT: &str = "Context switching for Hashicorp Vault";
const LONG_ABOUT: &str =
    "Context switching for Hashicorp Vault with support for multiple vaults and namespaces";
const VERSION: &str = "1.0";
const AUTHOR: &str = "Stenstromen";

impl FromStr for RenameArgs {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ',').collect();
        if parts.len() == 2 {
            Ok(RenameArgs(parts[0].to_string(), parts[1].to_string()))
        } else {
            Err("Failed to parse rename arguments")
        }
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author = AUTHOR, version = VERSION, about = ABOUT, long_about = LONG_ABOUT)]
pub struct Args {
    #[arg(short, long, help = "Switch to previous context. Usage: -s")]
    pub switchcontext: bool,

    #[arg(short, long, help = "Show current context. Usage: -c")]
    pub currentcontext: bool,

    #[arg(short, long, help = "Delete context. Usage: -d 'context_name'")]
    pub delete: Option<String>,

    #[arg(short, long, help = "Rename context. Usage: -r 'old_name,new_name'")]
    pub rename: Option<RenameArgs>,

    #[arg(required = false, help = "Vault context")]
    pub vault_context: Option<String>,
}
