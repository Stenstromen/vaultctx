use core::fmt;
use std::{fs, path::Path};

/* use clap::{command, Parser}; */
use serde::{ Deserialize, Serialize };

use crate::controller::create_vaultctx_file;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenameArgs(pub String, pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub addr: String,
    pub token: String,
    pub cacert: Option<String>,
    pub tls_server_name: Option<String>,
    pub capath: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
    pub client_timeout: Option<String>,
    pub cluster_addr: Option<String>,
    pub format: Option<Format>,
    pub license: Option<String>,
    pub license_path: Option<String>,
    pub log_level: Option<LogLevel>,
    pub max_retries: Option<MaxRetries>,
    pub redirect_addr: Option<String>,
    pub skip_verify: Option<SkipVerify>,
    pub cli_no_color: Option<CliNoColor>,
    pub rate_limit: Option<String>,
    pub namespace: Option<String>,
    pub srv_lookup: Option<String>,
    pub mfa: Option<String>,
    pub http_proxy: Option<String>,
    pub proxy_addr: Option<String>,
    pub disable_redirects: Option<DisableRedirects>,
}

pub struct SharedData {
    pub configs: Vec<Config>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Table,
    Json,
    Yaml,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MaxRetries {
    Zero,
    One,
    Two,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkipVerify {
    True,
    False,
    Zero,
    One,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisableRedirects {
    True,
    False,
    Zero,
    One,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CliNoColor {
    True,
    False,
    Zero,
    One,
}

impl SharedData {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Failed to find home directory");
        let config_path = home_dir.join(".vaultctx");

        if !Path::new(&config_path).exists() {
            println!("Initial dummy config created at ~/.vaultctx");
            create_vaultctx_file(Vec::new());
            panic!("Failed to create initial config");
        }

        let contents = fs::read_to_string(&config_path).expect("Failed to read config");
        let configs: Vec<Config> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

        Self {
            configs,
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Format::Table => "table",
            Format::Json => "json",
            Format::Yaml => "yaml",
        })
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Err => "error",
        })
    }
}

impl fmt::Display for MaxRetries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            MaxRetries::Zero => "0",
            MaxRetries::One => "1",
            MaxRetries::Two => "2",
        })
    }
}

impl fmt::Display for SkipVerify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            SkipVerify::True => "true",
            SkipVerify::False => "false",
            SkipVerify::Zero => "0",
            SkipVerify::One => "1",
        })
    }
}

impl fmt::Display for DisableRedirects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            DisableRedirects::True => "true",
            DisableRedirects::False => "false",
            DisableRedirects::Zero => "0",
            DisableRedirects::One => "1",
        })
    }
}

impl fmt::Display for CliNoColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            CliNoColor::True => "true",
            CliNoColor::False => "false",
            CliNoColor::Zero => "0",
            CliNoColor::One => "1",
        })
    }
}
