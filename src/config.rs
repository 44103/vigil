use std::path::PathBuf;
use directories::ProjectDirs;
use serde::Deserialize;
use config::{Config, File};

pub const APP_QUALIFIER: &str = "com";
pub const APP_ORGANIZATION: &str = "44103";
pub const APP_NAME: &str = "vigil";

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub log_output_path: Option<String>,
    pub monitor_interval_secs: Option<u64>,
    pub server_port: Option<u16>,
}

pub fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
}

pub fn load_config() -> AppConfig {
    let proj_dirs = get_project_dirs();
    let mut builder = Config::builder();

    if let Some(dirs) = proj_dirs {
        let config_path = dirs.config_dir().join("config.toml");
        if config_path.exists() {
            builder = builder.add_source(File::from(config_path));
        }
    }

    builder
        .build()
        .and_then(|c| c.try_deserialize())
        .unwrap_or_else(|_| AppConfig {
            log_output_path: None,
            monitor_interval_secs: None,
            server_port: None,
        })
}

pub fn resolve_data_dir(config: &AppConfig) -> PathBuf {
    if let Some(ref path) = config.log_output_path {
        return PathBuf::from(path);
    }

    get_project_dirs()
        .map(|dirs| dirs.data_local_dir().join("data"))
        .unwrap_or_else(|| PathBuf::from("./data"))
}
