use std::fs::{read_to_string, write};
use std::future::Future;
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub qsign: QSign,
    pub developer: Developer,
    pub sql: Sql,
    #[cfg(feature = "onebot")]
    pub onebot: OneBot,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg(feature = "onebot")]
pub struct OneBot {
    pub http: HTTTPServer,
    pub ws: WebSocketServer,
    pub pws: PassiveWebSocketServer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg(feature = "onebot")]
pub struct HTTTPServer {
    pub enable: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg(feature = "onebot")]
pub struct WebSocketServer {
    pub enable: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg(feature = "onebot")]
pub struct PassiveWebSocketServer {
    pub enable: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QSign {
    pub server: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Developer {
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sql {
    pub enable: bool,
    pub address: String,
}

pub fn parse_local_config(path: PathBuf) -> Option<Config> {
    info!("Local config file: {}", path.to_str().unwrap());
    if let Ok(contents) = read_to_string(path) {
        let config: Config = toml::from_str(&contents).unwrap();
        return Some(config);
    } else {
        error!("Failed to read config file");
    }
    None
}

