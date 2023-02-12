use thiserror::Error;
use toml;

/// Main error type of the program, transparently handles all other error types.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    VirtualDeviceCreationError(#[from] VirtualDeviceCreationError),

    #[error(transparent)]
    DeviceError(#[from] DeviceError),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}

#[derive(Debug, Error)]
pub enum VirtualDeviceCreationError {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SupportedKeysEmpty(#[from] DeviceError),
}

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    SupportedKeysEmpty(String),

    #[error("Device {0} not found")]
    DeviceNotFound(String),

    #[error("No devices found{0}")]
    DevicesNotFound(&'static str),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Unrecognised key: {0}")]
    ParseKeyError(String),

    #[error("{0}")]
    ReadError(String),

    #[error(transparent)]
    DeserializeError(#[from] toml::de::Error),
}
