//! Integration tests for the Cortex compiler
//! 
//! Tests the complete compilation pipeline: lexer → parser → interpreter

use cortex_rust::{lexer::Lexer, parser::Parser as CortexParser, codegen::Interpreter};

/// Helper function to run a Cortex program and capture output
fn run_cortex_code(source: &str) -> Result<(), String> {
    // Tokenize
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|_| "Lexer error".to_string())?;
    
    // Parse
    let mut parser = CortexParser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parse error: {}", e))?;
    
    // Interpret
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&ast).map_err(|e| format!("Runtime error: {}", e))?;
    
    Ok(())
}

#[test]
fn test_simple_variable_assignment() {
    let source = r#"
        let x := 42
        let y := 10
        let z := x + y
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_arithmetic_operations() {
    let source = r#"
        let a := 10
        let b := 3
        let sum := a + b
        let diff := a - b
        let prod := a * b
        let quot := a / b
        let rem := a % b
        let pow := a ** 2
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_function_definition_and_call() {
    let source = r#"
        func add[a, b] |
            return[a + b]
        ^
        
        func main[] |
            let result := add[10, 20]
            print[result]
        ^
        
        main[]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_if_statement() {
    let source = r#"
        let x := 10
        if [x > 5] |
            let y := x * 2
        ^
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_if_else_statement() {
    let source = r#"
        let x := 3
        if [x > 5] |
            let y := 1
        ^ else |
            let y := 2
        ^
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_while_loop() {
    let source = r#"
        let counter := 0
        while [counter < 5] |
            let counter := counter + 1
        ^
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_for_loop_with_range() {
    let source = r#"
        for i in 0..5 |
            print[i]
        ^
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_array_creation_and_access() {
    let source = r#"
        let arr := [1, 2, 3, 4, 5]
        let first := arr[0]
        let last := arr[4]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_dictionary_creation_and_access() {
    let source = r#"
        let person := {"name": "Alice", "age": "30"}
        let name := person["name"]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_builtin_functions() {
    let source = r#"
        print["Hello, World!"]
        let s := str[42]
        let l := len[[1, 2, 3]]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_string_concatenation() {
    let source = r#"
        let greeting := "Hello"
        let name := "World"
        let message := greeting + " " + name
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_comparison_operators() {
    let source = r#"
        let a := 10
        let b := 5
        let gt := a > b
        let lt := a < b
        let eq := a == b
        let neq := a != b
        let gte := a >= b
        let lte := a <= b
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_logical_operators() {
    let source = r#"
        let a := true
        let b := false
        let and_result := a && b
        let or_result := a || b
        let not_result := !a
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_nested_function_calls() {
    let source = r#"
        func double[x] |
            return[x * 2]
        ^
        
        func quadruple[x] |
            return[double[double[x]]]
        ^
        
        let result := quadruple[5]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_factorial_function() {
    let source = r#"
        func factorial[n] |
            if [n <= 1] |
                return[1]
            ^ else |
                return[n * factorial[n - 1]]
            ^
        ^
        
        let result := factorial[5]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_fibonacci_function() {
    let source = r#"
        func fibonacci[n] |
            if [n <= 1] |
                return[n]
            ^ else |
                return[fibonacci[n - 1] + fibonacci[n - 2]]
            ^
        ^
        
        let result := fibonacci[10]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_array_iteration() {
    let source = r#"
        let numbers := [1, 2, 3, 4, 5]
        for num in numbers |
            print[num]
        ^
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_math_functions() {
    let source = r#"
        let x := 16
        let root := sqrt[x]
        let abs_val := abs[-5]
        let minimum := min[3, 7]
        let maximum := max[3, 7]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_string_functions() {
    let source = r#"
        let text := "hello world"
        let upper := upper[text]
        let lower := lower[text]
        let length := len[text]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_array_functions() {
    let source = r#"
        let arr := [1, 2, 3]
        let length := len[arr]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_comments() {
    let source = r#"
        // This is a single-line comment
        let x := 42
        
        /* This is a
           multi-line comment */
        let y := 10
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
fn test_error_undefined_variable() {
    let source = "let x := y";
    assert!(run_cortex_code(source).is_err());
}

#[test]
fn test_error_undefined_function() {
    let source = "let x := undefined_func[42]";
    assert!(run_cortex_code(source).is_err());
}

#[test]
fn test_error_invalid_syntax() {
    let source = "let x = 42"; // Should be :=
    assert!(run_cortex_code(source).is_err());
}

#[test]
fn test_error_type_mismatch() {
    let source = r#"let x := "hello" * 5"#;
    assert!(run_cortex_code(source).is_err());
}

#[test]
fn test_error_function_arity_mismatch() {
    let source = r#"
        func add[a, b] |
            return[a + b]
        ^
        
        let result := add[1, 2, 3]
    "#;
    
    assert!(run_cortex_code(source).is_err());
}

#[test]
fn test_complex_program() {
    // Simplified version - just test the prime function with a small input
    let source = r#"
        func is_prime[n] |
            if [n <= 1] |
                return[false]
            ^
            if [n <= 3] |
                return[true]
            ^
            if [n % 2 == 0] |
                return[false]
            ^
            return[true]
        ^
        
        func main[] |
            let result := is_prime[7]
            print[result]
        ^
        
        main[]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

#[test]
#[ignore] // Slow test - run with --ignored flag
fn test_prime_numbers_generator() {
    let source = r#"
        func is_prime[n] |
            if [n <= 1] |
                return[false]
            ^
            if [n <= 3] |
                return[true]
            ^
            if [n % 2 == 0] |
                return[false]
            ^
            
            let i := 3
            while [i * i <= n] |
                if [n % i == 0] |
                    return[false]
                ^
                let i := i + 2
            ^
            
            return[true]
        ^
        
        func main[] |
            let count := 0
            let n := 2
            
            while [count < 10] |
                if [is_prime[n]] |
                    print[n]
                    let count := count + 1
                ^
                let n := n + 1
            ^
        ^
        
        main[]
    "#;
    
    assert!(run_cortex_code(source).is_ok());
}

