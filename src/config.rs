use config::{Config, ConfigError, Environment, File};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Path to store cloned repositories
    pub repodir: PathBuf,
    /// Path to store cache
    pub cachedir: PathBuf,
    /// Port to listen on
    pub port: u16,
    /// Interface to listen on
    pub host: String,
    /// Base URL
    pub base_url: String,
    /// Number of worker threads
    pub workers: usize,
}

impl Settings {
    /// Load the configuration from file and environment.
    ///
    /// # Errors
    ///
    /// * File cannot be read or parsed
    /// * Environment variables cannot be parsed
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("hoc.toml").required(false))
            .add_source(Environment::with_prefix("hoc"))
            .set_default("repodir", "./repos")?
            .set_default("cachedir", "./cache")?
            .set_default("workers", 4)?
            .set_default("port", 8080)?
            .set_default("host", "0.0.0.0")?
            .build()?
            .try_deserialize()
    }
}
