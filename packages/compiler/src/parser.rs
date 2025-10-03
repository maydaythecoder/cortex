//! Cortex Parser
//! 
//! Parses Cortex source code into an Abstract Syntax Tree (AST).

use crate::lexer::{Token, TokenInfo};
use crate::ast::*;
use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, got {actual} at line {line}:{column}")]
    UnexpectedToken { expected: String, actual: String, line: usize, column: usize },
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Invalid syntax at line {line}:{column}: {message}")]
    InvalidSyntax { line: usize, column: usize, message: String },
}

/// The Cortex parser that converts tokens into an Abstract Syntax Tree (AST).
/// 
/// Implements a recursive descent parser for the Cortex language.
/// 
/// # Examples
/// 
/// ```
/// use cortex_rust::{lexer::Lexer, parser::Parser};
/// 
/// let source = "let x := 42";
/// let mut lexer = Lexer::new(source);
/// let tokens = lexer.tokenize().unwrap();
/// 
/// let mut parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
/// 
/// assert_eq!(ast.statements.len(), 1);
/// ```
pub struct Parser {
    tokens: Vec<TokenInfo>,
    position: usize,
}

impl Parser {
    /// Creates a new parser with the given token stream.
    /// 
    /// # Arguments
    /// 
    /// * `tokens` - A vector of tokens with position information from the lexer
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cortex_rust::{lexer::Lexer, parser::Parser};
    /// 
    /// let mut lexer = Lexer::new("let x := 42");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// ```
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, position: 0 }
    }
    
    /// Returns the binding power (precedence and associativity) for binary operators.
    /// 
    /// Returns a tuple of (left_binding_power, right_binding_power).
    /// For left-associative operators: (n, n+1)
    /// For right-associative operators: (n+1, n)
    /// 
    /// Higher values = higher precedence
    /// 
    /// Reference: Pratt parsing / precedence climbing algorithm
    fn binding_power(op: &Token) -> Option<(u8, u8)> {
        match op {
            // Logical OR - lowest precedence
            Token::Or => Some((1, 2)),
            
            // Logical AND
            Token::And => Some((3, 4)),
            
            // Equality operators
            Token::Equals | Token::NotEquals => Some((5, 6)),
            
            // Comparison operators
            Token::LessThan | Token::LessEqual | 
            Token::GreaterThan | Token::GreaterEqual => Some((7, 8)),
            
            // Range operator
            Token::Range => Some((9, 10)),
            
            // Addition and subtraction
            Token::Plus | Token::Minus => Some((11, 12)),
            
            // Multiplication, division, and modulo
            Token::Multiply | Token::Divide | Token::Modulo => Some((13, 14)),
            
            // Power operator (right-associative)
            Token::Power => Some((16, 15)),
            
            // Not a binary operator
            _ => None,
        }
    }
    
    /// Returns the operator string for a token
    fn token_to_operator(token: &Token) -> Option<String> {
        match token {
            Token::Or => Some("||".to_string()),
            Token::And => Some("&&".to_string()),
            Token::Equals => Some("==".to_string()),
            Token::NotEquals => Some("!=".to_string()),
            Token::LessThan => Some("<".to_string()),
            Token::LessEqual => Some("<=".to_string()),
            Token::GreaterThan => Some(">".to_string()),
            Token::GreaterEqual => Some(">=".to_string()),
            Token::Range => Some("..".to_string()),
            Token::Plus => Some("+".to_string()),
            Token::Minus => Some("-".to_string()),
            Token::Multiply => Some("*".to_string()),
            Token::Divide => Some("/".to_string()),
            Token::Modulo => Some("%".to_string()),
            Token::Power => Some("**".to_string()),
            _ => None,
        }
    }
    
    /// Parses the token stream into an Abstract Syntax Tree (AST).
    /// 
    /// # Returns
    /// 
    /// * `Ok(Program)` - The root AST node containing all parsed statements
    /// * `Err(anyhow::Error)` - If a syntax error is encountered
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cortex_rust::{lexer::Lexer, parser::Parser};
    /// 
    /// let source = r#"
    ///     func add[a, b] |
    ///         return[a + b]
    ///     ^
    /// "#;
    /// 
    /// let mut lexer = Lexer::new(source);
    /// let tokens = lexer.tokenize().unwrap();
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse().unwrap();
    /// ```
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }
        
        Ok(Program::new(statements))
    }
    
    fn current_token(&self) -> Option<&TokenInfo> {
        self.tokens.get(self.position)
    }
    
    
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() ||
        matches!(self.current_token().map(|t| &t.token), Some(Token::Error))
    }
    
    fn advance(&mut self) -> Option<&TokenInfo> {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.current_token()
    }
    
    fn expect(&mut self, expected: Token) -> Result<TokenInfo> {
        let current = self.current_token()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        if current.token == expected {
            let token = current.clone();
            self.advance();
            Ok(token)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                actual: format!("{:?}", current.token),
                line: current.line,
                column: current.column,
            }.into())
        }
    }
    
    fn match_token(&mut self, token: Token) -> bool {
        if let Some(current) = self.current_token() {
            if current.token == token {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        // Skip comments and newlines
        while let Some(current) = self.current_token() {
            match &current.token {
                Token::Comment | Token::MultiLineComment | Token::Newline => {
                    self.advance();
                }
                _ => break,
            }
        }
        
        if self.is_at_end() {
            return Ok(None);
        }
        
        let statement = match self.current_token().map(|t| &t.token) {
            Some(Token::Let) => self.parse_assignment()?,
            Some(Token::Func) => self.parse_function()?,
            Some(Token::If) => self.parse_if_statement()?,
            Some(Token::While) => self.parse_while_loop()?,
            Some(Token::For) => self.parse_for_loop()?,
            Some(Token::Return) => self.parse_return_statement()?,
            Some(Token::Caret) => {
                // End of block
                return Ok(None);
            }
            Some(Token::Identifier(_)) => {
                // Check if this is a reassignment statement first
                if let Some(next_token) = self.current_token() {
                    if next_token.token == Token::Assign {
                        // This is a reassignment - parse it as such
                        let variable = match self.current_token() {
                            Some(token) => match &token.token {
                                Token::Identifier(name) => {
                                    let name = name.clone();
                                    self.advance();
                                    name
                                }
                                _ => return Err(ParseError::UnexpectedToken {
                                    expected: "variable name".to_string(),
                                    actual: format!("{:?}", token.token),
                                    line: token.line,
                                    column: token.column,
                                }.into()),
                            }
                            None => return Err(ParseError::UnexpectedEof.into()),
                        };
                        
                        self.expect(Token::Assign)?;
                        let value = self.parse_expression()?;
                        Statement::Assignment(Assignment::new(variable, value))
                    } else {
                        // Try to parse as expression (could be IndexExpression)
                        let expr = self.parse_expression()?;
                        
                        // Check if there's an assignment after the expression
                        let is_assignment = if let Some(next_token) = self.current_token() {
                            next_token.token == Token::Assign
                        } else {
                            false
                        };
                        
                        if is_assignment {
                            self.advance(); // Consume the := token
                            let value = self.parse_expression()?;
                            
                            // Create appropriate assignment based on left-hand side
                            match expr {
                                Expression::Index(index_expr) => {
                                    Statement::IndexAssignment(IndexAssignment::new(*index_expr.container.clone(), *index_expr.index.clone(), value))
                                }
                                Expression::Identifier(identifier) => {
                                    // This is actually a regular assignment, not an index assignment
                                    Statement::Assignment(Assignment::new(identifier.name, value))
                                }
                                _ => return Err(ParseError::UnexpectedToken {
                                    expected: "array or dictionary index".to_string(),
                                    actual: format!("{:?}", expr),
                                    line: 0, // We can't get line info here due to borrowing
                                    column: 0,
                                }.into())
                            }
                        } else {
                            Statement::Expression(expr)
                        }
                    }
                } else {
                    // Try to parse as expression statement
                    let expr = self.parse_expression()?;
                    Statement::Expression(expr)
                }
            }
            _ => {
                // Try to parse as expression statement
                let expr = self.parse_expression()?;
                Statement::Expression(expr)
            }
        };
        
        Ok(Some(statement))
    }
    
    fn parse_assignment(&mut self) -> Result<Statement> {
        self.expect(Token::Let)?;
        
        let var_name = match self.current_token() {
            Some(token) => match &token.token {
                Token::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                _ => return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    actual: format!("{:?}", token.token),
                    line: token.line,
                    column: token.column,
                }.into()),
            }
            None => return Err(ParseError::UnexpectedEof.into()),
        };
        
        // Check for type annotation
        let _type_annotation = if self.match_token(Token::Colon) {
            match self.current_token() {
                Some(token) => match &token.token {
                    Token::Identifier(type_name) => {
                        let type_name = type_name.clone();
                        self.advance();
                        Some(TypeAnnotation::new(type_name))
                    }
                    _ => return Err(ParseError::UnexpectedToken {
                        expected: "type name".to_string(),
                        actual: format!("{:?}", token.token),
                        line: token.line,
                        column: token.column,
                    }.into()),
                }
                None => return Err(ParseError::UnexpectedEof.into()),
            }
        } else {
            None
        };
        
        // Check for assignment type
        if self.match_token(Token::AssignConst) {
            let value = self.parse_expression()?;
            Ok(Statement::ConstantAssignment(ConstantAssignment::new(var_name, value)))
        } else if self.match_token(Token::Assign) {
            let value = self.parse_expression()?;
            Ok(Statement::Assignment(Assignment::new(var_name, value)))
        } else {
            let current = self.current_token().unwrap();
            Err(ParseError::UnexpectedToken {
                expected: "assignment operator".to_string(),
                actual: format!("{:?}", current.token),
                line: current.line,
                column: current.column,
            }.into())
        }
    }
    
    
    fn parse_function(&mut self) -> Result<Statement> {
        self.expect(Token::Func)?;
        
        let name = match self.current_token() {
            Some(token) => match &token.token {
                Token::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                _ => return Err(ParseError::UnexpectedToken {
                    expected: "function name".to_string(),
                    actual: format!("{:?}", token.token),
                    line: token.line,
                    column: token.column,
                }.into()),
            }
            None => return Err(ParseError::UnexpectedEof.into()),
        };
        
        // Parse parameters
        self.expect(Token::LeftBracket)?;
        let mut parameters = Vec::new();
        
        if !self.match_token(Token::RightBracket) {
            loop {
                let param_name = match self.current_token() {
                    Some(token) => match &token.token {
                        Token::Identifier(name) => {
                            let name = name.clone();
                            self.advance();
                            name
                        }
                        _ => return Err(ParseError::UnexpectedToken {
                            expected: "parameter name".to_string(),
                            actual: format!("{:?}", token.token),
                            line: token.line,
                            column: token.column,
                        }.into()),
                    }
                    None => return Err(ParseError::UnexpectedEof.into()),
                };
                
                // Check for type annotation
                let type_annotation = if self.match_token(Token::Colon) {
                    match self.current_token() {
                        Some(token) => match &token.token {
                            Token::Identifier(type_name) => {
                                let type_name = type_name.clone();
                                self.advance();
                                Some(TypeAnnotation::new(type_name))
                            }
                            _ => return Err(ParseError::UnexpectedToken {
                                expected: "type name".to_string(),
                                actual: format!("{:?}", token.token),
                                line: token.line,
                                column: token.column,
                            }.into()),
                        }
                        None => return Err(ParseError::UnexpectedEof.into()),
                    }
                } else {
                    None
                };
                
                parameters.push(Variable::new(param_name, type_annotation));
                
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
            
            self.expect(Token::RightBracket)?;
        }
        
        // Parse return type (optional)
        let return_type = if self.match_token(Token::Colon) {
            match self.current_token() {
                Some(token) => match &token.token {
                    Token::Identifier(type_name) => {
                        let type_name = type_name.clone();
                        self.advance();
                        Some(type_name)
                    }
                    _ => return Err(ParseError::UnexpectedToken {
                        expected: "return type".to_string(),
                        actual: format!("{:?}", token.token),
                        line: token.line,
                        column: token.column,
                    }.into()),
                }
                None => return Err(ParseError::UnexpectedEof.into()),
            }
        } else {
            None
        };
        
        // Parse body
        let body = self.parse_block()?;
        
        Ok(Statement::Function(Function::new(name, parameters, return_type, body)))
    }
    
    fn parse_block(&mut self) -> Result<Block> {
        self.expect(Token::Pipe)?;
        
        let mut statements = Vec::new();
        while !self.match_token(Token::Caret) {
            // Skip comments and newlines
            while let Some(current) = self.current_token() {
                match &current.token {
                    Token::Comment | Token::MultiLineComment | Token::Newline => {
                        self.advance();
                    }
                    _ => break,
                }
            }
            
            if self.is_at_end() {
                break;
            }
            
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }
        
        Ok(Block::new(statements))
    }
    
    fn parse_if_statement(&mut self) -> Result<Statement> {
        self.expect(Token::If)?;
        
        self.expect(Token::LeftBracket)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RightBracket)?;
        
        let then_block = self.parse_block()?;
        
        // Parse else block (optional) - handle else if chains
        let else_block = if self.match_token(Token::Else) {
            // Check if this is an "else if" by looking ahead
            if let Some(next_token) = self.current_token() {
                if next_token.token == Token::If {
                    // This is an "else if", parse it manually
                    self.advance(); // Consume the 'if' token
                    
                    self.expect(Token::LeftBracket)?;
                    let else_condition = self.parse_expression()?;
                    self.expect(Token::RightBracket)?;
                    
                    let else_then_block = self.parse_block()?;
                    
                    // Recursively parse else block for the else-if
                    let else_else_block = if self.match_token(Token::Else) {
                        // Check if this is another "else if"
                        if let Some(next_token) = self.current_token() {
                            if next_token.token == Token::If {
                                self.advance(); // Consume the 'if' token
                                let nested_else_if_statement = self.parse_if_statement()?;
                                Some(Block::new(vec![nested_else_if_statement]))
                            } else {
                                Some(self.parse_block()?)
                            }
                        } else {
                            Some(self.parse_block()?)
                        }
                    } else {
                        None
                    };
                    
                    let else_if_statement = Statement::IfStatement(IfStatement::new(else_condition, else_then_block, else_else_block));
                    Some(Block::new(vec![else_if_statement]))
                } else {
                    // This is a regular "else" block
                    Some(self.parse_block()?)
                }
            } else {
                // This is a regular "else" block
                Some(self.parse_block()?)
            }
        } else {
            None
        };
        
        Ok(Statement::IfStatement(IfStatement::new(condition, then_block, else_block)))
    }
    
    fn parse_while_loop(&mut self) -> Result<Statement> {
        self.expect(Token::While)?;
        
        self.expect(Token::LeftBracket)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RightBracket)?;
        
        let body = self.parse_block()?;
        
        Ok(Statement::WhileLoop(WhileLoop::new(condition, body)))
    }
    
    fn parse_for_loop(&mut self) -> Result<Statement> {
        self.expect(Token::For)?;
        
        // Parse variable name
        let variable = match self.current_token() {
            Some(token) => match &token.token {
                Token::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                _ => return Err(ParseError::UnexpectedToken {
                    expected: "variable name".to_string(),
                    actual: format!("{:?}", token.token),
                    line: token.line,
                    column: token.column,
                }.into()),
            }
            None => return Err(ParseError::UnexpectedEof.into()),
        };
        
        // Check if this is a modern for loop with 'in' keyword
        let iterable = if self.match_token(Token::In) {
            // Parse the iterable expression
            Some(self.parse_expression()?)
        } else {
            // Legacy for loop - check if there's a bracket
            if self.match_token(Token::LeftBracket) {
                self.expect(Token::RightBracket)?;
            }
            None
        };
        
        let body = self.parse_block()?;
        
        Ok(Statement::ForLoop(ForLoop::new(variable, iterable, body)))
    }
    
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.expect(Token::Return)?;
        
        // Check if there's a return value in brackets
        let value = if self.match_token(Token::LeftBracket) {
            if self.match_token(Token::RightBracket) {
                // return[] - no value
                None
            } else {
                // return[value] - parse the value
                let expr = self.parse_expression()?;
                self.expect(Token::RightBracket)?;
                Some(expr)
            }
        } else if !self.match_token(Token::Semicolon) {
            // return value - parse without brackets
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Statement::ReturnStatement(ReturnStatement::new(value)))
    }
    
    /// Parses an expression using Pratt parsing (precedence climbing).
    /// 
    /// This implementation correctly handles operator precedence and associativity,
    /// including right-associative operators like power (**).
    /// 
    /// # Arguments
    /// 
    /// * `min_bp` - Minimum binding power (precedence level) for this parse
    /// 
    /// # Reference
    /// 
    /// Based on Pratt parsing algorithm from:
    /// - https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_expression_bp(0)
    }
    
    fn parse_expression_bp(&mut self, min_bp: u8) -> Result<Expression> {
        // Parse prefix/unary operators and atoms
        let mut left = self.parse_prefix()?;
        
        // Parse postfix operators (function calls, array indexing)
        left = self.parse_postfix_ops(left)?;
        
        // Parse binary operators with precedence climbing
        loop {
            let current = match self.current_token() {
                Some(tok) => tok,
                None => break,
            };
            
            // Check if current token is a binary operator
            let (l_bp, r_bp) = match Self::binding_power(&current.token) {
                Some(bp) => bp,
                None => break,
            };
            
            // If the binding power is less than minimum, stop
            if l_bp < min_bp {
                break;
            }
            
            // Get the operator string
            let operator = Self::token_to_operator(&current.token)
                .expect("Token should have operator representation");
            
            // Special handling for range operator
            if current.token == Token::Range {
                self.advance();
                let right = self.parse_expression_bp(r_bp)?;
                left = Expression::Range(RangeExpression::new(left, right));
                continue;
            }
            
            // Consume the operator
            self.advance();
            
            // Parse the right-hand side with appropriate binding power
            let right = self.parse_expression_bp(r_bp)?;
            
            // Create binary operation
            left = Expression::BinaryOp(BinaryOp::new(left, operator, right));
        }
        
        Ok(left)
    }
    
    /// Parses prefix operators (unary operators) and atoms
    fn parse_prefix(&mut self) -> Result<Expression> {
        let current = self.current_token()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        match &current.token {
            Token::Not => {
                self.advance();
                let operand = self.parse_prefix()?;
                Ok(Expression::UnaryOp(UnaryOp::new("!".to_string(), operand)))
            }
            Token::Minus => {
                self.advance();
                let operand = self.parse_prefix()?;
                Ok(Expression::UnaryOp(UnaryOp::new("-".to_string(), operand)))
            }
            Token::Plus => {
                self.advance();
                let operand = self.parse_prefix()?;
                Ok(Expression::UnaryOp(UnaryOp::new("+".to_string(), operand)))
            }
            _ => self.parse_atom(),
        }
    }
    
    /// Parses atomic expressions (literals, identifiers, parenthesized expressions)
    fn parse_atom(&mut self) -> Result<Expression> {
        let current = self.current_token()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        match &current.token {
            Token::Number(n) => {
                let value = n.parse::<f64>()
                    .map_err(|_| ParseError::InvalidSyntax {
                        line: current.line,
                        column: current.column,
                        message: format!("Invalid number: {}", n),
                    })?;
                self.advance();
                Ok(Expression::Literal(Literal::new(LiteralValue::Number(value), "number".to_string())))
            }
            
            Token::String(s) => {
                let value = s.trim_matches('"').to_string();
                self.advance();
                Ok(Expression::Literal(Literal::new(LiteralValue::String(value), "string".to_string())))
            }
            
            Token::True => {
                self.advance();
                Ok(Expression::Literal(Literal::new(LiteralValue::Boolean(true), "boolean".to_string())))
            }
            
            Token::False => {
                self.advance();
                Ok(Expression::Literal(Literal::new(LiteralValue::Boolean(false), "boolean".to_string())))
            }
            
            Token::Null => {
                self.advance();
                Ok(Expression::Literal(Literal::new(LiteralValue::Null, "null".to_string())))
            }
            
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(Identifier::new(name)))
            }
            
            Token::LeftBracket => {
                self.parse_array()
            }
            
            Token::LeftBrace => {
                self.parse_dictionary()
            }
            
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                actual: format!("{:?}", current.token),
                line: current.line,
                column: current.column,
            }.into())
        }
    }
    
    /// Parses array literals
    fn parse_array(&mut self) -> Result<Expression> {
        self.advance(); // Consume '['
        
        if self.match_token(Token::RightBracket) {
            // Empty array
            Ok(Expression::Array(Array::new(vec![])))
        } else {
            // Parse array elements
            let mut elements = Vec::new();
            
            loop {
                let element = self.parse_expression()?;
                elements.push(element);
                
                if self.match_token(Token::RightBracket) {
                    break;
                }
                
                if !self.match_token(Token::Comma) {
                    return Err(ParseError::UnexpectedToken {
                        expected: "comma or closing bracket".to_string(),
                        actual: format!("{:?}", self.current_token().map(|t| &t.token).unwrap_or(&Token::Error)),
                        line: self.current_token().map(|t| t.line).unwrap_or(0),
                        column: self.current_token().map(|t| t.column).unwrap_or(0),
                    }.into());
                }
            }
            
            Ok(Expression::Array(Array::new(elements)))
        }
    }
    
    /// Parses postfix operators (function calls, array indexing)
    fn parse_postfix_ops(&mut self, mut left: Expression) -> Result<Expression> {
        // This method handles postfix operations like function calls and indexing
        loop {
            let current = match self.current_token() {
                Some(tok) => tok,
                None => break,
            };
            
            match &current.token {
                Token::LeftBracket => {
                    self.advance(); // Consume '['
                    
                    // Check if this is an empty function call
                    if self.match_token(Token::RightBracket) {
                        left = Expression::Index(IndexExpression::new(
                            left,
                            Expression::Literal(Literal::new(LiteralValue::Null, "null".to_string()))
                        ));
                    } else {
                        // Parse the arguments (could be single argument or multiple)
                        let mut arguments = Vec::new();
                        
                        loop {
                            let arg = self.parse_expression()?;
                            arguments.push(arg);
                            
                            if self.match_token(Token::RightBracket) {
                                break;
                            }
                            
                            if !self.match_token(Token::Comma) {
                                return Err(ParseError::UnexpectedToken {
                                    expected: "comma or closing bracket".to_string(),
                                    actual: format!("{:?}", self.current_token().map(|t| &t.token).unwrap_or(&Token::Error)),
                                    line: self.current_token().map(|t| t.line).unwrap_or(0),
                                    column: self.current_token().map(|t| t.column).unwrap_or(0),
                                }.into());
                            }
                        }
                        
                        // Handle function call arguments properly
                        let index = if arguments.len() == 1 {
                            arguments[0].clone()
        } else {
                            Expression::Array(Array::new(arguments))
                        };
                        
                        left = Expression::Index(IndexExpression::new(left, index));
                    }
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    
    // fn parse_call(&mut self, function_name: String) -> Result<Expression> {
    //     let mut arguments = Vec::new();
    //     
    //     if !self.match_token(Token::RightBracket) {
    //         loop {
    //             let arg = self.parse_expression()?;
    //             arguments.push(arg);
    //             
    //             if !self.match_token(Token::Comma) {
    //                 break;
    //             }
    //         }
    //         
    //         self.expect(Token::RightBracket)?;
    //     }
    //     
    //     Ok(Expression::Call(Call::new(
    //         Expression::Identifier(Identifier::new(function_name)),
    //         arguments,
    //     )))
    // }
    
    
    fn parse_dictionary(&mut self) -> Result<Expression> {
        self.expect(Token::LeftBrace)?;
        
        let mut pairs = Vec::new();
        if !self.match_token(Token::RightBrace) {
            loop {
                let key = self.parse_expression()?;
                self.expect(Token::Colon)?;
                let value = self.parse_expression()?;
                pairs.push((key, value));
                
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
            
            self.expect(Token::RightBrace)?;
        }
        
        Ok(Expression::Dictionary(Dictionary::new(pairs)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    
    #[test]
    fn test_parse_simple_assignment() {
        let source = "let x := 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
        if let Statement::Assignment(assignment) = &ast.statements[0] {
            assert_eq!(assignment.variable, "x");
        } else {
            panic!("Expected assignment statement");
        }
    }
    
    #[test]
    fn test_parse_function() {
        let source = "func add[a, b] | return[a + b] ^";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
        if let Statement::Function(function) = &ast.statements[0] {
            assert_eq!(function.name, "add");
            assert_eq!(function.parameters.len(), 2);
        } else {
            panic!("Expected function statement");
        }
    }
    
    #[test]
    fn test_parse_arithmetic() {
        let source = "1 + 2 * 3";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
        if let Statement::Expression(expr) = &ast.statements[0] {
            if let Expression::BinaryOp(binary_op) = expr {
                assert_eq!(binary_op.operator, "+");
            } else {
                panic!("Expected binary operation");
            }
        } else {
            panic!("Expected expression statement");
        }
    }
    
    #[test]
    fn test_operator_precedence() {
        // Test that multiplication has higher precedence than addition
        // 1 + 2 * 3 should parse as 1 + (2 * 3), not (1 + 2) * 3
        let source = "1 + 2 * 3";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Statement::Expression(Expression::BinaryOp(add_op)) = &ast.statements[0] {
            assert_eq!(add_op.operator, "+");
            // Right side should be multiplication
            if let Expression::BinaryOp(mul_op) = &*add_op.right {
                assert_eq!(mul_op.operator, "*");
            } else {
                panic!("Expected multiplication on right side");
            }
        } else {
            panic!("Expected addition expression");
        }
    }
    
    #[test]
    fn test_power_right_associativity() {
        // Test that power operator is right-associative
        // 2 ** 3 ** 2 should parse as 2 ** (3 ** 2), not (2 ** 3) ** 2
        let source = "2 ** 3 ** 2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Statement::Expression(Expression::BinaryOp(power_op)) = &ast.statements[0] {
            assert_eq!(power_op.operator, "**");
            // Right side should also be power
            if let Expression::BinaryOp(inner_power) = &*power_op.right {
                assert_eq!(inner_power.operator, "**");
            } else {
                panic!("Expected power operation on right side for right-associativity");
            }
        } else {
            panic!("Expected power expression");
        }
    }
    
    #[test]
    fn test_complex_precedence() {
        // Test complex expression: 2 + 3 * 4 ** 2 - 1
        // Should parse as: 2 + (3 * (4 ** 2)) - 1
        let source = "2 + 3 * 4 ** 2 - 1";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        // The root should be subtraction (lowest precedence in this expression)
        if let Statement::Expression(Expression::BinaryOp(sub_op)) = &ast.statements[0] {
            assert_eq!(sub_op.operator, "-");
            
            // Left side should be addition
            if let Expression::BinaryOp(add_op) = &*sub_op.left {
                assert_eq!(add_op.operator, "+");
                
                // Right side of addition should be multiplication
                if let Expression::BinaryOp(mul_op) = &*add_op.right {
                    assert_eq!(mul_op.operator, "*");
                    
                    // Right side of multiplication should be power
                    if let Expression::BinaryOp(power_op) = &*mul_op.right {
                        assert_eq!(power_op.operator, "**");
                    } else {
                        panic!("Expected power operation");
                    }
                } else {
                    panic!("Expected multiplication");
                }
            } else {
                panic!("Expected addition");
            }
        } else {
            panic!("Expected subtraction expression");
        }
    }
    
    #[test]
    fn test_logical_operators_precedence() {
        // Test that && has higher precedence than ||
        // true || false && true should parse as true || (false && true)
        let source = "true || false && true";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Statement::Expression(Expression::BinaryOp(or_op)) = &ast.statements[0] {
            assert_eq!(or_op.operator, "||");
            
            // Right side should be AND
            if let Expression::BinaryOp(and_op) = &*or_op.right {
                assert_eq!(and_op.operator, "&&");
            } else {
                panic!("Expected AND operation on right side");
            }
        } else {
            panic!("Expected OR expression");
        }
    }
    
    #[test]
    fn test_comparison_operators() {
        // Test comparison operators
        let source = "1 < 2 && 3 > 2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Statement::Expression(Expression::BinaryOp(and_op)) = &ast.statements[0] {
            assert_eq!(and_op.operator, "&&");
            
            // Both sides should be comparison operators
            if let Expression::BinaryOp(left_cmp) = &*and_op.left {
                assert_eq!(left_cmp.operator, "<");
            } else {
                panic!("Expected less-than comparison");
            }
            
            if let Expression::BinaryOp(right_cmp) = &*and_op.right {
                assert_eq!(right_cmp.operator, ">");
            } else {
                panic!("Expected greater-than comparison");
            }
        } else {
            panic!("Expected AND expression");
        }
    }
}
