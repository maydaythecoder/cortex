"""
Cortex Parser

Parses Cortex source code into an Abstract Syntax Tree (AST).
"""

from typing import List, Optional, Any
from .lexer import Lexer, Token, TokenType
from ..ast.nodes import *


class ParseError(Exception):
    """Exception raised during parsing."""
    pass


class Parser:
    """Parser for Cortex language."""
    
    def __init__(self, source: str):
        self.lexer = Lexer(source)
        self.tokens = self.lexer.tokenize()
        self.position = 0
        self.current_token = self.tokens[0] if self.tokens else None
    
    def parse(self) -> Program:
        """Parse the source code into an AST."""
        statements = []
        
        while self.current_token and self.current_token.type != TokenType.EOF:
            statement = self._parse_statement()
            if statement:
                statements.append(statement)
        
        return Program(statements)
    
    def _advance(self):
        """Move to the next token."""
        self.position += 1
        if self.position < len(self.tokens):
            self.current_token = self.tokens[self.position]
        else:
            self.current_token = None
    
    def _peek(self, offset: int = 1) -> Optional[Token]:
        """Peek at a future token without advancing."""
        peek_pos = self.position + offset
        if peek_pos < len(self.tokens):
            return self.tokens[peek_pos]
        return None
    
    def _expect(self, token_type: TokenType) -> Token:
        """Expect a specific token type and advance."""
        if not self.current_token or self.current_token.type != token_type:
            expected = token_type.value
            actual = self.current_token.type.value if self.current_token else "EOF"
            raise ParseError(f"Expected {expected}, got {actual} at {self.current_token.line}:{self.current_token.column}")
        
        token = self.current_token
        self._advance()
        return token
    
    def _parse_statement(self) -> Optional[Statement]:
        """Parse a statement."""
        if not self.current_token:
            return None
        
        # Skip comments
        while self.current_token and self.current_token.type == TokenType.COMMENT:
            self._advance()
        
        if not self.current_token:
            return None
        
        if self.current_token.type == TokenType.LET:
            return self._parse_assignment()
        elif self.current_token.type == TokenType.FUNC:
            return self._parse_function()
        elif self.current_token.type == TokenType.IF:
            return self._parse_if_statement()
        elif self.current_token.type == TokenType.WHILE:
            return self._parse_while_loop()
        elif self.current_token.type == TokenType.FOR:
            return self._parse_for_loop()
        elif self.current_token.type == TokenType.RETURN:
            return self._parse_return_statement()
        else:
            # Try to parse as expression statement
            expr = self._parse_expression()
            if expr and self.current_token and self.current_token.type == TokenType.SEMICOLON:
                self._advance()
            return expr
    
    def _parse_assignment(self) -> Statement:
        """Parse variable assignment."""
        self._expect(TokenType.LET)
        
        var_name = self._expect(TokenType.IDENTIFIER).value
        
        # Check for type annotation
        type_annotation = None
        if self.current_token and self.current_token.type == TokenType.COLON:
            self._advance()
            type_name = self._expect(TokenType.IDENTIFIER).value
            type_annotation = TypeAnnotation(type_name)
        
        # Check for assignment type
        if self.current_token and self.current_token.type == TokenType.ASSIGN_CONST:
            self._advance()
            value = self._parse_expression()
            return ConstantAssignment(var_name, value)
        elif self.current_token and self.current_token.type == TokenType.ASSIGN:
            self._advance()
            value = self._parse_expression()
            return Assignment(var_name, value)
        else:
            raise ParseError(f"Expected assignment operator, got {self.current_token.type.value if self.current_token else 'EOF'}")
    
    def _parse_function(self) -> Function:
        """Parse function definition."""
        self._expect(TokenType.FUNC)
        
        name = self._expect(TokenType.IDENTIFIER).value
        
        # Parse parameters
        self._expect(TokenType.LEFT_BRACKET)
        parameters = []
        
        if self.current_token and self.current_token.type != TokenType.RIGHT_BRACKET:
            while True:
                param_name = self._expect(TokenType.IDENTIFIER).value
                
                # Check for type annotation
                type_annotation = None
                if self.current_token and self.current_token.type == TokenType.COLON:
                    self._advance()
                    type_name = self._expect(TokenType.IDENTIFIER).value
                    type_annotation = TypeAnnotation(type_name)
                
                parameters.append(Variable(param_name, type_annotation))
                
                if self.current_token and self.current_token.type == TokenType.COMMA:
                    self._advance()
                else:
                    break
        
        self._expect(TokenType.RIGHT_BRACKET)
        
        # Parse return type (optional)
        return_type = None
        if self.current_token and self.current_token.type == TokenType.COLON:
            self._advance()
            return_type = self._expect(TokenType.IDENTIFIER).value
        
        # Parse body
        body = self._parse_block()
        
        return Function(name, parameters, return_type, body)
    
    def _parse_block(self) -> Block:
        """Parse a code block."""
        self._expect(TokenType.PIPE)
        
        statements = []
        while self.current_token and self.current_token.type != TokenType.CARET:
            statement = self._parse_statement()
            if statement:
                statements.append(statement)
        
        self._expect(TokenType.CARET)
        return Block(statements)
    
    def _parse_if_statement(self) -> IfStatement:
        """Parse if statement."""
        self._expect(TokenType.IF)
        
        self._expect(TokenType.LEFT_BRACKET)
        condition = self._parse_expression()
        self._expect(TokenType.RIGHT_BRACKET)
        
        then_block = self._parse_block()
        
        # Parse else block (optional)
        else_block = None
        if self.current_token and self.current_token.type == TokenType.ELSE:
            self._advance()
            else_block = self._parse_block()
        
        return IfStatement(condition, then_block, else_block)
    
    def _parse_while_loop(self) -> WhileLoop:
        """Parse while loop."""
        self._expect(TokenType.WHILE)
        
        self._expect(TokenType.LEFT_BRACKET)
        condition = self._parse_expression()
        self._expect(TokenType.RIGHT_BRACKET)
        
        body = self._parse_block()
        
        return WhileLoop(condition, body)
    
    def _parse_for_loop(self) -> ForLoop:
        """Parse for loop."""
        self._expect(TokenType.FOR)
        
        self._expect(TokenType.LEFT_BRACKET)
        variable = self._expect(TokenType.IDENTIFIER).value
        self._expect(TokenType.RIGHT_BRACKET)
        
        body = self._parse_block()
        
        return ForLoop(variable, None, body)  # Simplified for now
    
    def _parse_return_statement(self) -> ReturnStatement:
        """Parse return statement."""
        self._expect(TokenType.RETURN)
        
        value = None
        if self.current_token and self.current_token.type != TokenType.SEMICOLON:
            value = self._parse_expression()
        
        return ReturnStatement(value)
    
    def _parse_expression(self) -> Expression:
        """Parse an expression."""
        return self._parse_logical_or()
    
    def _parse_logical_or(self) -> Expression:
        """Parse logical OR expression."""
        left = self._parse_logical_and()
        
        while self.current_token and self.current_token.type == TokenType.OR:
            operator = self.current_token.value
            self._advance()
            right = self._parse_logical_and()
            left = BinaryOp(left, operator, right)
        
        return left
    
    def _parse_logical_and(self) -> Expression:
        """Parse logical AND expression."""
        left = self._parse_equality()
        
        while self.current_token and self.current_token.type == TokenType.AND:
            operator = self.current_token.value
            self._advance()
            right = self._parse_equality()
            left = BinaryOp(left, operator, right)
        
        return left
    
    def _parse_equality(self) -> Expression:
        """Parse equality expression."""
        left = self._parse_comparison()
        
        while self.current_token and self.current_token.type in [TokenType.EQUALS, TokenType.NOT_EQUALS]:
            operator = self.current_token.value
            self._advance()
            right = self._parse_comparison()
            left = BinaryOp(left, operator, right)
        
        return left
    
    def _parse_comparison(self) -> Expression:
        """Parse comparison expression."""
        left = self._parse_term()
        
        while self.current_token and self.current_token.type in [
            TokenType.LESS_THAN, TokenType.LESS_EQUAL,
            TokenType.GREATER_THAN, TokenType.GREATER_EQUAL
        ]:
            operator = self.current_token.value
            self._advance()
            right = self._parse_term()
            left = BinaryOp(left, operator, right)
        
        return left
    
    def _parse_term(self) -> Expression:
        """Parse addition/subtraction expression."""
        left = self._parse_factor()
        
        while self.current_token and self.current_token.type in [TokenType.PLUS, TokenType.MINUS]:
            operator = self.current_token.value
            self._advance()
            right = self._parse_factor()
            left = BinaryOp(left, operator, right)
        
        return left
    
    def _parse_factor(self) -> Expression:
        """Parse multiplication/division expression."""
        left = self._parse_unary()
        
        while self.current_token and self.current_token.type in [
            TokenType.MULTIPLY, TokenType.DIVIDE, TokenType.MODULO, TokenType.POWER
        ]:
            operator = self.current_token.value
            self._advance()
            right = self._parse_unary()
            left = BinaryOp(left, operator, right)
        
        return left
    
    def _parse_unary(self) -> Expression:
        """Parse unary expression."""
        if self.current_token and self.current_token.type == TokenType.NOT:
            operator = self.current_token.value
            self._advance()
            operand = self._parse_unary()
            return UnaryOp(operator, operand)
        
        return self._parse_primary()
    
    def _parse_primary(self) -> Expression:
        """Parse primary expression."""
        if not self.current_token:
            raise ParseError("Unexpected end of input")
        
        if self.current_token.type == TokenType.NUMBER:
            value = float(self.current_token.value)
            self._advance()
            return Literal(value, "number")
        
        elif self.current_token.type == TokenType.STRING:
            value = self.current_token.value[1:-1]  # Remove quotes
            self._advance()
            return Literal(value, "string")
        
        elif self.current_token.type == TokenType.TRUE:
            self._advance()
            return Literal(True, "boolean")
        
        elif self.current_token.type == TokenType.FALSE:
            self._advance()
            return Literal(False, "boolean")
        
        elif self.current_token.type == TokenType.NULL:
            self._advance()
            return Literal(None, "null")
        
        elif self.current_token.type == TokenType.IDENTIFIER:
            name = self.current_token.value
            self._advance()
            
            # Check if it's a function call
            if self.current_token and self.current_token.type == TokenType.LEFT_BRACKET:
                return self._parse_call(name)
            else:
                return Identifier(name)
        
        elif self.current_token.type == TokenType.LEFT_BRACKET:
            self._advance()
            expr = self._parse_expression()
            self._expect(TokenType.RIGHT_BRACKET)
            return expr
        
        elif self.current_token.type == TokenType.LEFT_BRACE:
            return self._parse_dictionary()
        
        else:
            raise ParseError(f"Unexpected token: {self.current_token.type.value}")
    
    def _parse_call(self, function_name: str) -> Call:
        """Parse function call."""
        self._expect(TokenType.LEFT_BRACKET)
        
        arguments = []
        if self.current_token and self.current_token.type != TokenType.RIGHT_BRACKET:
            while True:
                arg = self._parse_expression()
                arguments.append(arg)
                
                if self.current_token and self.current_token.type == TokenType.COMMA:
                    self._advance()
                else:
                    break
        
        self._expect(TokenType.RIGHT_BRACKET)
        
        return Call(Identifier(function_name), arguments)
    
    def _parse_dictionary(self) -> Dictionary:
        """Parse dictionary literal."""
        self._expect(TokenType.LEFT_BRACE)
        
        pairs = []
        if self.current_token and self.current_token.type != TokenType.RIGHT_BRACE:
            while True:
                key = self._parse_expression()
                self._expect(TokenType.COLON)
                value = self._parse_expression()
                pairs.append((key, value))
                
                if self.current_token and self.current_token.type == TokenType.COMMA:
                    self._advance()
                else:
                    break
        
        self._expect(TokenType.RIGHT_BRACE)
        
        return Dictionary(pairs)
