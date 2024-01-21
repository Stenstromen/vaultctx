use std::{ process::Command, path::Path, fs };

use crate::model::Config;
use crate::controller::create_vaultctx_file;

pub fn list_contexts() {
    /*     let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");

    if !Path::new(&config_path).exists() {
        println!("Initial dummy config created at ~/.vaultctx");
        create_vaultctx_file(Vec::new());
        return;
    }

    let contents = fs::read_to_string(&config_path).expect("Failed to read config");

    let configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    for config in configs {
        println!("{}", config.name);
    } */
    choose_context_interactive();
}

fn choose_context_interactive() {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");

    if !Path::new(&config_path).exists() {
        println!("Initial dummy config created at ~/.vaultctx");
        create_vaultctx_file(Vec::new());
        return;
    }

    let contents = fs::read_to_string(&config_path).expect("Failed to read config");

    let configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    let names: Vec<String> = configs
        .into_iter()
        .map(|config| config.name)
        .collect();
    let names_str = names.join("\n");

    let command = format!("echo '{}' | KUBECTX_FORCE_COLOR=1 fzf --ansi --no-preview", names_str);

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()
        .expect("Failed to execute command");

    let _ = child.wait().expect("Failed to wait on child");
}
