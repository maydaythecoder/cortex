//! Cortex Lexer
//! 
//! Tokenizes Cortex source code into a stream of tokens using the logos crate.

use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?", |lex| lex.slice().to_string())]
    Number(String),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    String(String),
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    // Keywords
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
    
    // Keywords for iteration
    #[token("in")]
    In,
    
    // Range operator
    #[token("..")]
    Range,
    
    // Operators
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
    
    #[token(":=")]
    Assign,
    
    #[token("::")]
    AssignConst,
    
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
    
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    #[token("!")]
    Not,
    
    // AI/ML specific operators
    #[token("@")]
    DotProduct,
    
    #[token("∇")]
    Gradient,
    
    #[token("∂")]
    Partial,
    
    // Delimiters
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
    
    // Special
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

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub length: usize,
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
            _ => write!(f, "{:?}", self),
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
    current_line: usize,
    current_column: usize,
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
            current_line: 1,
            current_column: 1,
        }
    }
    
    /// Tokenizes the entire source code into a vector of tokens with position information.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<TokenInfo>)` - A vector of tokens with line and column information
    /// * `Err(())` - If a lexical error is encountered
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
    pub fn tokenize(&mut self) -> Result<Vec<TokenInfo>, ()> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.lexer.next() {
            let span = self.lexer.span();
            let line = self.current_line;
            let column = self.current_column;
            let length = span.end - span.start;
            
            // Update position tracking
            let token_text = &self.source[span.start..span.end];
            self.update_position(token_text);
            
            let token_info = TokenInfo {
                token: token?,
                line,
                column,
                length,
            };
            
            tokens.push(token_info);
        }
        
        Ok(tokens)
    }
    
    fn update_position(&mut self, text: &str) {
        for ch in text.chars() {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
        }
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
}
