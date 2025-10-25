use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmacsLogError {
    #[error("Invalid log level symbol: {0}")]
    InvalidLogLevelSymbol(String),
    #[error("Logger has already been initialized")]
    LoggerAlreadyInitialized,
}
