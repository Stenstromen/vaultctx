mod cmd;
mod view;
mod model;
mod controller;

use controller::{ delete_entry, vault_context_details, switch_to_previous_context };
use view::list_contexts;
use cmd::Args;
use clap::Parser;
use std::env;

fn main() {
    let args = Args::parse();

    if args.currentcontext {
        match env::var("VAULT_CONTEXT") {
            Ok(val) if !val.is_empty() => {
                println!("{}", val);
            }
            _ => {
                println!("No Vault Context found");
            }
        }
    } else if args.switchcontext {
        switch_to_previous_context();
    } else if let Some(name) = args.delete {
        delete_entry(&name);
    } else if let Some(vault_context) = args.vault_context {
        vault_context_details(&vault_context);
    } else {
        list_contexts();
    }
}
