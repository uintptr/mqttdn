use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ConfigFileNotFound,
    HomeNotFound,
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
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
