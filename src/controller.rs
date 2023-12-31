use std::{
    path::{ Path, PathBuf },
    fs::{ File, self, OpenOptions },
    io::Write,
    os::unix::fs::PermissionsExt,
};

use crate::model::{
    Config,
    Format,
    LogLevel,
    MaxRetries,
    SkipVerify,
    DisableRedirects,
    CliNoColor,
};

const PREVIOUS_CONTEXT_FILE: &str = ".previous_vault_context";

fn get_previous_context_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    home_dir.join(PREVIOUS_CONTEXT_FILE)
}

fn get_previous_context() -> Option<String> {
    let previous_context_file_path = get_previous_context_file_path();
    fs::read_to_string(previous_context_file_path).ok()
}

pub fn create_vaultctx_file(_configs: Vec<Config>) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");

    if !Path::new(&config_path).exists() {
        File::create(&config_path).expect("Failed to create file");
    }

    let dummy_vault = Config {
        name: "dummy_vault".to_string(),
        addr: "127.0.0.1".to_string(),
        token: "hvs.abcdefghijklmnopqrstxyz".to_string(),
        cacert: Some("ca.crt".to_string()),
        tls_server_name: Some("vault.local".to_string()),
        capath: Some("/etc/ssl/certs".to_string()),
        client_cert: Some("cert.crt".to_string()),
        client_key: Some("cert.key".to_string()),
        client_timeout: Some("30".to_string()),
        cluster_addr: Some("1.1.1.1".to_string()),
        format: Some(Format::Table),
        license: Some("license.lic".to_string()),
        license_path: Some("/etc/vault/license".to_string()),
        log_level: Some(LogLevel::Info),
        max_retries: Some(MaxRetries::Two),
        redirect_addr: Some("1.0.0.1".to_string()),
        skip_verify: Some(SkipVerify::False),
        cli_no_color: Some(CliNoColor::False),
        rate_limit: Some("null".to_string()),
        namespace: Some("null".to_string()),
        srv_lookup: Some("null".to_string()),
        mfa: Some("null".to_string()),
        http_proxy: Some("null".to_string()),
        proxy_addr: Some("null".to_string()),
        disable_redirects: Some(DisableRedirects::False),
    };

    let configs = vec![dummy_vault];

    let yaml = serde_yaml::to_string(&configs).expect("Failed to serialize to YAML");

    let mut file = File::create(&config_path).expect("Failed to create file");
    file.write_all(yaml.as_bytes()).expect("Failed to write to file");
}

pub fn delete_entry(entry_name: &str) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");
    let contents = fs::read_to_string(config_path).expect("Failed to read .vaultctx");

    let mut configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    configs.retain(|config| config.name != entry_name);

    let new_contents = serde_yaml::to_string(&configs).expect("Failed to serialize data");

    fs::write(contents, new_contents).expect("Failed to write to .vaultctx");

    println!("Entry '{}' deleted", entry_name);
}

pub fn vault_context_details(section_name: &str) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");
    let contents = fs::read_to_string(config_path).expect("Failed to read .vaultctx");
    let configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    let mut found = false;
    let mut config_data = String::new();

    fn append_env_var(config_data: &mut String, key: &str, value: Option<&String>) {
        if let Some(val) = value {
            config_data.push_str(&format!("export {}='{}'\n", key, val));
        }
    }

    macro_rules! append_config {
        ($data:expr, $config:expr, $field:ident, $env_var:expr, Format) => {
            if let Some(ref value) = $config.$field {
                let value_str = match *value {
                    Format::Table => "table",
                    Format::Json => "json",
                    Format::Yaml => "yaml",
                };
                $data.push_str(&format!("export {}='{}'\n", $env_var, value_str));
            }
        };
        ($data:expr, $config:expr, $field:ident, $env_var:expr) => {
            if let Some(ref value) = $config.$field {
                $data.push_str(&format!("export {}='{}'\n", $env_var, value));
            }
        };
    }

    for config in configs {
        if config.name == section_name {
            append_env_var(&mut config_data, "VAULT_CONTEXT", Some(&config.name));
            append_env_var(&mut config_data, "VAULT_ADDR", Some(&config.addr));
            append_env_var(&mut config_data, "VAULT_TOKEN", Some(&config.token));

            append_config!(&mut config_data, config, cacert, "VAULT_CACERT");
            append_config!(&mut config_data, config, tls_server_name, "VAULT_TLS_SERVER_NAME");
            append_config!(&mut config_data, config, capath, "VAULT_CAPATH");
            append_config!(&mut config_data, config, client_cert, "VAULT_CLIENT_CERT");
            append_config!(&mut config_data, config, client_key, "VAULT_CLIENT_KEY");
            append_config!(&mut config_data, config, client_timeout, "VAULT_CLIENT_TIMEOUT");
            append_config!(&mut config_data, config, cluster_addr, "VAULT_CLUSTER_ADDR");
            append_config!(&mut config_data, config, format, "VAULT_FORMAT", Format);
            append_config!(&mut config_data, config, license, "VAULT_LICENCE");
            append_config!(&mut config_data, config, license_path, "VAULT_LICENCE_PATH");
            append_config!(&mut config_data, config, log_level, "VAULT_LOG_LEVEL");
            append_config!(&mut config_data, config, max_retries, "VAULT_MAX_RETRIES");
            append_config!(&mut config_data, config, redirect_addr, "VAULT_REDIRECT_ADDR");
            append_config!(&mut config_data, config, skip_verify, "VAULT_SKIP_VERIFY");
            append_config!(&mut config_data, config, cli_no_color, "VAULT_CLI_NO_COLOR");
            append_config!(&mut config_data, config, rate_limit, "VAULT_RATE_LIMIT");
            append_config!(&mut config_data, config, namespace, "VAULT_NAMESPACE");
            append_config!(&mut config_data, config, srv_lookup, "VAULT_SRV_LOOKUP");
            append_config!(&mut config_data, config, mfa, "VAULT_MFA");
            append_config!(&mut config_data, config, http_proxy, "VAULT_HTTP_PROXY");
            append_config!(&mut config_data, config, proxy_addr, "VAULT_PROXY_ADDR");
            append_config!(&mut config_data, config, disable_redirects, "VAULT_DISABLE_REDIRECTS");

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

pub fn switch_to_previous_context() {
    save_current_context();
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    if let Some(previous_context) = get_previous_context() {
        let config_data = String::new();
        apply_and_save_context(&config_data, &previous_context, &home_dir);
        println!("Switched back to previous context {}", previous_context);
    } else {
        println!("No previous context found");
    }
}

pub fn rename_context(old_name: &str, new_name: &str) {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let config_path = home_dir.join(".vaultctx");

    // Read the existing configuration
    let contents = fs::read_to_string(&config_path).expect("Failed to read .vaultctx");
    let mut configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    // Find and update the entry with the new name
    let mut renamed = false;
    for config in &mut configs {
        if config.name == old_name {
            config.name = new_name.to_string();
            renamed = true;
            break;
        }
    }

    if renamed {
        // Write the updated configuration back to the file
        let new_contents = serde_yaml::to_string(&configs).expect("Failed to serialize data");
        fs::write(&config_path, new_contents).expect("Failed to write to .vaultctx");
        println!("Context renamed from '{}' to '{}'", old_name, new_name);
    } else {
        println!("Context '{}' not found", old_name);
    }
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

fn apply_and_save_context(config_data: &str, section_name: &str, home_dir: &PathBuf) {
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

fn append_to_shell_rc(home: &str, line: &str) {
    let bash_rc = format!("{}/.bashrc", home);
    let zsh_rc = format!("{}/.zshrc", home);

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
