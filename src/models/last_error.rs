use radiation_counter_api::{ErrorCode};

/// Error variants which can be returned by the Radiation Counter
#[derive(Clone, Debug, GraphQLEnum)]
pub enum Error {
    /// No errors
    None,
    /// The command to fetch the last error failed
    CommandError = 0x04,
    /// A reset had to occur
    ResetOccurred,
    /// Unknown command received
    UnknownCommand,
    /// All other errors
    UnknownError
}

fn to_error(error_code: ErrorCode) -> Error {
    match error_code {
        ErrorCode::None => Error::None,
        ErrorCode::CommandError => Error::CommandError,
        ErrorCode::ResetOccurred => Error::ResetOccurred,
        ErrorCode::UnknownCommand => Error::UnknownCommand,
        ErrorCode::UnknownError => Error::UnknownError,
    }
}

impl Into<Error> for ErrorCode {
    fn into(self) -> Error {
        to_error(self)
    }
}