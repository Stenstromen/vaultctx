use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short,long)]
    context: Option<String>,

    #[arg(short, long)]
    delete: Option<String>,

    #[arg(required = false)]
    section_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    address: String,
    token: String,
    cacert: String,
    tls_server_name: String,
}

fn main() {
    let args = Args::parse();

    if let Some(name) = args.delete {
        delete_entry(&name);
    } else if let Some(context) = args.context {
        match context.as_str() {
            "user" => {
                println!("User context selected");
            },
            _ => {
                println!("Run list command here");
            }
        }
    } else if let Some(section_name) = args.section_name {
        print_section_details(&section_name);
    } else {
        list_contexts();
    }
}

fn list_contexts() {
    let contents = fs::read_to_string("config.yaml")
    .expect("Failed to read config.yaml");

let configs: Vec<Config> = serde_yaml::from_str(&contents)
    .expect("Failed to parse YAML");

for config in configs {
    println!("{}", config.name);
}
}

fn delete_entry(entry_name: &str) {
    let contents = fs::read_to_string("config.yaml")
        .expect("Failed to read config.yaml");

    let mut configs: Vec<Config> = serde_yaml::from_str(&contents)
        .expect("Failed to parse YAML");

    configs.retain(|config| config.name != entry_name);

    let new_contents = serde_yaml::to_string(&configs)
        .expect("Failed to serialize data");

    fs::write("config.yaml", new_contents)
        .expect("Failed to write to config.yaml");

    println!("Entry '{}' deleted", entry_name);
}

fn print_section_details(section_name: &str) {
    let contents = fs::read_to_string("config.yaml")
        .expect("Failed to read config.yaml");

    let configs: Vec<Config> = serde_yaml::from_str(&contents)
        .expect("Failed to parse YAML");

    for config in configs {
        if config.name == section_name {
            println!("Name: {}", config.name);
            println!("Address: {}", config.address);
            println!("Token: {}", config.token);
            println!("CACert: {}", config.cacert);
            println!("TLSServerName: {}", config.tls_server_name);
            break;
        }
    }
}