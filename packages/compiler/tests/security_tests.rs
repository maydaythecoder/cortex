//! Security tests for the Cortex compiler
//! 
//! Tests various security measures including recursion limits, input validation,
//! and resource constraints.

use cortex_rust::{
    lexer::{Lexer, Token},
    parser::Parser,
    security::{SecurityConfig, SecurityContext},
};

#[test]
fn test_recursion_limit_enforcement() {
    // Create a strict security context with low recursion limit
    let security = SecurityContext::strict();
    
    // Create deeply nested expression that should exceed recursion limit
    let mut source = String::new();
    for _ in 0..200 {
        source.push_str("(");
    }
    source.push_str("42");
    for _ in 0..200 {
        source.push_str(")");
    }
    
    let mut lexer = Lexer::with_security(&source, security);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::with_security(tokens, SecurityContext::strict());
    
    // This should fail due to recursion limit
    assert!(parser.parse().is_err());
}

#[test]
fn test_source_size_limit() {
    // Create a very large source file that exceeds size limit
    let mut source = String::new();
    for _ in 0..200000 { // 200KB of 'a' characters
        source.push('a');
    }
    
    let security = SecurityContext::strict();
    let mut lexer = Lexer::with_security(&source, security);
    
    // This should fail due to source size limit
    assert!(lexer.tokenize().is_err());
}

#[test]
fn test_string_length_limit() {
    // Create a very long string literal
    let mut source = String::from("\"");
    for _ in 0..2000 { // 2KB string
        source.push('x');
    }
    source.push('"');
    
    let security = SecurityContext::strict();
    let mut lexer = Lexer::with_security(&source, security);
    
    // This should fail due to string length limit
    assert!(lexer.tokenize().is_err());
}

#[test]
fn test_identifier_length_limit() {
    // Create a very long identifier
    let mut source = String::new();
    for _ in 0..300 { // 300 character identifier
        source.push('a');
    }
    
    let security = SecurityContext::strict();
    let mut lexer = Lexer::with_security(&source, security);
    
    // This should fail due to identifier length limit
    assert!(lexer.tokenize().is_err());
}

#[test]
fn test_function_parameter_limit() {
    // Create a function with too many parameters
    let mut source = String::from("func test[");
    for i in 0..50 {
        if i > 0 {
            source.push_str(", ");
        }
        source.push_str(&format!("param{}", i));
    }
    source.push_str("] | return[42] ^");
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::with_security(tokens, SecurityContext::strict());
    
    // This should fail due to parameter count limit
    assert!(parser.parse().is_err());
}

#[test]
fn test_array_size_limit() {
    // Create an array with too many elements
    let mut source = String::from("[");
    for i in 0..2000 {
        if i > 0 {
            source.push_str(", ");
        }
        source.push_str(&format!("{}", i));
    }
    source.push(']');
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::with_security(tokens, SecurityContext::strict());
    
    // This should fail due to collection size limit
    assert!(parser.parse().is_err());
}

#[test]
fn test_dictionary_size_limit() {
    // Create a dictionary with too many pairs
    let mut source = String::from("{");
    for i in 0..2000 {
        if i > 0 {
            source.push_str(", ");
        }
        source.push_str(&format!("\"key{}\": {}", i, i));
    }
    source.push('}');
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::with_security(tokens, SecurityContext::strict());
    
    // This should fail due to collection size limit
    assert!(parser.parse().is_err());
}

#[test]
fn test_security_configs() {
    // Test default configuration
    let default_config = SecurityConfig::new();
    assert_eq!(default_config.max_recursion_depth, 1000);
    assert_eq!(default_config.max_source_size, 1024 * 1024);
    assert!(!default_config.strict_mode);
    
    // Test strict configuration
    let strict_config = SecurityConfig::strict();
    assert_eq!(strict_config.max_recursion_depth, 100);
    assert_eq!(strict_config.max_source_size, 100 * 1024);
    assert!(strict_config.strict_mode);
    
    // Test permissive configuration
    let permissive_config = SecurityConfig::permissive();
    assert_eq!(permissive_config.max_recursion_depth, 10_000);
    assert_eq!(permissive_config.max_source_size, 10 * 1024 * 1024);
    assert!(!permissive_config.strict_mode);
}

#[test]
fn test_security_contexts() {
    // Test default context
    let default_context = SecurityContext::new();
    assert_eq!(default_context.config.max_recursion_depth, 1000);
    
    // Test strict context
    let strict_context = SecurityContext::strict();
    assert_eq!(strict_context.config.max_recursion_depth, 100);
    assert!(strict_context.config.strict_mode);
    
    // Test permissive context
    let permissive_context = SecurityContext::permissive();
    assert_eq!(permissive_context.config.max_recursion_depth, 10_000);
    assert!(!permissive_context.config.strict_mode);
}

#[test]
fn test_normal_operations_with_security() {
    // Test that normal operations still work with security enabled
    let source = r#"
        func add[a, b] |
            return[a + b]
        ^
        
        let result := add[10, 20]
        print[result]
    "#;
    
    let security = SecurityContext::new();
    let mut lexer = Lexer::with_security(source, security);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::with_security(tokens, SecurityContext::new());
    let ast = parser.parse().unwrap();
    
    // Should parse successfully (function + assignment + print)
    assert!(ast.statements.len() >= 2);
}

#[test]
fn test_recursion_tracker_accuracy() {
    let mut tracker = SecurityContext::strict().recursion_tracker;
    
    // Test entering and exiting recursion levels (strict limit is 100)
    for i in 1..=100 {
        assert!(tracker.enter().is_ok());
        assert_eq!(tracker.current_depth(), i);
    }
    
    // Test exceeding limit (should fail at depth 101)
    assert!(tracker.enter().is_err());
    
    // Test exiting
    for i in (1..=101).rev() {
        tracker.exit();
        assert_eq!(tracker.current_depth(), i - 1);
    }
}

#[test]
fn test_security_error_messages() {
    // Test that security errors provide helpful messages
    let mut source = String::from("\"");
    for _ in 0..2000 {
        source.push('x');
    }
    source.push('"');
    
    let security = SecurityContext::strict();
    let mut lexer = Lexer::with_security(&source, security);
    let result = lexer.tokenize();
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = format!("{}", error);
    
    // Should contain helpful information about the security limit
    assert!(error_msg.contains("String literal length"));
    assert!(error_msg.contains("exceeds maximum allowed"));
}
