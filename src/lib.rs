use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};

// CError - custom error handling
#[derive(Debug)]
pub enum CError {
    Input(String), // wrong number of arguements in cli command
    IO(String),    // wrong filepath
}

impl From<std::io::Error> for CError {
    fn from(_: std::io::Error) -> Self {
        CError::IO("".to_string())
    }
}

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CError::Input(msg) => write!(f, "{}", msg),
            CError::IO(msg) => write!(f, "{}", msg),
        }
    }
}
//

// Token initialization
struct Token<'a> {
    a: &'a str,
}

struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for line in self.source.lines() {
            tokens.push(Token { a: line });
        }

        tokens
    }
}

pub fn run_from_file(path: &str) -> Result<(), CError> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(CError::IO("Incorrect filepath!".to_string())),
    };
    let mut reader = BufReader::new(&file);
    let mut file_code = String::new();

    if file.metadata()?.is_dir() {
        return Err(CError::IO(
            "Current path is a directory, not a file!".to_string(),
        ));
    }

    let _ = reader.read_to_string(&mut file_code);

    run(&file_code);

    Ok(())
}

pub fn run_promting() -> Result<(), CError> {
    println!("Running from prompts!...");
    Ok(())
}

pub fn run(source: &str) {
    let scanner = Scanner { source: source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("Token: {}", token.a);
    }
}
