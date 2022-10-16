use thiserror::Error;

/// Main error type of the program, transparently handles all other error types.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    VirtualDeviceCreationError(#[from] VirtualDeviceCreationError),

    #[error(transparent)]
    DeviceError(#[from] DeviceError),
}

#[derive(Debug, Error)]
pub enum VirtualDeviceCreationError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SupportedKeysEmpty(#[from] DeviceError),
}

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    SupportedKeysEmpty(String),

    #[error("Device {0} not found")]
    DeviceNotFound(String),

    #[error("No devices found{0}")]
    DevicesNotFound(&'static str),
}
