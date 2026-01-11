use crate::build_info::BUILD_INFO;
use crate::config::RatelStreamConfig;
use crate::error::RatelStreamError;
use itertools::Itertools;
use monoio::net::TcpListener;
use tracing::{Level, debug, enabled, info};

mod build_info;
mod config;
pub mod error;

pub async fn run() -> Result<(), &'static RatelStreamError> {
    if enabled!(Level::DEBUG) {
        let envs = std::env::vars()
            .map(|(key, value)| format!("{key}={value}"))
            .join("\n");
        debug!("Envs:\n{envs}");
    }

    let config = RatelStreamConfig::get().as_ref()?;
    info!("Effective configuration: {:#?}", &config);

    info!(
        "Initializing Ratel Stream v{} ({})...",
        BUILD_INFO.version,
        BUILD_INFO.short_commit_hash()
    );

    let listener = TcpListener::bind("127.0.0.1:50002").unwrap();

    info!("Successfully initialized Ratel Stream");

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((_stream, addr)) => {
                println!("accepted a connection from {}", addr);
            }
            Err(e) => {
                println!("accepted connection failed: {}", e);
                return Ok(());
            }
        }
    }

    // Ok(())
}
