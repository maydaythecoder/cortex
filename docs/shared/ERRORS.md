# Cortex Compiler Error Reference

This document provides a comprehensive reference for all error types, messages, and troubleshooting information for the Cortex programming language compilers.

## Table of Contents

1. [Error Categories](#error-categories)
2. [Common Error Patterns](#common-error-patterns)
3. [Language-Specific Errors](#language-specific-errors)
4. [Troubleshooting Guide](#troubleshooting-guide)
5. [Error Recovery Strategies](#error-recovery-strategies)
6. [Debugging Tips](#debugging-tips)

## Error Categories

### 1. Lexical Errors

Errors that occur during the tokenization phase when the lexer cannot recognize valid tokens.

**Common Causes:**

- Invalid characters in identifiers
- Unterminated string literals
- Malformed numeric literals
- Invalid escape sequences

**Examples:**

``` txt
Error: Unexpected character '#' at line 5, column 3
Error: Unterminated string literal at line 10
Error: Invalid number format '12.34.56' at line 3
```

### 2. Syntax Errors

Errors that occur during parsing when the code structure doesn't match the language grammar.

**Common Causes:**

- Missing semicolons or delimiters
- Mismatched brackets or parentheses
- Invalid statement structure
- Incorrect function definitions

**Examples:**

``` txt
Error: Expected '|' after function parameters at line 7
Error: Unexpected token '^' at line 12, expected expression
Error: Missing closing bracket ']' at line 5
```

### 3. Semantic Errors

Errors that occur during semantic analysis when the code is syntactically correct but semantically invalid.

**Common Causes:**

- Undefined variables or functions
- Type mismatches
- Invalid function calls
- Scope violations

**Examples:**

``` txt
Error: Undefined variable 'x' at line 8
Error: Function 'add' expects 2 arguments, got 3
Error: Type mismatch: expected number, got string
```

### 4. Runtime Errors

Errors that occur during program execution.

**Common Causes:**

- Division by zero
- Array index out of bounds
- Null pointer dereference
- Stack overflow

**Examples:**

``` txt
Error: Division by zero at line 15
Error: Array index 10 out of bounds (array size: 5)
Error: Stack overflow in recursive function
```

## Common Error Patterns

### Variable Declaration Issues

**Pattern:** `Undefined variable 'variable_name'`

- **Cause:** Variable used before declaration or outside its scope
- **Solution:** Declare the variable before use or check scope

**Pattern:** `Variable 'variable_name' already declared`

- **Cause:** Variable declared multiple times in the same scope
- **Solution:** Use different variable names or check for duplicate declarations

### Function Call Issues

**Pattern:** `Undefined function 'function_name'`

- **Cause:** Function not declared or imported
- **Solution:** Declare the function before calling it

**Pattern:** `Function 'function_name' expects N arguments, got M`

- **Cause:** Incorrect number of arguments in function call
- **Solution:** Check function signature and provide correct number of arguments

### Type System Issues

**Pattern:** `Type mismatch: expected type_A, got type_B`

- **Cause:** Incompatible types in operation or assignment
- **Solution:** Use type conversion or correct the type

**Pattern:** `Cannot convert type_A to type_B`

- **Cause:** No valid conversion between types
- **Solution:** Use explicit type conversion or change the operation

## Language-Specific Errors

### Python Implementation Errors

**LLVM-Related Errors:**

- `'NoneType' object has no attribute 'gep'`
- `Operands must be the same type`
- `Undefined variable in global scope`

**Common Python-Specific Issues:**

- Global variable handling in functions
- String concatenation type mismatches
- LLVM IR generation errors

### Rust Implementation Errors

**Interpreter-Related Errors:**

- `Undefined variable in function scope`
- `Function parameter binding errors`
- `Return value handling issues`

**Common Rust-Specific Issues:**

- Borrow checker conflicts
- Function parameter scoping
- Return statement implementation

## Troubleshooting Guide

### Step 1: Identify Error Category

1. Check if it's a lexical error (invalid characters)
2. Check if it's a syntax error (malformed structure)
3. Check if it's a semantic error (undefined symbols)
4. Check if it's a runtime error (execution failure)

### Step 2: Locate Error Source

1. Note the line and column numbers
2. Check the surrounding code context
3. Look for common patterns in the error message
4. Compare with working examples

### Step 3: Apply Fixes

1. Use the error message to guide the fix
2. Check language-specific documentation
3. Verify the fix doesn't introduce new errors
4. Test with multiple examples

### Step 4: Verify Solution

1. Compile the fixed code
2. Run test cases
3. Check for regressions
4. Update documentation if needed

## Error Recovery Strategies

### Automatic Recovery

- **Lexical:** Skip invalid characters and continue
- **Syntax:** Insert missing tokens and continue parsing
- **Semantic:** Provide default values for undefined variables

### Manual Recovery

- **User Intervention:** Prompt for corrections
- **Fallback Behavior:** Use alternative implementations
- **Error Reporting:** Provide detailed error information

### Graceful Degradation

- **Partial Compilation:** Compile valid parts, report errors
- **Warning Mode:** Continue with warnings instead of errors
- **Debug Mode:** Provide additional debugging information

## Debugging Tips

### Enable Verbose Output

```bash
# Python implementation
python cortexc.py --verbose run program.ctx

# Rust implementation
cargo run -- --verbose run program.ctx
```

### Use Debug Mode

```bash
# Python implementation
python cortexc.py --debug build program.ctx

# Rust implementation
cargo run -- --debug build program.ctx
```

### Check Intermediate Representations

- **Python:** Examine generated LLVM IR
- **Rust:** Use interpreter debug output
- **Both:** Enable AST visualization

### Common Debugging Techniques

1. **Add Print Statements:** Insert debug output in code
2. **Use Breakpoints:** Pause execution at specific points
3. **Trace Execution:** Follow program flow step by step
4. **Check State:** Verify variable values and function calls

## Error Message Format

### Standard Format

``` txt
Error: [Error Type] [Description] at line [line], column [column]
Context: [Surrounding code]
Suggestion: [How to fix]
```

### Example

``` txt
Error: Syntax Error - Expected '|' after function parameters at line 7, column 15
Context: func add[a, b] 
                ^
Suggestion: Add '|' after the parameter list
```

## Best Practices

### For Developers

1. **Clear Error Messages:** Provide specific, actionable error messages
2. **Error Context:** Include surrounding code and line numbers
3. **Recovery Suggestions:** Offer specific fixes when possible
4. **Consistent Format:** Use standardized error message format

### For Users

1. **Read Error Messages:** Carefully read the full error message
2. **Check Context:** Look at the surrounding code
3. **Follow Suggestions:** Try the suggested fixes first
4. **Test Incrementally:** Fix one error at a time

## Related Documentation

- [Python Error Reference](../python/ERRORS.md)
- [Rust Error Reference](../rust/ERRORS.md)
- [Development Guide](DEVELOPMENT.md)
- [Language Specification](SYNTAX.md)

## Contributing

When adding new error types or improving error messages:

1. Update this shared documentation
2. Update language-specific error documentation
3. Add test cases for new error scenarios
4. Ensure error messages are user-friendly
5. Include recovery suggestions where possible
