
/// These represent recoverable errors that should be logged as part of the sync job log
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,

    #[error("{0}")]
    ParseError(String),
}

// #[derive(thiserror::Error, Debug)]
// pub enum TikaError {
//     #[error("Unknown error")]
//     Unknown,

//     #[error("{0}")]
//     ParseError(String),

//     #[error("{0}")]
//     JniError(String),
// }

pub type ExtractResult<T> = Result<T, Error>;