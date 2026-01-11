use crate::error::RatelStreamError;
use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct RatelStreamConfig {
    // pub management: ManagementConfig,
}

impl RatelStreamConfig {
    pub fn get() -> &'static Result<Self, RatelStreamError> {
        static CONFIG: OnceLock<Result<RatelStreamConfig, RatelStreamError>> = OnceLock::new();

        CONFIG.get_or_init(|| {
            let profile = env::var("RATELSTREAM_PROFILE").unwrap_or_else(|_| "production".into());
            let config_dir =
                env::var("RATELSTREAM_CONFIG_ROOT").unwrap_or_else(|_| "./config".into());

            info!(
                "Using configuration profile {} and config root directory {}",
                &profile, &config_dir
            );

            let default_filename = format!("{}/{}", &config_dir, "default");
            let profile_filename = format!("{}/{}", &config_dir, &profile);

            Config::builder()
                .add_source(File::with_name(&default_filename))
                .add_source(File::with_name(&profile_filename))
                .add_source(Environment::with_prefix("RATELSTREAM").separator("__"))
                .build()?
                .try_deserialize()
                .map_err(RatelStreamError::ConfigError)
        })
    }
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct ManagementConfig {
//     pub server: HttpConfig,
// }
//
// #[derive(Debug, Deserialize, Clone, Validate)]
// pub struct HttpConfig {
//     pub address: String,
//     pub port: u16,
//     #[validate(custom(function = "validate_request_timeout"))]
//     pub request_timeout: Duration,
// }

// fn validate_request_timeout(duration: &Duration) -> Result<(), ValidationError> {
//     if duration.to_std().is_none() {
//         return Err(ValidationError::new(
//             "Invalid request timeout, it must not have months and years",
//         ));
//     }
//     Ok(())
// }
