use std::{env, sync::atomic::AtomicU16};

use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::sync::atomic::Ordering::SeqCst;
use super::config_types::{KafkaConfiguration, LoggingConfiguration, ServerConfiguration};


pub static SERVER_PORT: AtomicU16 = AtomicU16::new(0);
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configuration {
    pub kafka: KafkaConfiguration,
    pub server: ServerConfiguration,
    pub logging: LoggingConfiguration,
}

impl Configuration {
    pub fn load() -> Result<Self, ConfigError> {
        let profiles_raw_string = env::var("RUST_PROFILES_ACTIVE").unwrap_or_default();
        let active_profiles: Vec<&str> = profiles_raw_string
            .split(',')
            .into_iter()
            .map(|p| p.trim())
            .filter(|p| !(*p).is_empty())
            .collect();


        // Load always properties of application.yml
        let mut builder =
            Config::builder().add_source(File::with_name("resources/application-local.yml"));

        // Load property files for profiles
        // for profile in active_profiles {
        //     builder = builder.add_source(
        //         File::with_name(&format!("resources/application-{}.yml", profile)).required(false),
        //     );
        // }
        let parsed_config: Result<Configuration, ConfigError> = builder.build().unwrap().try_deserialize();

        // Set server port statically
        if let Ok(config) = &parsed_config {
            SERVER_PORT.store(3005, SeqCst);
        }

        // Return config
        parsed_config
    }
}