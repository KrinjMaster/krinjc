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
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
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
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
}

impl Token {
    // fn to_string(&self) -> String {
    //     format!(
    //         "type: {:?}, lexeme: {}, literal: {:?}",
    //         self.token_type, self.lexeme, self.literal
    //     )
    // }
}

struct Scanner<'a> {
    source: &'a str,
    filepath: &'a str,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line_count: u32,
    column_count: u32,
    diagnostics: Diagnostics,
}

#[derive(Debug)]
pub struct Diagnostics {
    pub errors: Vec<CError>,
}

impl Diagnostics {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn push_error(&mut self, error: CError) {
        self.errors.push(error);
    }
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
            diagnostics: Diagnostics { errors: vec![] },
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            let c = self.advance();

            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '}' => self.add_token(TokenType::LeftBrace, None),
                '{' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => {
                    if self.check_next('=') {
                        self.current += 1;
                        self.add_token(TokenType::BangEqual, None);
                    } else {
                        self.add_token(TokenType::Bang, None)
                    }
                }
                '=' => {
                    if self.check_next('=') {
                        self.current += 1;
                        self.add_token(TokenType::EqualEqual, None);
                    } else {
                        self.add_token(TokenType::Equal, None);
                    }
                }
                '<' => {
                    if self.check_next('=') {
                        self.current += 1;
                        self.add_token(TokenType::LessEqual, None);
                    } else {
                        self.add_token(TokenType::Less, None);
                    }
                }
                '>' => {
                    if self.check_next('=') {
                        self.current += 1;
                        self.add_token(TokenType::GreaterEqual, None);
                    } else {
                        self.add_token(TokenType::Greater, None);
                    }
                }
                '/' => {
                    if self.check_next('/') {
                        while !self.check_next('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }
                '\"' => {
                    self.start += 1;
                    while !self.is_at_end() && !self.check_next('"') {
                        self.current += 1;

                        if self.check_next('"') {
                            return self.add_token(
                                TokenType::String,
                                Some(Literal::String(
                                    self.source[self.start as usize..self.current as usize]
                                        .to_string(),
                                )),
                            );
                        }
                    }

                    if self.is_at_end() {
                        self.diagnostics.push_error(CError::Compile(
                            self.get_error_message(format!("Unterminated string")),
                        ));
                    }
                }
                '\n' => {
                    self.line_count += 1;
                    self.column_count = 1;
                }
                '\t' => {
                    self.column_count += 1;
                }
                '\r' => {
                    self.column_count += 1;
                }
                ' ' => {
                    self.column_count += 1;
                }
                _ => {
                    self.diagnostics.push_error(CError::Compile(
                        self.get_error_message(format!("Unexpected character: `{}`", c)),
                    ));
                }
            }

            self.start = self.current;
        }

        self.add_token(TokenType::Eof, None);
    }

    fn check_next(&self, expected: char) -> bool {
        if self.current < self.source.len() as u32 {
            return self.source.chars().nth(self.current as usize) == Some(expected);
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        self.column_count += 1;
        c
    }

    fn get_error_message(&self, message: String) -> String {
        let line_no = self.line_count;
        let line_no_colored = line_no.to_string().blue().bold();
        let line = self
            .source
            .lines()
            .nth((line_no - 1) as usize)
            .unwrap()
            .to_string();
        let col_no = self.column_count;

        let width = line_no.to_string().len();
        let gutter_pad = " ".repeat(width);

        let arrow = "   -->".blue().bold();
        let gutter = "|".blue().bold();

        let file = self.filepath.replacen("./", ".../", 1);
        let pointer = format!("{:>width$}^", "", width = col_no as usize)
            .red()
            .bold();

        format!(
            "{message}\n\
            {arrow} {file}:{line}:{col}\n\
            \n\
            {gutter_pad} {gutter}\n\
            {line_no_colored:>width$} {gutter} {src}\n\
            {gutter_pad} {gutter}{pointer}",
            arrow = arrow,
            file = file,
            line = line_no,
            col = col_no,
            gutter_pad = gutter_pad,
            width = width,
            src = line,
            pointer = pointer,
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

pub fn run(source: &str, filepath: &str) -> Diagnostics {
    let mut scanner = Scanner::new(source, filepath);
    let _ = scanner.scan_tokens();

    if scanner.tokens.len() == 1 {
        return Diagnostics {
            errors: vec![CError::Compile(format!("Empty source code"))],
        };
    }

    if scanner.diagnostics.has_errors() {
        return scanner.diagnostics;
    }

    for token in &scanner.tokens {
        println!("{:?}", token);
    }

    scanner.diagnostics
}

pub fn run_from_file(path: &str) -> Result<Diagnostics, CError> {
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

    Ok(run(&file_code, path))
}

pub fn run_promting() -> Result<Diagnostics, CError> {
    println!("Running from prompts!...");
    todo!("implement run_promting");

    Ok(Diagnostics { errors: vec![] })
}
