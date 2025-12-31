use colored_text::Colorize;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

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

#[derive(Debug)]
enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
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
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "type: {:?}, lexeme: {}, literal: {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

struct Scanner<'a> {
    source: &'a str,
    filepath: &'a str,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line_count: u32,
    column_count: u32,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str, filepath: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            filepath,
            start: 0,
            current: 0,
            line_count: 1,
            column_count: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<(), CError> {
        for line in self.source.lines() {
            for c in line.chars() {
                match c {
                    '(' => self.add_token(TokenType::LEFT_PAREN, None),
                    ')' => self.add_token(TokenType::RIGHT_PAREN, None),
                    '}' => self.add_token(TokenType::LEFT_BRACE, None),
                    '{' => self.add_token(TokenType::RIGHT_BRACE, None),
                    ',' => self.add_token(TokenType::COMMA, None),
                    '.' => self.add_token(TokenType::DOT, None),
                    '-' => self.add_token(TokenType::MINUS, None),
                    '+' => self.add_token(TokenType::PLUS, None),
                    ';' => self.add_token(TokenType::SEMICOLON, None),
                    '*' => self.add_token(TokenType::STAR, None),
                    _ => {
                        return Err(CError::Compile(self.get_error_message(c, line)));
                    }
                }
                self.column_count += 1;
            }
            self.line_count += 1;
            self.column_count = 1;
        }

        self.add_token(TokenType::EOF, None);
        Ok(())
    }

    fn get_error_message(&self, incorrect_char: char, line: &str) -> String {
        let error_pointer: String = " ".repeat(self.column_count as usize - 1) + "^";
        format!(
            "Unexpected character `{}`\n   {}{}:{}:{}\n\n{}{}\n{}{}{}\n{}{}{}",
            incorrect_char,
            "--> ".blue(),
            self.filepath.replacen("./", ".../", 1),
            self.line_count,
            self.column_count,
            " ".repeat(self.line_count as usize),
            "|".blue().bold(),
            self.line_count.blue().bold(),
            " | ".blue().bold(),
            line,
            " ".repeat(self.line_count as usize),
            "| ".blue().bold(),
            error_pointer.bold().red()
        )
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start as usize..self.current as usize].to_string();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
        });
    }
}

pub fn run(source: &str, filepath: &str) -> Result<(), CError> {
    let mut scanner = Scanner::new(source, filepath);
    let _ = scanner.scan_tokens()?;

    if scanner.tokens.len() == 1 {
        return Err(CError::Compile(format!("Empty source code")));
    }

    Ok(())
}

pub fn run_from_file(path: &str) -> Result<(), CError> {
    let file_path = Path::new(path);

    let file = File::open(file_path).ctx("Failed to open file")?;

    if file
        .metadata()
        .ctx("Failed to read metadata of the file")?
        .is_dir()
    {
        return Err(CError::Input(format!("Path is a directory")));
    }

    match file_path.extension().and_then(OsStr::to_str).unwrap_or("") {
        "krj" => {}
        _ => {
            return Err(CError::Input(format!(
                "Incorrect or emtpy file extension, use `.krj`"
            )))
        }
    }

    let mut file_code = String::new();

    BufReader::new(file)
        .read_to_string(&mut file_code)
        .ctx("Failed to read file contents")?;

    run(&file_code, path)
}

pub fn run_promting() -> Result<(), CError> {
    println!("Running from prompts!...");
    todo!("implement run_promting");

    Ok(())
}
