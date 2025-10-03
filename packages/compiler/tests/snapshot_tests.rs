//! Snapshot Tests for Cortex Compiler
//! 
//! These tests use the `insta` crate to capture and validate AST snapshots.
//! This helps prevent regressions by comparing parsed ASTs against known-good snapshots.
//! 
//! To review snapshots: `cargo insta review`
//! To update snapshots: `cargo insta test --review`

use cortex_rust::{lexer::Lexer, parser::Parser};

#[test]
fn snapshot_simple_assignment() {
    let source = "let x := 42";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_function_declaration() {
    let source = r#"
        func fibonacci[n: number] : number |
            if[n < 2] |
                return[n]
            ^
            return[fibonacci[n - 1] + fibonacci[n - 2]]
        ^
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_complex_expression() {
    let source = "let result := (a + b) ** 2 + c * d / e";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_operator_precedence() {
    let source = "2 + 3 * 4 ** 2 - 1";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_power_right_associativity() {
    let source = "2 ** 3 ** 2";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_logical_operators() {
    let source = "true || false && true";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_if_else_chain() {
    let source = r#"
        if[x < 0] |
            print["negative"]
        ^ else if[x == 0] |
            print["zero"]
        ^ else |
            print["positive"]
        ^
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_for_loop_with_range() {
    let source = r#"
        for i in 0..10 |
            print[i]
        ^
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_array_creation() {
    let source = "let arr := [1, 2, 3, 4, 5]";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_dictionary_creation() {
    let source = r#"let dict := {"name": "Cortex", "version": 1.0}"#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_nested_function_calls() {
    let source = "add[multiply[2, 3], divide[10, 2]]";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
#[ignore] // TODO: Implement AI/ML operators in parser (Phase 4)
fn snapshot_ai_ml_operators() {
    // Test AI/ML specific operators like dot product, gradient, partial
    let source = r#"
        let result := matrix1 @ matrix2
        let grad := ∇[loss]
        let partial := ∂[f, x]
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
#[ignore] // TODO: Implement AI/ML operators in parser (Phase 4)
fn snapshot_complex_ai_program() {
    let source = r#"
        func neural_network[inputs, weights] |
            let layer1 := inputs @ weights
            let activation := relu[layer1]
            return[activation]
        ^
        
        let model := neural_network[data, params]
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_while_loop() {
    let source = r#"
        let i := 0
        while[i < 10] |
            print[i]
            i := i + 1
        ^
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_array_indexing() {
    let source = "arr[0] := 42";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_comparison_chain() {
    let source = "1 < 2 && 2 <= 3 && 3 == 3 && 4 != 5";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_unary_operators() {
    let source = "let x := -5 + !false";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_constant_declaration() {
    let source = "let PI :: 3.14159";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_multi_param_function() {
    let source = r#"
        func calculate[x: number, y: number, z: number] : number |
            return[x * y + z]
        ^
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_empty_array() {
    let source = "let empty := []";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}

#[test]
fn snapshot_string_literals() {
    let source = r#"let message := "Hello, Cortex!""#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    insta::assert_yaml_snapshot!(ast);
}
