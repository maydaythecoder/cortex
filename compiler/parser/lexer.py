"""
Cortex Lexer

Tokenizes Cortex source code into a stream of tokens.
"""

from enum import Enum
from typing import List, Optional
import re


class TokenType(Enum):
    # Literals
    NUMBER = "NUMBER"
    STRING = "STRING"
    IDENTIFIER = "IDENTIFIER"
    BOOLEAN = "BOOLEAN"
    
    # Keywords
    LET = "LET"
    FUNC = "FUNC"
    IF = "IF"
    ELSE = "ELSE"
    WHILE = "WHILE"
    FOR = "FOR"
    RETURN = "RETURN"
    TRUE = "TRUE"
    FALSE = "FALSE"
    NULL = "NULL"
    
    # Operators
    PLUS = "PLUS"
    MINUS = "MINUS"
    MULTIPLY = "MULTIPLY"
    DIVIDE = "DIVIDE"
    MODULO = "MODULO"
    POWER = "POWER"
    ASSIGN = "ASSIGN"
    ASSIGN_CONST = "ASSIGN_CONST"  # ::
    EQUALS = "EQUALS"
    NOT_EQUALS = "NOT_EQUALS"
    LESS_THAN = "LESS_THAN"
    LESS_EQUAL = "LESS_EQUAL"
    GREATER_THAN = "GREATER_THAN"
    GREATER_EQUAL = "GREATER_EQUAL"
    AND = "AND"
    OR = "OR"
    NOT = "NOT"
    
    # AI/ML specific operators
    DOT_PRODUCT = "DOT_PRODUCT"  # @
    OUTER_PRODUCT = "OUTER_PRODUCT"  # ⊗
    GRADIENT = "GRADIENT"  # ∇
    PARTIAL = "PARTIAL"  # ∂
    NORM = "NORM"  # |x|
    
    # Delimiters
    LEFT_BRACKET = "LEFT_BRACKET"  # [
    RIGHT_BRACKET = "RIGHT_BRACKET"  # ]
    LEFT_BRACE = "LEFT_BRACE"  # {
    RIGHT_BRACE = "RIGHT_BRACE"  # }
    LEFT_PAREN = "LEFT_PAREN"  # (
    RIGHT_PAREN = "RIGHT_PAREN"  # )
    PIPE = "PIPE"  # |
    CARET = "CARET"  # ^
    COMMA = "COMMA"
    SEMICOLON = "SEMICOLON"
    COLON = "COLON"
    DOT = "DOT"
    
    # Special
    NEWLINE = "NEWLINE"
    EOF = "EOF"
    COMMENT = "COMMENT"


class Token:
    def __init__(self, type: TokenType, value: str, line: int, column: int):
        self.type = type
        self.value = value
        self.line = line
        self.column = column
    
    def __repr__(self):
        return f"Token({self.type.value}, '{self.value}', {self.line}:{self.column})"


class Lexer:
    def __init__(self, source: str):
        self.source = source
        self.position = 0
        self.line = 1
        self.column = 1
        self.tokens = []
    
    def tokenize(self) -> List[Token]:
        """Tokenize the source code."""
        while self.position < len(self.source):
            self._skip_whitespace()
            if self.position >= len(self.source):
                break
            
            char = self.source[self.position]
            
            # Handle comments
            if char == '/' and self._peek() == '/':
                self._read_single_line_comment()
                continue
            elif char == '/' and self._peek() == '*':
                self._read_multi_line_comment()
                continue
            
            # Handle numbers
            if char.isdigit() or char == '.':
                self._read_number()
                continue
            
            # Handle strings
            if char == '"':
                self._read_string()
                continue
            
            # Handle identifiers and keywords
            if char.isalpha() or char == '_':
                self._read_identifier()
                continue
            
            # Handle operators and delimiters
            if char in '+-*/%=<>!&|^[]{}()|^,;:.':
                self._read_operator()
                continue
            
            # Handle special characters
            if char == '\n':
                self._add_token(TokenType.NEWLINE, '\n')
                self._advance()
                self.line += 1
                self.column = 1
                continue
            
            # Unknown character
            raise SyntaxError(f"Unexpected character '{char}' at {self.line}:{self.column}")
        
        self._add_token(TokenType.EOF, '')
        return self.tokens
    
    def _skip_whitespace(self):
        """Skip whitespace characters."""
        while self.position < len(self.source) and self.source[self.position].isspace():
            if self.source[self.position] == '\n':
                self.line += 1
                self.column = 1
            else:
                self.column += 1
            self.position += 1
    
    def _read_single_line_comment(self):
        """Read a single-line comment."""
        start_line = self.line
        start_column = self.column
        comment = "//"
        self._advance(2)
        
        while self.position < len(self.source) and self.source[self.position] != '\n':
            comment += self.source[self.position]
            self._advance()
        
        self._add_token(TokenType.COMMENT, comment, start_line, start_column)
    
    def _read_multi_line_comment(self):
        """Read a multi-line comment."""
        start_line = self.line
        start_column = self.column
        comment = "/*"
        self._advance(2)
        
        while self.position < len(self.source) - 1:
            if self.source[self.position] == '*' and self.source[self.position + 1] == '/':
                comment += "*/"
                self._advance(2)
                break
            comment += self.source[self.position]
            if self.source[self.position] == '\n':
                self.line += 1
                self.column = 1
            else:
                self.column += 1
            self.position += 1
        else:
            raise SyntaxError(f"Unterminated comment at {start_line}:{start_column}")
        
        self._add_token(TokenType.COMMENT, comment, start_line, start_column)
    
    def _read_number(self):
        """Read a number literal."""
        start_line = self.line
        start_column = self.column
        number = ""
        
        # Read integer part
        while self.position < len(self.source) and self.source[self.position].isdigit():
            number += self.source[self.position]
            self._advance()
        
        # Read decimal part
        if self.position < len(self.source) and self.source[self.position] == '.':
            number += self.source[self.position]
            self._advance()
            while self.position < len(self.source) and self.source[self.position].isdigit():
                number += self.source[self.position]
                self._advance()
        
        # Read scientific notation
        if self.position < len(self.source) and self.source[self.position].lower() == 'e':
            number += self.source[self.position]
            self._advance()
            if self.position < len(self.source) and self.source[self.position] in '+-':
                number += self.source[self.position]
                self._advance()
            while self.position < len(self.source) and self.source[self.position].isdigit():
                number += self.source[self.position]
                self._advance()
        
        self._add_token(TokenType.NUMBER, number, start_line, start_column)
    
    def _read_string(self):
        """Read a string literal."""
        start_line = self.line
        start_column = self.column
        string = '"'
        self._advance()  # Skip opening quote
        
        while self.position < len(self.source) and self.source[self.position] != '"':
            if self.source[self.position] == '\\':
                string += self.source[self.position]
                self._advance()
                if self.position < len(self.source):
                    string += self.source[self.position]
                    self._advance()
            else:
                string += self.source[self.position]
                self._advance()
        
        if self.position >= len(self.source):
            raise SyntaxError(f"Unterminated string at {start_line}:{start_column}")
        
        string += '"'
        self._advance()  # Skip closing quote
        
        self._add_token(TokenType.STRING, string, start_line, start_column)
    
    def _read_identifier(self):
        """Read an identifier or keyword."""
        start_line = self.line
        start_column = self.column
        identifier = ""
        
        while (self.position < len(self.source) and 
               (self.source[self.position].isalnum() or self.source[self.position] == '_')):
            identifier += self.source[self.position]
            self._advance()
        
        # Check if it's a keyword
        keyword_map = {
            'let': TokenType.LET,
            'func': TokenType.FUNC,
            'if': TokenType.IF,
            'else': TokenType.ELSE,
            'while': TokenType.WHILE,
            'for': TokenType.FOR,
            'return': TokenType.RETURN,
            'true': TokenType.TRUE,
            'false': TokenType.FALSE,
            'null': TokenType.NULL,
        }
        
        token_type = keyword_map.get(identifier.lower(), TokenType.IDENTIFIER)
        self._add_token(token_type, identifier, start_line, start_column)
    
    def _read_operator(self):
        """Read an operator or delimiter."""
        start_line = self.line
        start_column = self.column
        char = self.source[self.position]
        
        # Handle multi-character operators
        if char == ':' and self._peek() == ':':
            self._add_token(TokenType.ASSIGN_CONST, '::')
            self._advance(2)
            return
        elif char == ':' and self._peek() == '=':
            self._add_token(TokenType.ASSIGN, ':=')
            self._advance(2)
            return
        elif char == '=' and self._peek() == '=':
            self._add_token(TokenType.EQUALS, '==')
            self._advance(2)
            return
        elif char == '!' and self._peek() == '=':
            self._add_token(TokenType.NOT_EQUALS, '!=')
            self._advance(2)
            return
        elif char == '<' and self._peek() == '=':
            self._add_token(TokenType.LESS_EQUAL, '<=')
            self._advance(2)
            return
        elif char == '>' and self._peek() == '=':
            self._add_token(TokenType.GREATER_EQUAL, '>=')
            self._advance(2)
            return
        elif char == '&' and self._peek() == '&':
            self._add_token(TokenType.AND, '&&')
            self._advance(2)
            return
        elif char == '|' and self._peek() == '|':
            self._add_token(TokenType.OR, '||')
            self._advance(2)
            return
        elif char == '*' and self._peek() == '*':
            self._add_token(TokenType.POWER, '**')
            self._advance(2)
            return
        
        # Handle single-character operators
        operator_map = {
            '+': TokenType.PLUS,
            '-': TokenType.MINUS,
            '*': TokenType.MULTIPLY,
            '/': TokenType.DIVIDE,
            '%': TokenType.MODULO,
            '=': TokenType.ASSIGN,
            '<': TokenType.LESS_THAN,
            '>': TokenType.GREATER_THAN,
            '!': TokenType.NOT,
            '@': TokenType.DOT_PRODUCT,
            '⊗': TokenType.OUTER_PRODUCT,
            '∇': TokenType.GRADIENT,
            '∂': TokenType.PARTIAL,
            '[': TokenType.LEFT_BRACKET,
            ']': TokenType.RIGHT_BRACKET,
            '{': TokenType.LEFT_BRACE,
            '}': TokenType.RIGHT_BRACE,
            '(': TokenType.LEFT_PAREN,
            ')': TokenType.RIGHT_PAREN,
            '|': TokenType.PIPE,
            '^': TokenType.CARET,
            ',': TokenType.COMMA,
            ';': TokenType.SEMICOLON,
            ':': TokenType.COLON,
            '.': TokenType.DOT,
        }
        
        token_type = operator_map.get(char, None)
        if token_type:
            self._add_token(token_type, char, start_line, start_column)
            self._advance()
        else:
            raise SyntaxError(f"Unknown operator '{char}' at {self.line}:{self.column}")
    
    def _peek(self, offset: int = 1) -> Optional[str]:
        """Peek at the next character without advancing."""
        if self.position + offset < len(self.source):
            return self.source[self.position + offset]
        return None
    
    def _advance(self, count: int = 1):
        """Advance the position."""
        for _ in range(count):
            if self.position < len(self.source):
                self.position += 1
                self.column += 1
    
    def _add_token(self, type: TokenType, value: str, line: int = None, column: int = None):
        """Add a token to the list."""
        if line is None:
            line = self.line
        if column is None:
            column = self.column
        self.tokens.append(Token(type, value, line, column))
