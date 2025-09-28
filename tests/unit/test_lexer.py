"""
Unit tests for the Cortex lexer.
"""

import unittest
from compiler.parser.lexer import Lexer, TokenType


class TestLexer(unittest.TestCase):
    """Test cases for the Cortex lexer."""
    
    def test_basic_tokens(self):
        """Test basic token recognition."""
        source = "let x := 42"
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        expected_types = [
            TokenType.LET,
            TokenType.IDENTIFIER,
            TokenType.ASSIGN,
            TokenType.NUMBER
        ]
        
        self.assertEqual(len(tokens), len(expected_types) + 1)  # +1 for EOF
        for i, expected_type in enumerate(expected_types):
            self.assertEqual(tokens[i].type, expected_type)
    
    def test_numbers(self):
        """Test number tokenization."""
        source = "42 3.14 1e5 2.5e-3"
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        # Filter out non-number tokens
        number_tokens = [t for t in tokens if t.type == TokenType.NUMBER]
        
        expected_values = ["42", "3.14", "1e5", "2.5e-3"]
        self.assertEqual(len(number_tokens), len(expected_values))
        
        for i, expected_value in enumerate(expected_values):
            self.assertEqual(number_tokens[i].value, expected_value)
    
    def test_strings(self):
        """Test string tokenization."""
        source = '"hello" "world with spaces" "escaped\\nstring"'
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        # Filter out non-string tokens
        string_tokens = [t for t in tokens if t.type == TokenType.STRING]
        
        expected_values = ['"hello"', '"world with spaces"', '"escaped\\nstring"']
        self.assertEqual(len(string_tokens), len(expected_values))
        
        for i, expected_value in enumerate(expected_values):
            self.assertEqual(string_tokens[i].value, expected_value)
    
    def test_operators(self):
        """Test operator tokenization."""
        source = "+ - * / % ** == != < <= > >= && || ! @ ⊗ ∇ ∂"
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        expected_types = [
            TokenType.PLUS, TokenType.MINUS, TokenType.MULTIPLY, TokenType.DIVIDE,
            TokenType.MODULO, TokenType.POWER, TokenType.EQUALS, TokenType.NOT_EQUALS,
            TokenType.LESS_THAN, TokenType.LESS_EQUAL, TokenType.GREATER_THAN,
            TokenType.GREATER_EQUAL, TokenType.AND, TokenType.OR, TokenType.NOT,
            TokenType.DOT_PRODUCT, TokenType.OUTER_PRODUCT, TokenType.GRADIENT,
            TokenType.PARTIAL
        ]
        
        # Filter out non-operator tokens
        operator_tokens = [t for t in tokens if t.type in expected_types]
        
        self.assertEqual(len(operator_tokens), len(expected_types))
        for i, expected_type in enumerate(expected_types):
            self.assertEqual(operator_tokens[i].type, expected_type)
    
    def test_delimiters(self):
        """Test delimiter tokenization."""
        source = "[ ] { } ( ) | ^ , ; : ."
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        expected_types = [
            TokenType.LEFT_BRACKET, TokenType.RIGHT_BRACKET,
            TokenType.LEFT_BRACE, TokenType.RIGHT_BRACE,
            TokenType.LEFT_PAREN, TokenType.RIGHT_PAREN,
            TokenType.PIPE, TokenType.CARET,
            TokenType.COMMA, TokenType.SEMICOLON,
            TokenType.COLON, TokenType.DOT
        ]
        
        # Filter out non-delimiter tokens
        delimiter_tokens = [t for t in tokens if t.type in expected_types]
        
        self.assertEqual(len(delimiter_tokens), len(expected_types))
        for i, expected_type in enumerate(expected_types):
            self.assertEqual(delimiter_tokens[i].type, expected_type)
    
    def test_keywords(self):
        """Test keyword tokenization."""
        source = "let func if else while for return true false null"
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        expected_types = [
            TokenType.LET, TokenType.FUNC, TokenType.IF, TokenType.ELSE,
            TokenType.WHILE, TokenType.FOR, TokenType.RETURN,
            TokenType.TRUE, TokenType.FALSE, TokenType.NULL
        ]
        
        # Filter out non-keyword tokens
        keyword_tokens = [t for t in tokens if t.type in expected_types]
        
        self.assertEqual(len(keyword_tokens), len(expected_types))
        for i, expected_type in enumerate(expected_types):
            self.assertEqual(keyword_tokens[i].type, expected_type)
    
    def test_comments(self):
        """Test comment tokenization."""
        source = """
        // Single line comment
        /* Multi-line
           comment */
        let x := 42
        """
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        # Filter out non-comment tokens
        comment_tokens = [t for t in tokens if t.type == TokenType.COMMENT]
        
        self.assertEqual(len(comment_tokens), 2)
        self.assertTrue(comment_tokens[0].value.startswith("//"))
        self.assertTrue(comment_tokens[1].value.startswith("/*"))
    
    def test_identifiers(self):
        """Test identifier tokenization."""
        source = "variable_name _private CONSTANT_NAME var123"
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        # Filter out non-identifier tokens
        identifier_tokens = [t for t in tokens if t.type == TokenType.IDENTIFIER]
        
        expected_values = ["variable_name", "_private", "CONSTANT_NAME", "var123"]
        self.assertEqual(len(identifier_tokens), len(expected_values))
        
        for i, expected_value in enumerate(expected_values):
            self.assertEqual(identifier_tokens[i].value, expected_value)
    
    def test_const_assignment(self):
        """Test constant assignment operator."""
        source = "let MAX_EPOCHS :: 1000"
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        expected_types = [
            TokenType.LET,
            TokenType.IDENTIFIER,
            TokenType.ASSIGN_CONST,
            TokenType.NUMBER
        ]
        
        # Filter out non-expected tokens
        relevant_tokens = [t for t in tokens if t.type in expected_types]
        
        self.assertEqual(len(relevant_tokens), len(expected_types))
        for i, expected_type in enumerate(expected_types):
            self.assertEqual(relevant_tokens[i].type, expected_type)
    
    def test_error_handling(self):
        """Test error handling for invalid input."""
        source = "let x := 42 @ invalid_char"
        lexer = Lexer(source)
        
        # This should raise a SyntaxError for the invalid character
        with self.assertRaises(SyntaxError):
            lexer.tokenize()
    
    def test_line_column_tracking(self):
        """Test line and column tracking."""
        source = """let x := 42
let y := 3.14"""
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        # Check first line
        let_token = tokens[0]
        self.assertEqual(let_token.line, 1)
        self.assertEqual(let_token.column, 1)
        
        # Check second line
        second_let_token = None
        for token in tokens:
            if token.type == TokenType.LET and token.line == 2:
                second_let_token = token
                break
        
        self.assertIsNotNone(second_let_token)
        self.assertEqual(second_let_token.line, 2)
        self.assertEqual(second_let_token.column, 1)


if __name__ == '__main__':
    unittest.main()
