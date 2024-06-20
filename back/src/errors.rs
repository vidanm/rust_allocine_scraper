use fantoccini::error::CmdError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    CmdError(CmdError),
    ElementNotFound,
    IllegalArgument,
}

impl Error for AppError {
    // fn description(&self) -> &str {
    //     match *self {
    //         AppError::CmdError(ref e) => e.to_string().as_str(),
    //         AppError::ElementNotFound => "Element not found",
    //         AppError::IllegalArgument => "Illegal argument provided",
    //     }
    // }

    // fn cause(&self) -> Option<&dyn Error> {
    //     match *self {
    //         AppError::CmdError(ref e) => e.source(),
    //         AppError::ElementNotFound |
    //         AppError::IllegalArgument => None,
    //     }
    // }
}

impl From<CmdError> for AppError {
    fn from(err: CmdError) -> AppError {
        AppError::CmdError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
