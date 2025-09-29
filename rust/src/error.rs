//! Error handling for the Cortex compiler

use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lexical error: {message} at line {line}:{column}")]
    LexicalError { message: String, line: usize, column: usize },
    
    #[error("Parse error: {message} at line {line}:{column}")]
    ParseError { message: String, line: usize, column: usize },
    
    #[error("Type error: {message}")]
    TypeError { message: String },
    
    #[error("Code generation error: {message}")]
    CodeGenError { message: String },
    
    #[error("LLVM error: {message}")]
    LLVMError { message: String },
    
    #[error("IO error: {message}")]
    IOError { message: String },
    
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::IOError {
            message: error.to_string(),
        }
    }
}

// Removed inkwell dependency
