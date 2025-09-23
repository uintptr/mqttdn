use log::error;
use mqttdn::{config::Config, error::Result};
use rstaples::{logging::StaplesLogger, staples::printkv};

fn main() -> Result<()> {
    StaplesLogger::new()
        .with_stderr()
        .with_log_level(log::LevelFilter::Info)
        .start()?;

    let config = match Config::load() {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(e);
        }
    };

    println!("MQTT Desktop Notification");
    printkv("Server", config.server.host);
    printkv("Topics", config.topics.len());

    Ok(())
}
