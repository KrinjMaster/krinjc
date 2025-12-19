use std::fmt;

#[derive(Debug)]
pub enum Error {
    InputError(String), // wrong number of arguements in cli command
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InputError(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn handle_err(err: Error) {
    eprintln!("Error: {}\n", err);

    match err {
        Error::InputError(_) => {
            eprintln!("run 'jkrinjc --help' to view all commands");
        }
    }
}
