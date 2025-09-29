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

pub struct Parser {
    tokens: Vec<TokenInfo>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, position: 0 }
    }
    
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
    
    fn peek_token(&self, offset: usize) -> Option<&TokenInfo> {
        self.tokens.get(self.position + offset)
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
        
        // Parse else block (optional)
        let else_block = if self.match_token(Token::Else) {
            Some(self.parse_block()?)
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
        
        self.expect(Token::LeftBracket)?;
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
        self.expect(Token::RightBracket)?;
        
        let body = self.parse_block()?;
        
        Ok(Statement::ForLoop(ForLoop::new(variable, None, body)))
    }
    
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.expect(Token::Return)?;
        
        let value = if !self.match_token(Token::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Statement::ReturnStatement(ReturnStatement::new(value)))
    }
    
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and()?;
        
        while self.match_token(Token::Or) {
            let right = self.parse_logical_and()?;
            left = Expression::BinaryOp(BinaryOp::new(left, "|".to_string(), right));
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;
        
        while self.match_token(Token::And) {
            let right = self.parse_equality()?;
            left = Expression::BinaryOp(BinaryOp::new(left, "&&".to_string(), right));
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;
        
        while self.match_token(Token::Equals) | self.match_token(Token::NotEquals) {
            let operator = match self.tokens.get(self.position - 1) {
                Some(token) => match &token.token {
                    Token::Equals => "==".to_string(),
                    Token::NotEquals => "!=".to_string(),
                    _ => unreachable!(),
                }
                None => unreachable!(),
            };
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp(BinaryOp::new(left, operator, right));
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_term()?;
        
        while self.match_token(Token::LessThan) | 
              self.match_token(Token::LessEqual) | 
              self.match_token(Token::GreaterThan) | 
              self.match_token(Token::GreaterEqual) {
            let operator = match self.tokens.get(self.position - 1) {
                Some(token) => match &token.token {
                    Token::LessThan => "<".to_string(),
                    Token::LessEqual => "<=".to_string(),
                    Token::GreaterThan => ">".to_string(),
                    Token::GreaterEqual => ">=".to_string(),
                    _ => unreachable!(),
                }
                None => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expression::BinaryOp(BinaryOp::new(left, operator, right));
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Expression> {
        let mut left = self.parse_factor()?;
        
        while self.match_token(Token::Plus) | self.match_token(Token::Minus) {
            let operator = match self.tokens.get(self.position - 1) {
                Some(token) => match &token.token {
                    Token::Plus => "+".to_string(),
                    Token::Minus => "-".to_string(),
                    _ => unreachable!(),
                }
                None => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expression::BinaryOp(BinaryOp::new(left, operator, right));
        }
        
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;
        
        while self.match_token(Token::Multiply) | 
              self.match_token(Token::Divide) | 
              self.match_token(Token::Modulo) | 
              self.match_token(Token::Power) {
            let operator = match self.tokens.get(self.position - 1) {
                Some(token) => match &token.token {
                    Token::Multiply => "*".to_string(),
                    Token::Divide => "/".to_string(),
                    Token::Modulo => "%".to_string(),
                    Token::Power => "**".to_string(),
                    _ => unreachable!(),
                }
                None => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expression::BinaryOp(BinaryOp::new(left, operator, right));
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expression> {
        if self.match_token(Token::Not) {
            let operand = self.parse_unary()?;
            Ok(Expression::UnaryOp(UnaryOp::new("!".to_string(), operand)))
        } else {
            self.parse_primary()
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expression> {
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
                
                // Check if it's a function call first
                if self.peek_token(0).map(|t| &t.token) == Some(&Token::LeftBracket) {
                    // Consume the LeftBracket token since parse_call expects it
                    self.advance(); // Consume the '['
                    self.parse_call_argless(name)
                } else {
                    Ok(Expression::Identifier(Identifier::new(name)))
                }
            }
            
            Token::LeftBracket => {
                self.advance();
                
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
            
            Token::LeftBrace => {
                self.parse_dictionary()
            }
            
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                actual: format!("{:?}", current.token),
                line: current.line,
                column: current.column,
            }.into())
        }
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
    
    fn parse_call_argless(&mut self, function_name: String) -> Result<Expression> {
        let mut arguments = Vec::new();
        
        if !self.match_token(Token::RightBracket) {
            loop {
                let arg = self.parse_expression()?;
                arguments.push(arg);
                
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
            
            self.expect(Token::RightBracket)?;
        }
        
        Ok(Expression::Call(Call::new(
            Expression::Identifier(Identifier::new(function_name)),
            arguments,
        )))
    }
    
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
}
