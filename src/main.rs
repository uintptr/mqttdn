use std::{fs, path::PathBuf, process::Command, time::Duration};

use log::{LevelFilter, error, info};
use mqttdn::{
    config::{Config, MQTTTopic},
    error::{Error, Result},
    osd::Osd,
};
use pidlock::Pidlock;
use rstaples::{logging::StaplesLogger, staples::printkv};

use clap::Parser;
use rumqttc::{Client, Event, Incoming, MqttOptions};
use which::which;

const PID_FILE: &str = "~/.config/mqttdn/mqttdn.pid";

#[derive(Parser, Debug)]
struct UserArgs {
    /// pid file
    #[arg(short, long, default_value=PID_FILE)]
    pid_file: String,

    /// log file
    #[arg(short, long)]
    log_file: Option<String>,

    /// verbose
    #[arg(short, long)]
    verbose: bool,
}

fn expand_tilde<S>(path: S) -> Result<PathBuf>
where
    S: AsRef<str>,
{
    let home = home::home_dir().ok_or(Error::HomeNotFound)?;

    if path.as_ref().starts_with("~/") {
        Ok(home.join(&path.as_ref()[2..]))
    } else {
        Ok(PathBuf::from(path.as_ref()))
    }
}

fn init_pid_file<S>(pid_file: S) -> Result<PathBuf>
where
    S: AsRef<str>,
{
    let pid_path = expand_tilde(pid_file)?;

    let pid_parent = pid_path.parent().ok_or(Error::ParentPathNotFound)?;

    if !pid_parent.exists() {
        fs::create_dir_all(pid_parent)?;
    }

    Ok(pid_path)
}

fn exec_command<S>(command: S) -> Result<()>
where
    S: AsRef<str>,
{
    let words = shell_words::split(command.as_ref())?;

    if words.is_empty() {
        return Err(Error::CommandNotFound);
    }

    info!("executing {}", command.as_ref());

    let program = match which(&words[0]) {
        Ok(v) => v,
        Err(_) => return Err(Error::ProgramNotFound),
    };

    let mut child = Command::new(program).args(&words[1..]).spawn()?;

    match child.wait() {
        Ok(exit) => match exit.success() {
            true => Ok(()),
            false => Err(Error::ExecFailure),
        },
        Err(e) => Err(e.into()),
    }
}

//
// best effort
//
fn process_topic(osd: &Osd, topic: &MQTTTopic, _payload: &str) {
    if let Some(command) = &topic.command {
        if let Err(e) = exec_command(command) {
            error!("{e}");
        }
    }

    if let Some(message) = &topic.osd {
        if let Err(e) = osd.display(message) {
            error!("{e}");
        }
    }
}

fn mqtt_loop(config: &Config) -> Result<()> {
    let osd = Osd::new()?;

    loop {
        info!("connecting to {}", config.server.host);

        let mut options = MqttOptions::new("mqttdn", &config.server.host, 1883);
        options.set_keep_alive(Duration::from_secs(30));

        let (client, mut connection) = Client::new(options, 10);

        for t in &config.topics {
            info!("sub {}", t.topic);
            client.subscribe(&t.topic, rumqttc::QoS::AtLeastOnce)?;
        }

        for event in connection.iter() {
            if let Err(e) = event {
                error!("{e}");
                break;
            }

            if let Ok(Event::Incoming(Incoming::Publish(publish))) = event {
                if publish.payload.is_empty() {
                    continue;
                }

                let topic = &publish.topic;
                let payload = match String::from_utf8(publish.payload.to_vec()) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("{e}");
                        continue;
                    }
                };

                info!("event topic={topic} payload={payload}");

                for t in &config.topics {
                    if &t.topic == topic && t.payload == payload {
                        process_topic(&osd, t, &payload)
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let args = UserArgs::parse();

    let log_level = match args.verbose {
        true => LevelFilter::Info,
        false => LevelFilter::Error,
    };

    let logger = StaplesLogger::new().with_stderr().with_log_level(log_level);

    match args.log_file {
        Some(v) => logger.with_log_file(v).start()?,
        None => logger.start()?,
    }

    let pid_file = init_pid_file(&args.pid_file)?;

    let config = match Config::load() {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(e);
        }
    };

    info!("pid={} exists={}", pid_file.display(), pid_file.exists());

    let mut lock = Pidlock::new_validated(pid_file)?;

    match lock.acquire() {
        Ok(()) => {
            println!("MQTT Desktop Notification");
            printkv("Server", &config.server.host);
            printkv("Topics", config.topics.len());
            printkv("Verbose", args.verbose);

            mqtt_loop(&config)
        }
        Err(pidlock::PidlockError::LockExists) => Err(Error::AlreadyRunning),
        Err(e) => Err(e.into()),
    }
}
