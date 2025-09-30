//! Error handling for the Cortex compiler

use thiserror::Error;
use std::fmt;

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

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file_path: Option<String>,
    pub line: usize,
    pub column: usize,
    pub source_line: Option<String>,
    pub suggestion: Option<String>,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file_path {
            write!(f, "in {}:{}:{}", file, self.line, self.column)?;
        } else {
            write!(f, "at {}:{}", self.line, self.column)?;
        }
        
        if let Some(source) = &self.source_line {
            write!(f, "\n  {}", source)?;
            write!(f, "\n  {}^", " ".repeat(self.column.saturating_sub(1)))?;
        }
        
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  hint: {}", suggestion)?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DetailedError {
    pub error: CompilerError,
    pub context: ErrorContext,
}

impl fmt::Display for DetailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.error, self.context)
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

pub type Result<T> = std::result::Result<T, DetailedError>;
