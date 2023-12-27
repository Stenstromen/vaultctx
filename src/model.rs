use core::fmt;

/* use clap::{command, Parser}; */
use serde::{ Deserialize, Serialize };

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
    pub max_retries: Option<String>,
    pub redirect_addr: Option<String>,
    pub skip_verify: Option<String>,
    pub cli_no_color: Option<String>,
    pub rate_limit: Option<String>,
    pub namespace: Option<String>,
    pub srv_lookup: Option<String>,
    pub mfa: Option<String>,
    pub http_proxy: Option<String>,
    pub proxy_addr: Option<String>,
    pub disable_redirects: Option<String>,
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