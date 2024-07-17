use std::fmt::Display;

#[derive(Debug)]
pub enum CommandError {
    InvalidArgsError(ArgParseError),
    NotFoundError,
}

#[derive(Debug)]
pub enum ArgParseError {
    TooManyArguments,
    MissingArgs(&'static str),
}

//TODO clean
impl Display for ArgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgParseError::TooManyArguments => write!(f, "Too Many Arguments"),
            ArgParseError::MissingArgs(arg) => write!(f, "Missing argument: {arg}"),
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::InvalidArgsError(ArgParseError::TooManyArguments) => {
                write!(f, "{}", ArgParseError::TooManyArguments)
            }
            &Self::InvalidArgsError(ArgParseError::MissingArgs(arg)) => {
                write!(f, "{}", ArgParseError::MissingArgs(arg))
            }
            &Self::NotFoundError => {
                write!(f, "command not found")
            }
        }
    }
}
