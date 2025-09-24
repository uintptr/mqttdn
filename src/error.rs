use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ConfigFileNotFound,
    HomeNotFound,
    AlreadyRunning,
    ParentPathNotFound,
    ConnectionFailure,
    ProgramNotFound,
    ExecFailure,
    CommandNotFound,
    //
    // 2nd party
    //
    #[from]
    Io(std::io::Error),
    //
    // 3rd party
    //
    #[from]
    Rstaples(rstaples::error::Error),
    #[from]
    DeToml(toml::de::Error),
    #[from]
    PidLock(pidlock::PidlockError),
    #[from]
    Mqtt(rumqttc::ClientError),
    #[from]
    ShellError(shell_words::ParseError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
