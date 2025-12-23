use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};

// CError - custom error handling
#[derive(Debug)]
pub enum CError {
    Input(String),
    Compile(String),
    Context {
        message: &'static str,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CError::Input(msg) => write!(f, "Input error: {msg}"),
            CError::Compile(msg) => write!(f, "Compile error: {msg}"),
            CError::Context { message, source } => {
                write!(f, "{message}: {source}")
            }
        }
    }
}

impl Error for CError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CError::Context { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

pub trait ResultContext<T> {
    fn ctx(self, message: &'static str) -> Result<T, CError>;
}

impl<T, E> ResultContext<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ctx(self, message: &'static str) -> Result<T, CError> {
        self.map_err(|e| CError::Context {
            message,
            source: Box::new(e),
        })
    }
}

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

pub fn run(source: &str) -> Result<(), CError> {
    let scanner = Scanner { source: source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("Token: {}", token.a);
    }

    Ok(())
}

pub fn run_from_file(path: &str) -> Result<(), CError> {
    let file = File::open(path).ctx("Failed to open file")?;

    if file.metadata().ctx("Failed to read metadata")?.is_dir() {
        return Err(CError::Input("Path is a directory".into()));
    }

    let mut file_code = String::new();

    BufReader::new(file)
        .read_to_string(&mut file_code)
        .ctx("Failed to read file contents")?;

    run(&file_code)
}

pub fn run_promting() -> Result<(), CError> {
    println!("Running from prompts!...");
    Ok(())
}
