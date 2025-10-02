//! Cortex Compiler Library
//! 
//! A high-performance AI/ML compiler written in Rust.
//! 
//! This library provides a complete compilation pipeline for the Cortex programming language:
//! - **Lexer**: Tokenizes Cortex source code into a stream of tokens
//! - **Parser**: Parses tokens into an Abstract Syntax Tree (AST)
//! - **Interpreter**: Directly interprets the AST without compilation
//! - **Error Handling**: Comprehensive error reporting with context
//! 
//! # Examples
//! 
//! ```
//! use cortex_rust::{lexer::Lexer, parser::Parser as CortexParser, codegen::Interpreter};
//! 
//! let source = r#"
//!     func add[a, b] |
//!         return[a + b]
//!     ^
//!     
//!     let result := add[10, 20]
//!     print[result]
//! "#;
//! 
//! // Tokenize
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.tokenize().unwrap();
//! 
//! // Parse
//! let mut parser = CortexParser::new(tokens);
//! let ast = parser.parse().unwrap();
//! 
//! // Interpret
//! let mut interpreter = Interpreter::new();
//! interpreter.interpret(&ast).unwrap();
//! ```

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod codegen;
pub mod error;

pub use lexer::Lexer;
pub use parser::Parser;
pub use ast::Program;
pub use codegen::Interpreter;
pub use error::CompilerError;

