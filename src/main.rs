mod model;
use model::{Args, Config, Format};
use dirs;
use clap::Parser;
use serde_yaml;
use std::fs::File;
use std::path::{ Path, PathBuf };
use std::{ fs::{ self, OpenOptions }, io::Write, os::unix::prelude::PermissionsExt, env };

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
    } else if let Some(section_name) = args.section_name {
        print_section_details(&section_name);
        //prepare_for_context_switch(&section_name, &dirs::home_dir().expect("Failed to find home directory"));
    } else {
        list_contexts();
    }
}

fn apply_and_save_context(config_data: &str, section_name: &str, home_dir: &PathBuf) {
    // Update ~/.vctx with the new context data
    let vctx_path = home_dir.join(".vctx");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&vctx_path)
        .expect("Failed to open or create .vctx file");

    file.write_all(config_data.as_bytes()).expect("Failed to write to .vctx file");
    fs::set_permissions(&vctx_path, fs::Permissions::from_mode(0o600)).expect(
        "Failed to set file permissions"
    );

    append_to_shell_rc(home_dir.to_str().unwrap(), "[ -f ~/.vctx ] && source ~/.vctx");

    println!("\x1b[32mVault Context Switched to {}\x1b[0m", section_name);
    println!("\x1b[32mPlease reapply shell file or run 'source ~/.vctx' to apply\x1b[0m");
}

const PREVIOUS_CONTEXT_FILE: &str = ".previous_vault_context";

fn get_previous_context_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    home_dir.join(PREVIOUS_CONTEXT_FILE)
}

fn save_current_context() {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let vctx_path = home_dir.join(".vctx");

    if let Ok(contents) = fs::read_to_string(&vctx_path) {
        if
            let Some(context_line) = contents
                .lines()
                .find(|line| line.starts_with("export VAULT_CONTEXT='"))
        {
            if let Some(start) = context_line.find("'") {
                let end = context_line.rfind("'").unwrap_or(context_line.len());
                let current_context = &context_line[start + 1..end];

                let previous_context_file_path = get_previous_context_file_path();
                fs::write(previous_context_file_path, current_context).expect(
                    "Failed to write current context to file"
                );
            }
        }
    }
}

fn get_previous_context() -> Option<String> {
    let previous_context_file_path = get_previous_context_file_path();
    fs::read_to_string(previous_context_file_path).ok()
}

fn switch_to_previous_context() {
    save_current_context();
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    if let Some(previous_context) = get_previous_context() {
        let config_data = String::new(); // Populate config_data for previous_context
        apply_and_save_context(&config_data, &previous_context, &home_dir);
        println!("Switched back to previous context {}", previous_context);
    } else {
        println!("No previous context found");
    }
}

fn create_vaultctx_file(_configs: Vec<Config>) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");

    if !Path::new(&config_path).exists() {
        File::create(&config_path).expect("Failed to create file");
    }

    let dummy_vault = Config {
        name: "dummy_vault".to_string(),
        addr: "127.0.0.1".to_string(),
        token: "sometoken".to_string(),
        cacert: Some("asd".to_string()),
        tls_server_name: Some("asd".to_string()),
        capath: Some("asd".to_string()),
        client_cert: Some("asd".to_string()),
        client_key: Some("asd".to_string()),
        client_timeout: Some("asd".to_string()),
        cluster_addr: Some("asd".to_string()),
        format: Some(Format::Table),
        license: Some("asd".to_string()),
        license_path: Some("asd".to_string()),
        log_level: Some("asd".to_string()),
        max_retries: Some("asd".to_string()),
        redirect_addr: Some("asd".to_string()),
        skip_verify: Some("asd".to_string()),
        cli_no_color: Some("asd".to_string()),
        rate_limit: Some("asd".to_string()),
        namespace: Some("asd".to_string()),
        srv_lookup: Some("asd".to_string()),
        mfa: Some("asd".to_string()),
        http_proxy: Some("asd".to_string()),
        proxy_addr: Some("asd".to_string()),
        disable_redirects: Some("asd".to_string()),
    };

    let configs = vec![dummy_vault];

    let yaml = serde_yaml::to_string(&configs).expect("Failed to serialize to YAML");

    let mut file = File::create(&config_path).expect("Failed to create file");
    file.write_all(yaml.as_bytes()).expect("Failed to write to file");
}

fn list_contexts() {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
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
    }
}

fn delete_entry(entry_name: &str) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");
    let contents = fs::read_to_string(config_path).expect("Failed to read .vaultctx");

    let mut configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    configs.retain(|config| config.name != entry_name);

    let new_contents = serde_yaml::to_string(&configs).expect("Failed to serialize data");

    fs::write(contents, new_contents).expect("Failed to write to .vaultctx");

    println!("Entry '{}' deleted", entry_name);
}

fn print_section_details(section_name: &str) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");
    let contents = fs::read_to_string(config_path).expect("Failed to read .vaultctx");
    let configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    let mut found = false;
    let mut config_data = String::new();

    for config in configs {
        if config.name == section_name {
            config_data.push_str(&format!("export VAULT_CONTEXT='{}'\n", config.name));
            config_data.push_str(&format!("export VAULT_ADDRESS='{}'\n", config.addr));
            config_data.push_str(&format!("export VAULT_TOKEN='{}'\n", config.token));

            // Optional fields
            if let Some(cacert) = &config.cacert {
                config_data.push_str(&format!("export VAULT_CACERT='{}'\n", cacert));
            }
            // ... repeat for other fields ...
            if let Some(tls_server_name) = &config.tls_server_name {
                config_data.push_str(
                    &format!("export VAULT_TLS_SERVER_NAME='{}'\n", tls_server_name)
                );
            }
            if let Some(capath) = &config.capath {
                config_data.push_str(&format!("export VAULT_CAPATH='{}'\n", capath));
            }
            if let Some(client_cert) = &config.client_cert {
                config_data.push_str(&format!("export VAULT_CLIENT_CERT='{}'\n", client_cert));
            }
            if let Some(client_key) = &config.client_key {
                config_data.push_str(&format!("export VAULT_CLIENT_KEY='{}'\n", client_key));
            }
            if let Some(client_timeout) = &config.client_timeout {
                config_data.push_str(
                    &format!("export VAULT_CLIENT_TIMEOUT='{}'\n", client_timeout)
                );
            }
            if let Some(cluster_addr) = &config.cluster_addr {
                config_data.push_str(&format!("export VAULT_CLUSTER_ADDR='{}'\n", cluster_addr));
            }
            if let Some(format) = &config.format {
                config_data.push_str(&format!("export VAULT_FORMAT='{}'\n", format));
            }
            if let Some(license) = &config.license {
                config_data.push_str(&format!("export VAULT_LICENSE='{}'\n", license));
            }
            if let Some(license_path) = &config.license_path {
                config_data.push_str(&format!("export VAULT_LICENSE_PATH='{}'\n", license_path));
            }
            if let Some(log_level) = &config.log_level {
                config_data.push_str(&format!("export VAULT_LOG_LEVEL='{}'\n", log_level));
            }
            if let Some(max_retries) = &config.max_retries {
                config_data.push_str(&format!("export VAULT_MAX_RETRIES='{}'\n", max_retries));
            }
            if let Some(redirect_addr) = &config.redirect_addr {
                config_data.push_str(&format!("export VAULT_REDIRECT_ADDR='{}'\n", redirect_addr));
            }
            if let Some(skip_verify) = &config.skip_verify {
                config_data.push_str(&format!("export VAULT_SKIP_VERIFY='{}'\n", skip_verify));
            }
            if let Some(cli_no_color) = &config.cli_no_color {
                config_data.push_str(&format!("export VAULT_CLI_NO_COLOR='{}'\n", cli_no_color));
            }
            if let Some(rate_limit) = &config.rate_limit {
                config_data.push_str(&format!("export VAULT_RATE_LIMIT='{}'\n", rate_limit));
            }
            if let Some(namespace) = &config.namespace {
                config_data.push_str(&format!("export VAULT_NAMESPACE='{}'\n", namespace));
            }
            if let Some(srv_lookup) = &config.srv_lookup {
                config_data.push_str(&format!("export VAULT_SRV_LOOKUP='{}'\n", srv_lookup));
            }
            if let Some(mfa) = &config.mfa {
                config_data.push_str(&format!("export VAULT_MFA='{}'\n", mfa));
            }
            if let Some(http_proxy) = &config.http_proxy {
                config_data.push_str(&format!("export VAULT_HTTP_PROXY='{}'\n", http_proxy));
            }
            if let Some(proxy_addr) = &config.proxy_addr {
                config_data.push_str(&format!("export VAULT_PROXY_ADDR='{}'\n", proxy_addr));
            }
            if let Some(disable_redirects) = &config.disable_redirects {
                config_data.push_str(
                    &format!("export VAULT_DISABLE_REDIRECTS='{}'\n", disable_redirects)
                );
            }

            found = true;
            break;
        }
    }
    if found {
        save_current_context();
        apply_and_save_context(&config_data, section_name, &home_dir);
    } else {
        println!("Section '{}' not found", section_name);
    }

    if !found {
        create_vaultctx_file(Vec::new());
    }
}

fn append_to_shell_rc(home: &str, line: &str) {
    let bash_rc = format!("{}/.bashrc", home);
    let zsh_rc = format!("{}/.zshrc", home);

    // This function will append the line to .bashrc or .zshrc if it doesn't already contain it
    let append_if_missing = |file_path: &str| {
        if let Ok(content) = fs::read_to_string(file_path) {
            if !content.contains(line) {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(file_path)
                    .expect("Failed to open file");
                writeln!(file, "{}", line).expect("Failed to write to file");
            }
        }
    };

    append_if_missing(&bash_rc);
    append_if_missing(&zsh_rc);
}
