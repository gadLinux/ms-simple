use std::fmt;
use std::result;
use std::convert::From;
use std::error;
use std::io;

pub type Result<T> = result::Result<T, ConfigError>;


#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    Io(io::Error),
    Serde(serde_yaml::Error),
}

impl From<io::Error> for ConfigError {
    #[inline]
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    #[inline]
    fn from(err: serde_yaml::Error) -> ConfigError {
        ConfigError::Serde(err)
    }

}


impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Serde(ref err) => err.fmt(f),
            ConfigError::Io(ref err) => err.fmt(f),
            ConfigError::NotFound => write!(f, "Cannot find the configuration"),
        }
    }
}


impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Serde(ref err) => err.description(),
            ConfigError::Io(ref err) => err.description(),
            ConfigError::NotFound => "resource not found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}
