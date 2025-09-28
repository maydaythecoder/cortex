# Rust Implementation Error Reference

This document provides specific error information for the Rust-based Cortex compiler implementation using an interpreter.

## Table of Contents

1. [Rust-Specific Errors](#rust-specific-errors)
2. [Interpreter Errors](#interpreter-errors)
3. [Borrow Checker Issues](#borrow-checker-issues)
4. [Cargo and Build Errors](#cargo-and-build-errors)
5. [Troubleshooting Rust Errors](#troubleshooting-rust-errors)

## Rust-Specific Errors

### Compilation Errors

**Error:** `error[E0502]: cannot borrow *self as mutable because it is also borrowed as immutable`

- **Cause:** Borrow checker conflict in interpreter methods
- **Location:** `codegen.rs` - function call handling
- **Solution:** Clone values before mutable borrows or restructure code

**Error:** `error[E0308]: mismatched types expected String, found &str`

- **Cause:** String type mismatch in lexer or parser
- **Location:** `lexer.rs` - token definitions
- **Solution:** Use consistent string types throughout

**Error:** `error[E0277]: ? operator couldn't convert the error`

- **Cause:** Error type mismatch in error handling
- **Location:** `main.rs` - error propagation
- **Solution:** Use consistent error types or proper error conversion

### Lexer Errors

**Error:** `Since 0.13 Logos no longer requires the #[error] variant`

- **Cause:** Deprecated Logos attribute usage
- **Location:** `lexer.rs` - Token enum
- **Solution:** Remove `#[error]` attribute from Error variant

**Error:** `Token type mismatch in lexer`

- **Cause:** Inconsistent token type definitions
- **Location:** `lexer.rs` - token parsing
- **Solution:** Ensure consistent token types

### Parser Errors

**Error:** `Unexpected token: expected expression, got Newline`

- **Cause:** Parser not properly skipping whitespace and comments
- **Location:** `parser.rs` - statement parsing
- **Solution:** Add proper token skipping logic

**Error:** `Parse error: expected '|' after function parameters`

- **Cause:** Malformed function definition
- **Location:** `parser.rs` - function parsing
- **Solution:** Check function syntax and delimiters

## Interpreter Errors

### Variable Scope Issues

**Error:** `Undefined variable: variable_name`

- **Cause:** Variable not found in current scope
- **Location:** `codegen.rs` - variable lookup
- **Solution:** Check variable declaration and scope

**Error:** `Variable 'variable_name' already declared`

- **Cause:** Variable redeclaration in same scope
- **Location:** `codegen.rs` - variable assignment
- **Solution:** Use different variable names or check scope

### Function Handling

**Error:** `Undefined function: function_name`

- **Cause:** Function not declared before use
- **Location:** `codegen.rs` - function call
- **Solution:** Declare function before calling it

**Error:** `Function 'function_name' expects N arguments, got M`

- **Cause:** Incorrect number of arguments in function call
- **Location:** `codegen.rs` - function call validation
- **Solution:** Check function signature and arguments

**Error:** `Function parameter binding failed`

- **Cause:** Error binding function parameters to arguments
- **Location:** `codegen.rs` - parameter binding
- **Solution:** Check parameter and argument types

### Return Statement Issues

**Error:** `Return statement outside function`

- **Cause:** Return statement used outside function context
- **Location:** `codegen.rs` - return statement handling
- **Solution:** Ensure return statements are inside functions

**Error:** `Function return type mismatch`

- **Cause:** Function returns different type than expected
- **Location:** `codegen.rs` - return value handling
- **Solution:** Ensure consistent return types

## Borrow Checker Issues

### Common Borrow Conflicts

**Error:** `cannot borrow *self as mutable because it is also borrowed as immutable`

- **Cause:** Holding immutable borrow while attempting mutable borrow
- **Location:** Various interpreter methods
- **Solution:** Clone values or restructure to avoid conflicts

**Error:** `borrow of moved value`

- **Cause:** Using value after it has been moved
- **Location:** Function parameter handling
- **Solution:** Use references or clone values as needed

**Error:** `use of moved value in closure`

- **Cause:** Moving values into closures
- **Location:** Error handling and callbacks
- **Solution:** Use references or clone values for closures

### Solutions for Borrow Issues

1. **Clone Values:**

   ```rust
   let function = function.clone();
   self.interpret_function_call(&function, &call.arguments)?;
   ```

2. **Use References:**

   ```rust
   fn interpret_expression(&mut self, expr: &Expression) -> Result<Value>
   ```

3. **Restructure Code:**

   ```rust
   // Evaluate condition first
   let condition = self.interpret_expression(&while_loop.condition)?;
   if !self.is_truthy(&condition) {
       break;
   }
   // Then interpret body
   self.interpret_block(&while_loop.body)?;
   ```

## Cargo and Build Errors

### Dependency Issues

**Error:** `failed to resolve: use of unresolved module or unlinked crate`

- **Cause:** Missing or incompatible dependencies
- **Location:** `Cargo.toml` - dependencies
- **Solution:** Check dependency versions and compatibility

**Error:** `package not found in registry`

- **Cause:** Package name or version doesn't exist
- **Location:** `Cargo.toml` - dependencies
- **Solution:** Verify package names and versions

### Build Configuration

**Error:** `target not found`

- **Cause:** Invalid target specification
- **Location:** `Cargo.toml` - target configuration
- **Solution:** Check target specification

**Error:** `feature not found`

- **Cause:** Requested feature doesn't exist
- **Location:** `Cargo.toml` - features
- **Solution:** Check available features for the package

## Runtime Errors

### Interpreter Runtime Issues

**Error:** `Division by zero`

- **Cause:** Attempting to divide by zero
- **Location:** `codegen.rs` - arithmetic operations
- **Solution:** Add zero-division checks

**Error:** `Stack overflow`

- **Cause:** Infinite recursion or deep call stack
- **Location:** `codegen.rs` - function calls
- **Solution:** Add recursion depth limits

**Error:** `Out of memory`

- **Cause:** Excessive memory allocation
- **Location:** `codegen.rs` - value storage
- **Solution:** Optimize memory usage and add limits

### Value Type Errors

**Error:** `Type mismatch in operation`

- **Cause:** Incompatible types in binary operations
- **Location:** `codegen.rs` - binary operations
- **Solution:** Add type checking and conversion

**Error:** `Invalid value for operation`

- **Cause:** Value not suitable for the operation
- **Location:** `codegen.rs` - value operations
- **Solution:** Add value validation

## Troubleshooting Rust Errors

### Debug Mode

Enable debug output:

```bash
cargo run -- --debug run program.ctx
```

### Verbose Output

Enable verbose logging:

```bash
RUST_LOG=debug cargo run -- run program.ctx
```

### Cargo Clean

Clean build artifacts:

```bash
cargo clean
cargo build
```

### Common Fixes

1. **Update Dependencies:**

   ```bash
   cargo update
   ```

2. **Check Rust Version:**

   ```bash
   rustc --version
   cargo --version
   ```

3. **Clear Cargo Cache:**

   ```bash
   cargo clean
   rm -rf ~/.cargo/registry/cache
   ```

### Error Logging

Enable detailed error logging:

```rust
use log::debug;
debug!("Debug information: {:?}", value);
```

## Performance Issues

### Interpreter Performance

**Issue:** Slow execution

- **Cause:** Inefficient interpreter implementation
- **Solution:** Optimize hot paths and reduce allocations

**Issue:** High memory usage

- **Cause:** Excessive value cloning and storage
- **Solution:** Use references where possible and implement cleanup

### Build Performance

**Issue:** Slow compilation

- **Cause:** Large dependency tree or inefficient code
- **Solution:** Optimize dependencies and code structure

## Testing and Validation

### Unit Tests

Run Rust-specific tests:

```bash
cargo test
```

### Integration Tests

Test with example programs:

```bash
cargo run -- run ../docs/shared/examples/hello_world.ctx
```

### Benchmarking

Run performance benchmarks:

```bash
cargo bench
```

## Memory Management

### Value Storage

**Issue:** Memory leaks in value storage

- **Cause:** Values not properly cleaned up
- **Solution:** Implement proper cleanup and use `Drop` trait

**Issue:** Excessive cloning

- **Cause:** Unnecessary value cloning
- **Solution:** Use references and smart pointers

### Garbage Collection

**Issue:** Accumulated unused values

- **Cause:** No garbage collection in interpreter
- **Solution:** Implement reference counting or garbage collection

## Error Handling Best Practices

### Use Result Types

```rust
fn interpret_expression(&mut self, expr: &Expression) -> Result<Value> {
    // Implementation
}
```

### Provide Context

```rust
Err(anyhow::anyhow!(
    "Function {} expects {} arguments, got {}",
    function.name,
    function.parameters.len(),
    arguments.len()
))
```

### Chain Errors

```rust
let result = self.interpret_expression(arg)
    .context("Failed to interpret function argument")?;
```

## Related Documentation

- [Shared Error Reference](../shared/ERRORS.md)
- [Function Status](../shared/FUNCTION_STATUS.md)
- [Rust Implementation Guide](README.md)
- [Development Guide](../shared/DEVELOPMENT.md)
- [Rust Book](https://doc.rust-lang.org/book/)

## Contributing

When fixing Rust-specific errors:

1. Update this documentation
2. Add test cases for the error scenario
3. Ensure borrow checker compliance
4. Test with multiple Rust versions
5. Verify Cargo compatibility
6. Add proper error context and messages
