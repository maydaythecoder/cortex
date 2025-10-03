//! Error handling for the Cortex compiler

use thiserror::Error;
use std::fmt;

/// Source location information for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self { line, column, length }
    }
}

/// Context information for enhanced error reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file_path: Option<String>,
    pub source_line: String,
    pub suggestion: Option<String>,
    pub help: Option<String>,
}

impl ErrorContext {
    pub fn new(source_line: String) -> Self {
        Self {
            file_path: None,
            source_line,
            suggestion: None,
            help: None,
        }
    }
    
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
    
    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
}

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, got {actual} at line {line}:{column}")]
    UnexpectedToken { expected: String, actual: String, line: usize, column: usize },
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Invalid syntax at line {line}:{column}: {message}")]
    InvalidSyntax { line: usize, column: usize, message: String },
}

#[derive(Error, Debug, Clone)]
pub enum CompilerError {
    #[error("Lexical error: {message} at line {line}:{column}")]
    LexicalError { message: String, line: usize, column: usize },
    
    #[error("Parse error: {message} at line {line}:{column}")]
    ParseError { message: String, line: usize, column: usize },
    
    #[error("Type error: {message} at line {line}:{column}")]
    TypeError { message: String, line: usize, column: usize },
    
    #[error("Runtime error: {message} at line {line}:{column}")]
    RuntimeError { message: String, line: usize, column: usize },
    
    #[error("Code generation error: {message}")]
    CodeGenError { message: String },
    
    #[error("IO error: {message}")]
    IOError { message: String },
    
    #[error("Internal error: {message}")]
    InternalError { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file_path {
            writeln!(f, "  --> {}", file)?;
        }
        
        if !self.source_line.is_empty() {
            writeln!(f, "   | {}", self.source_line)?;
        }
        
        if let Some(suggestion) = &self.suggestion {
            writeln!(f, "   = suggestion: {}", suggestion)?;
        }
        
        if let Some(help) = &self.help {
            writeln!(f, "   = help: {}", help)?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DetailedError {
    pub error: CompilerError,
    pub context: Option<ErrorContext>,
}

impl DetailedError {
    pub fn new(error: CompilerError) -> Self {
        Self {
            error,
            context: None,
        }
    }
    
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl fmt::Display for DetailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        if let Some(context) = &self.context {
            write!(f, "\n{}", context)?;
        }
        Ok(())
    }
}

impl std::error::Error for DetailedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::IOError {
            message: error.to_string(),
        }
    }
}

impl From<anyhow::Error> for CompilerError {
    fn from(error: anyhow::Error) -> Self {
        CompilerError::InternalError {
            message: error.to_string(),
        }
    }
}

impl From<ParseError> for DetailedError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::UnexpectedToken { expected, actual, line, column } => {
                let compiler_error = CompilerError::ParseError {
                    message: format!("Expected {}, got {}", expected, actual),
                    line,
                    column,
                };
                DetailedError::new(compiler_error)
            }
            ParseError::UnexpectedEof => {
                let compiler_error = CompilerError::ParseError {
                    message: "Unexpected end of input".to_string(),
                    line: 0,
                    column: 0,
                };
                DetailedError::new(compiler_error)
            }
            ParseError::InvalidSyntax { line, column, message } => {
                let compiler_error = CompilerError::ParseError {
                    message,
                    line,
                    column,
                };
                DetailedError::new(compiler_error)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, DetailedError>;
