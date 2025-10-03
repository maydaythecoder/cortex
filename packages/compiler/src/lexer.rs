//! Cortex Lexer
//! 
//! Tokenizes Cortex source code into a stream of tokens using the logos crate.
//! Provides enhanced error reporting with source context and token classification.

use logos::Logos;
use std::fmt;
use crate::error::{CompilerError, DetailedError, ErrorContext};

/// Token categories for better organization and classification
#[derive(Debug, Clone, PartialEq)]
pub enum TokenCategory {
    Literal,
    Keyword,
    Operator,
    Delimiter,
    Special,
    Error,
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // ================================
    // LITERALS
    // ================================
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?", |lex| lex.slice().to_string())]
    Number(String),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    String(String),
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    // ================================
    // KEYWORDS
    // ================================
    #[token("let")]
    Let,
    
    #[token("func")]
    Func,
    
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    #[token("while")]
    While,
    
    #[token("for")]
    For,
    
    #[token("return")]
    Return,
    
    #[token("true")]
    True,
    
    #[token("false")]
    False,
    
    #[token("null")]
    Null,
    
    #[token("in")]
    In,
    
    // ================================
    // OPERATORS
    // ================================
    // Arithmetic operators
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Multiply,
    
    #[token("/")]
    Divide,
    
    #[token("%")]
    Modulo,
    
    #[token("**")]
    Power,
    
    // Assignment operators
    #[token(":=")]
    Assign,
    
    #[token("::")]
    AssignConst,
    
    // Comparison operators
    #[token("==")]
    Equals,
    
    #[token("!=")]
    NotEquals,
    
    #[token("<")]
    LessThan,
    
    #[token("<=")]
    LessEqual,
    
    #[token(">")]
    GreaterThan,
    
    #[token(">=")]
    GreaterEqual,
    
    // Logical operators
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    #[token("!")]
    Not,
    
    // Range operator
    #[token("..")]
    Range,
    
    // AI/ML specific operators
    #[token("@")]
    DotProduct,
    
    #[token("∇")]
    Gradient,
    
    #[token("∂")]
    Partial,
    
    // ================================
    // DELIMITERS
    // ================================
    #[token("[")]
    LeftBracket,
    
    #[token("]")]
    RightBracket,
    
    #[token("{")]
    LeftBrace,
    
    #[token("}")]
    RightBrace,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("|")]
    Pipe,
    
    #[token("^")]
    Caret,
    
    #[token(",")]
    Comma,
    
    #[token(";")]
    Semicolon,
    
    #[token(":")]
    Colon,
    
    #[token(".")]
    Dot,
    
    // ================================
    // SPECIAL
    // ================================
    #[regex(r"\n")]
    Newline,
    
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,
    
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiLineComment,
    
    // Whitespace (ignored)
    #[regex(r"[ \t\r]+", logos::skip)]
    Whitespace,
    
    // Error token for invalid characters
    Error,
}

/// Enhanced token information with metadata and helper methods
#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub source_text: String,
}

impl TokenInfo {
    /// Creates a new TokenInfo with the given parameters
    pub fn new(token: Token, line: usize, column: usize, length: usize, source_text: String) -> Self {
        Self {
            token,
            line,
            column,
            length,
            source_text,
        }
    }
    
    /// Returns the category of this token
    pub fn category(&self) -> TokenCategory {
        self.token.category()
    }
    
    /// Returns true if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        self.token.is_keyword()
    }
    
    /// Returns true if this token is an operator
    pub fn is_operator(&self) -> bool {
        self.token.is_operator()
    }
    
    /// Returns true if this token is a literal
    pub fn is_literal(&self) -> bool {
        self.token.is_literal()
    }
    
    /// Returns true if this token is a delimiter
    pub fn is_delimiter(&self) -> bool {
        self.token.is_delimiter()
    }
    
    /// Returns the string representation of the token
    pub fn as_string(&self) -> String {
        self.token.as_string()
    }
    
    /// Returns the precedence level for operators (0 = no precedence)
    pub fn precedence(&self) -> u8 {
        self.token.precedence()
    }
    
    /// Returns true if this operator is left-associative
    pub fn is_left_associative(&self) -> bool {
        self.token.is_left_associative()
    }
}

impl Token {
    /// Returns the category of this token
    pub fn category(&self) -> TokenCategory {
        match self {
            // Literals
            Token::Number(_) | Token::String(_) | Token::Identifier(_) => TokenCategory::Literal,
            
            // Keywords
            Token::Let | Token::Func | Token::If | Token::Else | Token::While | 
            Token::For | Token::Return | Token::True | Token::False | 
            Token::Null | Token::In => TokenCategory::Keyword,
            
            // Operators
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Modulo |
            Token::Power | Token::Assign | Token::AssignConst | Token::Equals | 
            Token::NotEquals | Token::LessThan | Token::LessEqual | Token::GreaterThan |
            Token::GreaterEqual | Token::And | Token::Or | Token::Not | Token::Range |
            Token::DotProduct | Token::Gradient | Token::Partial => TokenCategory::Operator,
            
            // Delimiters
            Token::LeftBracket | Token::RightBracket | Token::LeftBrace | Token::RightBrace |
            Token::LeftParen | Token::RightParen | Token::Pipe | Token::Caret | 
            Token::Comma | Token::Semicolon | Token::Colon | Token::Dot => TokenCategory::Delimiter,
            
            // Special
            Token::Newline | Token::Comment | Token::MultiLineComment | Token::Whitespace => TokenCategory::Special,
            
            // Error
            Token::Error => TokenCategory::Error,
        }
    }
    
    /// Returns true if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self.category(), TokenCategory::Keyword)
    }
    
    /// Returns true if this token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self.category(), TokenCategory::Operator)
    }
    
    /// Returns true if this token is a literal
    pub fn is_literal(&self) -> bool {
        matches!(self.category(), TokenCategory::Literal)
    }
    
    /// Returns true if this token is a delimiter
    pub fn is_delimiter(&self) -> bool {
        matches!(self.category(), TokenCategory::Delimiter)
    }
    
    /// Returns the string representation of the token
    pub fn as_string(&self) -> String {
        match self {
            // Literals
            Token::Number(n) => n.clone(),
            Token::String(s) => s.clone(),
            Token::Identifier(i) => i.clone(),
            
            // Keywords
            Token::Let => "let".to_string(),
            Token::Func => "func".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::While => "while".to_string(),
            Token::For => "for".to_string(),
            Token::Return => "return".to_string(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::Null => "null".to_string(),
            Token::In => "in".to_string(),
            
            // Operators
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Modulo => "%".to_string(),
            Token::Power => "**".to_string(),
            Token::Assign => ":=".to_string(),
            Token::AssignConst => "::".to_string(),
            Token::Equals => "==".to_string(),
            Token::NotEquals => "!=".to_string(),
            Token::LessThan => "<".to_string(),
            Token::LessEqual => "<=".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::GreaterEqual => ">=".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
            Token::Not => "!".to_string(),
            Token::Range => "..".to_string(),
            Token::DotProduct => "@".to_string(),
            Token::Gradient => "∇".to_string(),
            Token::Partial => "∂".to_string(),
            
            // Delimiters
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Pipe => "|".to_string(),
            Token::Caret => "^".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Colon => ":".to_string(),
            Token::Dot => ".".to_string(),
            
            // Special
            Token::Newline => "\n".to_string(),
            Token::Comment => "//".to_string(),
            Token::MultiLineComment => "/*".to_string(),
            Token::Whitespace => " ".to_string(),
            
            // Error
            Token::Error => "ERROR".to_string(),
        }
    }
    
    /// Returns the precedence level for operators (0 = no precedence)
    pub fn precedence(&self) -> u8 {
        match self {
            // Logical operators (lowest precedence)
            Token::Or => 1,
            Token::And => 2,
            
            // Equality operators
            Token::Equals | Token::NotEquals => 3,
            
            // Comparison operators
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual => 4,
            
            // Range operator
            Token::Range => 5,
            
            // Addition and subtraction
            Token::Plus | Token::Minus => 6,
            
            // Multiplication, division, and modulo
            Token::Multiply | Token::Divide | Token::Modulo => 7,
            
            // Power operator (right-associative)
            Token::Power => 8,
            
            // AI/ML operators (highest precedence)
            Token::DotProduct | Token::Gradient | Token::Partial => 9,
            
            // Not an operator
            _ => 0,
        }
    }
    
    /// Returns true if this operator is left-associative
    pub fn is_left_associative(&self) -> bool {
        match self {
            // Right-associative operators
            Token::Power | Token::Assign | Token::AssignConst => false,
            
            // Left-associative operators
            Token::Or | Token::And | Token::Equals | Token::NotEquals |
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual |
            Token::Range | Token::Plus | Token::Minus | Token::Multiply |
            Token::Divide | Token::Modulo | Token::DotProduct | Token::Gradient |
            Token::Partial => true,
            
            // Not an operator
            _ => false,
        }
    }
    
    /// Returns true if this token is a binary operator
    pub fn is_binary_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Modulo |
            Token::Power | Token::Equals | Token::NotEquals | Token::LessThan | 
            Token::LessEqual | Token::GreaterThan | Token::GreaterEqual | Token::And | 
            Token::Or | Token::Range | Token::DotProduct | Token::Gradient | Token::Partial => true,
            _ => false,
        }
    }
    
    /// Returns true if this token is a unary operator
    pub fn is_unary_operator(&self) -> bool {
        match self {
            Token::Minus | Token::Not => true,
            _ => false,
        }
    }
    
    /// Returns true if this token is an assignment operator
    pub fn is_assignment_operator(&self) -> bool {
        match self {
            Token::Assign | Token::AssignConst => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "Number({})", n),
            Token::String(s) => write!(f, "String({})", s),
            Token::Identifier(i) => write!(f, "Identifier({})", i),
            Token::Comment => write!(f, "Comment"),
            Token::MultiLineComment => write!(f, "MultiLineComment"),
            Token::Newline => write!(f, "Newline"),
            Token::Error => write!(f, "Error"),
            _ => write!(f, "{}", self.as_string()),
        }
    }
}

/// The main lexer for Cortex source code.
/// 
/// Converts raw source code into a sequence of tokens with position information.
/// 
/// # Examples
/// 
/// ```
/// use cortex_rust::lexer::Lexer;
/// 
/// let source = "let x := 42";
/// let mut lexer = Lexer::new(source);
/// let tokens = lexer.tokenize().unwrap();
/// 
/// assert_eq!(tokens.len(), 4);
/// ```
pub struct Lexer<'a> {
    source: &'a str,
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given source code.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The Cortex source code to tokenize
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cortex_rust::lexer::Lexer;
    /// 
    /// let lexer = Lexer::new("let x := 42");
    /// ```
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Token::lexer(source),
        }
    }
    
    /// Tokenizes the entire source code into a vector of tokens with position information.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<TokenInfo>)` - A vector of tokens with line and column information
    /// * `Err(DetailedError)` - If a lexical error is encountered with detailed context
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cortex_rust::lexer::Lexer;
    /// 
    /// let mut lexer = Lexer::new("func add[a, b] | return[a + b] ^");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert!(tokens.len() > 0);
    /// ```
    pub fn tokenize(&mut self) -> Result<Vec<TokenInfo>, DetailedError> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.lexer.next() {
            let span = self.lexer.span();
            let token_text = &self.source[span.start..span.end];
            
            // Calculate position from span
            let (line, column) = self.calculate_position(span.start);
            let length = span.end - span.start;
            
            match token {
                Ok(token) => {
                    let token_info = TokenInfo::new(
                        token,
                        line,
                        column,
                        length,
                        token_text.to_string(),
                    );
                    
                    tokens.push(token_info);
                }
                Err(_) => {
                    // Create detailed error with source context
                    let error = CompilerError::LexicalError {
                        message: format!("Invalid character: '{}'", token_text),
                        line,
                        column,
                    };
                    
                    let source_line = self.get_source_line(line);
                    let context = ErrorContext::new(source_line)
                        .with_suggestion(format!("Remove or replace '{}'", token_text))
                        .with_help("Valid characters include letters, numbers, operators, and delimiters".to_string());
                    
                    return Err(DetailedError::new(error).with_context(context));
                }
            }
        }
        
        Ok(tokens)
    }
    
    /// Calculates line and column position from byte offset
    fn calculate_position(&self, byte_offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        let mut current_offset = 0;
        
        for ch in self.source.chars() {
            if current_offset >= byte_offset {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            
            current_offset += ch.len_utf8();
        }
        
        (line, column)
    }
    
    /// Gets the source line at the given line number for error reporting
    fn get_source_line(&self, line_num: usize) -> String {
        self.source
            .lines()
            .nth(line_num.saturating_sub(1))
            .unwrap_or("")
            .to_string()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let source = "let x := 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Let);
        assert_eq!(tokens[1].token, Token::Identifier("x".to_string()));
        assert_eq!(tokens[2].token, Token::Assign);
        assert_eq!(tokens[3].token, Token::Number("42".to_string()));
    }
    
    #[test]
    fn test_arithmetic_operators() {
        let source = "1 + 2 * 3 ** 4";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            Token::Number("1".to_string()),
            Token::Plus,
            Token::Number("2".to_string()),
            Token::Multiply,
            Token::Number("3".to_string()),
            Token::Power,
            Token::Number("4".to_string()),
        ];
        
        let actual: Vec<Token> = tokens.into_iter().map(|t| t.token).collect();
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_string_literals() {
        let source = r#""hello world""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String(r#""hello world""#.to_string()));
    }
    
    #[test]
    fn test_string_literals_escaped() {
        // Test escaped quotes - note that the lexer includes the quotes and backslashes
        let source = r#""hello \"world\"""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 1);
        // The lexer preserves the raw string including escape sequences
        assert!(matches!(&tokens[0].token, Token::String(_)));
    }
    
    #[test]
    fn test_comments() {
        let source = "// This is a comment\nlet x := 42 /* multi-line comment */";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Comments are skipped by logos, so they won't be in the token stream
        // Just verify that the lexer doesn't crash on comments
        assert!(tokens.len() > 0);
        assert!(tokens.iter().any(|t| matches!(t.token, Token::Let)));
    }
    
    #[test]
    fn test_function_syntax() {
        let source = "func add[a, b] | return[a + b] ^";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            Token::Func,
            Token::Identifier("add".to_string()),
            Token::LeftBracket,
            Token::Identifier("a".to_string()),
            Token::Comma,
            Token::Identifier("b".to_string()),
            Token::RightBracket,
            Token::Pipe,
            Token::Return,
            Token::LeftBracket,
            Token::Identifier("a".to_string()),
            Token::Plus,
            Token::Identifier("b".to_string()),
            Token::RightBracket,
            Token::Caret,
        ];
        
        let actual: Vec<Token> = tokens.into_iter().map(|t| t.token).collect();
        assert_eq!(actual, expected);
    }
    
    // ================================
    // NEW PHASE 4 TESTS
    // ================================
    
    #[test]
    fn test_token_categories() {
        assert_eq!(Token::Let.category(), TokenCategory::Keyword);
        assert_eq!(Token::Plus.category(), TokenCategory::Operator);
        assert_eq!(Token::Number("42".to_string()).category(), TokenCategory::Literal);
        assert_eq!(Token::LeftBracket.category(), TokenCategory::Delimiter);
        assert_eq!(Token::Newline.category(), TokenCategory::Special);
        assert_eq!(Token::Error.category(), TokenCategory::Error);
    }
    
    #[test]
    fn test_token_classification_methods() {
        // Keywords
        assert!(Token::Let.is_keyword());
        assert!(Token::Func.is_keyword());
        assert!(!Token::Plus.is_keyword());
        
        // Operators
        assert!(Token::Plus.is_operator());
        assert!(Token::Multiply.is_operator());
        assert!(Token::Equals.is_operator());
        assert!(!Token::Let.is_operator());
        
        // Literals
        assert!(Token::Number("42".to_string()).is_literal());
        assert!(Token::String("hello".to_string()).is_literal());
        assert!(Token::Identifier("x".to_string()).is_literal());
        assert!(!Token::Let.is_literal());
        
        // Delimiters
        assert!(Token::LeftBracket.is_delimiter());
        assert!(Token::Comma.is_delimiter());
        assert!(!Token::Plus.is_delimiter());
    }
    
    #[test]
    fn test_operator_precedence() {
        // Logical operators (lowest precedence)
        assert_eq!(Token::Or.precedence(), 1);
        assert_eq!(Token::And.precedence(), 2);
        
        // Equality operators
        assert_eq!(Token::Equals.precedence(), 3);
        assert_eq!(Token::NotEquals.precedence(), 3);
        
        // Comparison operators
        assert_eq!(Token::LessThan.precedence(), 4);
        assert_eq!(Token::GreaterEqual.precedence(), 4);
        
        // Range operator
        assert_eq!(Token::Range.precedence(), 5);
        
        // Addition and subtraction
        assert_eq!(Token::Plus.precedence(), 6);
        assert_eq!(Token::Minus.precedence(), 6);
        
        // Multiplication, division, and modulo
        assert_eq!(Token::Multiply.precedence(), 7);
        assert_eq!(Token::Divide.precedence(), 7);
        assert_eq!(Token::Modulo.precedence(), 7);
        
        // Power operator
        assert_eq!(Token::Power.precedence(), 8);
        
        // AI/ML operators (highest precedence)
        assert_eq!(Token::DotProduct.precedence(), 9);
        assert_eq!(Token::Gradient.precedence(), 9);
        assert_eq!(Token::Partial.precedence(), 9);
        
        // Non-operators
        assert_eq!(Token::Let.precedence(), 0);
        assert_eq!(Token::Number("42".to_string()).precedence(), 0);
    }
    
    #[test]
    fn test_operator_associativity() {
        // Right-associative operators
        assert!(!Token::Power.is_left_associative());
        assert!(!Token::Assign.is_left_associative());
        assert!(!Token::AssignConst.is_left_associative());
        
        // Left-associative operators
        assert!(Token::Plus.is_left_associative());
        assert!(Token::Multiply.is_left_associative());
        assert!(Token::Equals.is_left_associative());
        assert!(Token::And.is_left_associative());
        
        // Non-operators
        assert!(!Token::Let.is_left_associative());
        assert!(!Token::Number("42".to_string()).is_left_associative());
    }
    
    #[test]
    fn test_binary_unary_operators() {
        // Binary operators
        assert!(Token::Plus.is_binary_operator());
        assert!(Token::Multiply.is_binary_operator());
        assert!(Token::Equals.is_binary_operator());
        assert!(Token::And.is_binary_operator());
        
        // Unary operators
        assert!(Token::Minus.is_unary_operator());
        assert!(Token::Not.is_unary_operator());
        
        // Minus can be both unary and binary
        assert!(Token::Minus.is_binary_operator());
        
        // Non-operators
        assert!(!Token::Let.is_binary_operator());
        assert!(!Token::Let.is_unary_operator());
    }
    
    #[test]
    fn test_assignment_operators() {
        assert!(Token::Assign.is_assignment_operator());
        assert!(Token::AssignConst.is_assignment_operator());
        assert!(!Token::Plus.is_assignment_operator());
        assert!(!Token::Let.is_assignment_operator());
    }
    
    #[test]
    fn test_token_as_string() {
        assert_eq!(Token::Let.as_string(), "let");
        assert_eq!(Token::Plus.as_string(), "+");
        assert_eq!(Token::Number("42".to_string()).as_string(), "42");
        assert_eq!(Token::String("hello".to_string()).as_string(), "hello");
        assert_eq!(Token::Identifier("x".to_string()).as_string(), "x");
        assert_eq!(Token::LeftBracket.as_string(), "[");
        assert_eq!(Token::Power.as_string(), "**");
        assert_eq!(Token::DotProduct.as_string(), "@");
    }
    
    #[test]
    fn test_token_info_metadata() {
        let source = "let x := 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Test that TokenInfo includes all metadata
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[0].source_text, "let");
        
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 5); // "let " = 4 chars + 1
        assert_eq!(tokens[1].source_text, "x");
        
        // Test helper methods
        assert!(tokens[0].is_keyword());
        assert!(tokens[1].is_literal());
        assert!(tokens[2].is_operator());
        assert!(tokens[3].is_literal());
    }
    
    #[test]
    fn test_enhanced_error_reporting() {
        let source = "let x := 42$"; // Invalid character '$'
        let mut lexer = Lexer::new(source);
        let result = lexer.tokenize();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        
        // Check that we get a detailed error with context
        match &error.error {
            CompilerError::LexicalError { message, line, column } => {
                assert!(message.contains("Invalid character"));
                assert_eq!(*line, 1);
                assert_eq!(*column, 12); // Position of '$'
            }
            _ => panic!("Expected LexicalError"),
        }
        
        // Check that context is provided
        assert!(error.context.is_some());
        let context = error.context.unwrap();
        assert_eq!(context.source_line, "let x := 42$");
        assert!(context.suggestion.is_some());
        assert!(context.help.is_some());
    }
}
