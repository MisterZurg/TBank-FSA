use envconfig::Envconfig;


use serde::{Deserialize, Serialize};

// ------------------------
// -- App configuration
// ------------------------
#[derive(Envconfig, Deserialize, Serialize, Debug)]
// #[serde(rename_all = "uppercase")]
pub struct AppConfig {
    #[envconfig(from = "POSTGRES_USER")]
    pub postgres_user: String,
    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub postgres_password: String,
    #[envconfig(from = "POSTGRES_DB")]
    pub postgres_db: String,
    #[envconfig(from = "POSTGRES_HOST")]
    pub postgres_host: String,
    #[envconfig(from = "POSTGRES_PORT")]
    pub postgres_port: u16,
}

impl AppConfig {
    pub fn new() -> Self{
        let cfg = AppConfig::init_from_env().unwrap();

        cfg
    }
}