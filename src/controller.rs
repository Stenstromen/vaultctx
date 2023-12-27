use std::{path::Path, fs::File, io::Write};

use crate::model::{Config, Format, LogLevel};

pub fn create_vaultctx_file(_configs: Vec<Config>) {
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
        log_level: Some(LogLevel::Info),
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