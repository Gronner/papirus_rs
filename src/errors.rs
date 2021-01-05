use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: Box<ErrorKind>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error {
            kind: Box::new(kind),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.kind {
            ErrorKind::DisplayUnsupportedCog => None,
            ErrorKind::DisplayPanelBroken => None,
            ErrorKind::DisplayDcFailed => None,
            ErrorKind::DisplayUnknown => None,
            ErrorKind::UnexpectedError(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.kind {
            ErrorKind::DisplayUnsupportedCog => write!(f, "You are using an unsupported chip on glass"),
            ErrorKind::DisplayPanelBroken => write!(f, "Your panel is broken, please replace it"),
            ErrorKind::DisplayDcFailed => write!(f, "Voltage to low, check if the power supply is working properly and fit for the screen"),
            ErrorKind::DisplayUnknown => write!(f, "An unkown error has occured with the Display"),
            ErrorKind::UnexpectedError(state) => write!(f, "An unexpected error has occured: {}", state),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Chip on glass not supported by driver
    DisplayUnsupportedCog,
    /// When the panel breaks
    DisplayPanelBroken,
    /// Volate pump level to low for operation
    DisplayDcFailed,
    /// Other display failure modes
    DisplayUnknown,
    /// When an unexpected error is encountered
    UnexpectedError(String),
}
