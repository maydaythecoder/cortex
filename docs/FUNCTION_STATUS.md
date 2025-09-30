# Cortex Language Function Status

This document provides a comprehensive overview of which functions and language features are implemented and working in each compiler implementation.

## Table of Contents

1. [Core Language Features](#core-language-features)
2. [Built-in Functions](#built-in-functions)
3. [Control Flow](#control-flow)
4. [Data Types](#data-types)
5. [Operators](#operators)
6. [Function Definitions](#function-definitions)
7. [Variable Handling](#variable-handling)
8. [Error Handling](#error-handling)
9. [Implementation Notes](#implementation-notes)

## Core Language Features

| Feature | Python (LLVM) | Rust (Interpreter) | Notes |
|---------|---------------|-------------------|-------|
| **Basic Syntax** | ✅ Working | ✅ Working | Both support Cortex syntax |
| **Comments** | ✅ Working | ✅ Working | Single-line (`//`) and multi-line (`/* */`) |
| **String Literals** | ✅ Working | ✅ Working | Double-quoted strings |
| **Number Literals** | ✅ Working | ✅ Working | Integer and floating-point |
| **Identifiers** | ✅ Working | ✅ Working | Variable and function names |

## Built-in Functions

| Function | Python (LLVM) | Rust (Interpreter) | Implementation Details |
|----------|---------------|-------------------|----------------------|
| **print()** | ✅ Working | ✅ Working | Outputs to stdout |
| **str()** | ⚠️ Partial | ✅ Working | Number-to-string conversion |
| **len()** | ❌ Not Implemented | ❌ Not Implemented | String/array length |
| **input()** | ❌ Not Implemented | ❌ Not Implemented | User input |
| **type()** | ❌ Not Implemented | ❌ Not Implemented | Type checking |
| **abs()** | ❌ Not Implemented | ❌ Not Implemented | Absolute value |
| **min()** | ❌ Not Implemented | ❌ Not Implemented | Minimum value |
| **max()** | ❌ Not Implemented | ❌ Not Implemented | Maximum value |
| **sqrt()** | ❌ Not Implemented | ❌ Not Implemented | Square root |
| **pow()** | ⚠️ Partial | ❌ Not Implemented | Power operation (Python has `**` operator) |

### Built-in Function Details

#### print()

- **Python:** Generates LLVM IR for `printf` calls
- **Rust:** Direct stdout output via `println!`
- **Status:** Fully functional in both implementations

#### str()

- **Python:** Placeholder implementation returning "number" or "value"
- **Rust:** Proper number-to-string conversion
- **Status:** Rust implementation is complete, Python needs improvement

## Control Flow

| Feature | Python (LLVM) | Rust (Interpreter) | Notes |
|---------|---------------|-------------------|-------|
| **if statements** | ✅ Working | ✅ Working | Conditional execution |
| **while loops** | ✅ Working | ✅ Working | Loop with condition |
| **for loops** | ⚠️ Partial | ⚠️ Partial | Range-based iteration |
| **break statements** | ❌ Not Implemented | ❌ Not Implemented | Loop termination |
| **continue statements** | ❌ Not Implemented | ❌ Not Implemented | Loop continuation |
| **return statements** | ✅ Working | ✅ Working | Function return values |

### Control Flow Details

#### if statements

- **Python:** Generates LLVM IR with conditional branches
- **Rust:** Interpreter evaluates conditions and executes branches
- **Status:** Both implementations handle if/else correctly

#### while loops

- **Python:** LLVM IR with loop blocks and conditional jumps
- **Rust:** Interpreter loop with condition evaluation
- **Status:** Both implementations work correctly

#### for loops

- **Python:** Basic structure implemented, needs range handling
- **Rust:** Placeholder implementation
- **Status:** Both need completion for range-based iteration

## Data Types

| Type | Python (LLVM) | Rust (Interpreter) | Implementation |
|------|---------------|-------------------|----------------|
| **Numbers** | ✅ Working | ✅ Working | Integer and floating-point |
| **Strings** | ✅ Working | ✅ Working | Text data |
| **Booleans** | ✅ Working | ✅ Working | True/false values |
| **Arrays** | ❌ Not Implemented | ❌ Not Implemented | Ordered collections |
| **Dictionaries** | ❌ Not Implemented | ❌ Not Implemented | Key-value pairs |
| **Null** | ✅ Working | ✅ Working | Empty/none values |

### Data Type Details

#### Numbers

- **Python:** LLVM `double` type for all numbers
- **Rust:** `f64` for floating-point numbers
- **Status:** Both handle arithmetic operations correctly

#### Strings

- **Python:** LLVM string constants and operations
- **Rust:** `String` type with proper memory management
- **Status:** Both support string literals and basic operations

#### Booleans

- **Python:** LLVM boolean operations
- **Rust:** `bool` type with truthiness evaluation
- **Status:** Both handle boolean logic correctly

## Operators

| Operator | Python (LLVM) | Rust (Interpreter) | Notes |
|----------|---------------|-------------------|-------|
| **Arithmetic** | | | |
| `+` (addition) | ✅ Working | ✅ Working | Numbers and strings |
| `-` (subtraction) | ✅ Working | ✅ Working | Numbers only |
| `*` (multiplication) | ✅ Working | ✅ Working | Numbers only |
| `/` (division) | ✅ Working | ✅ Working | Numbers only |
| `%` (modulo) | ✅ Working | ✅ Working | Numbers only |
| `**` (power) | ✅ Working | ❌ Not Implemented | Python uses external function |
| **Comparison** | | | |
| `==` (equal) | ✅ Working | ✅ Working | All types |
| `!=` (not equal) | ✅ Working | ✅ Working | All types |
| `<` (less than) | ✅ Working | ✅ Working | Numbers and strings |
| `<=` (less or equal) | ✅ Working | ✅ Working | Numbers and strings |
| `>` (greater than) | ✅ Working | ✅ Working | Numbers and strings |
| `>=` (greater or equal) | ✅ Working | ✅ Working | Numbers and strings |

Logical Operators
`&&` - Logical AND - `a && b`
`||` - Logical OR - `a || b`
`!` - Logical NOT - `!a

### Operator Details

#### String Concatenation

- **Python:** Basic implementation, needs improvement
- **Rust:** Proper string concatenation
- **Status:** Rust is more complete

#### Power Operation

- **Python:** Uses external LLVM function call
- **Rust:** Not implemented
- **Status:** Python has working implementation

## Function Definitions

| Feature | Python (LLVM) | Rust (Interpreter) | Notes |
|---------|---------------|-------------------|-------|
| **Function declaration** | ✅ Working | ✅ Working | `func name[params] \| body ^` |
| **Parameters** | ✅ Working | ✅ Working | Multiple parameters supported |
| **Return values** | ✅ Working | ✅ Working | Explicit return statements |
| **Local variables** | ✅ Working | ✅ Working | Function-scoped variables |
| **Global variables** | ✅ Working | ✅ Working | Program-scoped variables |
| **Recursion** | ✅ Working | ✅ Working | Self-calling functions |
| **Nested functions** | ❌ Not Implemented | ❌ Not Implemented | Functions inside functions |

### Function Definition Details

#### Parameter Handling

- **Python:** LLVM function parameters with proper types
- **Rust:** Interpreter parameter binding and scope management
- **Status:** Both handle parameters correctly

#### Return Values

- **Python:** LLVM return instructions
- **Rust:** Interpreter return value handling
- **Status:** Both support return statements

#### Variable Scoping

- **Python:** LLVM local and global variable management
- **Rust:** HashMap-based variable storage with scope isolation
- **Status:** Both handle variable scoping correctly

## Variable Handling

| Feature | Python (LLVM) | Rust (Interpreter) | Notes |
|---------|---------------|-------------------|-------|
| **Variable declaration** | ✅ Working | ✅ Working | `let` and `const` keywords |
| **Variable assignment** | ✅ Working | ✅ Working | `=` operator |
| **Global variables** | ✅ Working | ✅ Working | Program-level variables |
| **Local variables** | ✅ Working | ✅ Working | Function-level variables |
| **Variable shadowing** | ✅ Working | ✅ Working | Local overrides global |
| **Type inference** | ✅ Working | ✅ Working | Automatic type detection |

### Variable Handling Details

#### Declaration Keywords

- **Python:** Supports both `let` and `const`
- **Rust:** Supports both `let` and `const`
- **Status:** Both implementations are complete

#### Scope Management

- **Python:** LLVM-based scope handling
- **Rust:** HashMap-based scope with proper isolation
- **Status:** Both handle scoping correctly

## Error Handling

| Feature | Python (LLVM) | Rust (Interpreter) | Notes |
|---------|---------------|-------------------|-------|
| **Syntax errors** | ✅ Working | ✅ Working | Parser error reporting |
| **Type errors** | ⚠️ Partial | ⚠️ Partial | Basic type checking |
| **Runtime errors** | ⚠️ Partial | ⚠️ Partial | Division by zero, etc. |
| **Undefined variables** | ✅ Working | ✅ Working | Error reporting |
| **Undefined functions** | ✅ Working | ✅ Working | Error reporting |
| **Argument mismatch** | ✅ Working | ✅ Working | Parameter validation |

### Error Handling Details

#### Error Reporting

- **Python:** LLVM compilation errors
- **Rust:** Interpreter runtime errors with context
- **Status:** Both provide useful error messages

#### Type Safety

- **Python:** Basic type checking during LLVM generation
- **Rust:** Runtime type checking in interpreter
- **Status:** Both have basic type safety

## Implementation Notes

### Python Implementation (LLVM Backend)

- **Strengths:**
  - High-performance compiled output
  - LLVM optimization support
  - Professional-grade code generation
  - Good error reporting during compilation

- **Limitations:**
  - Complex LLVM setup requirements
  - Limited string operations
  - Placeholder implementations for some functions
  - Requires LLVM installation

### Rust Implementation (Interpreter Backend)

- **Strengths:**
  - Simple setup and execution
  - Complete string handling
  - Proper function parameter binding
  - Good error messages with context
  - Memory-safe implementation

- **Limitations:**
  - Slower execution (interpreted)
  - Missing some operators (power, not)
  - Limited built-in functions
  - No compilation to native code

## Testing Status

### Working Examples

Both implementations successfully run:

- `simple_arithmetic.ctx` - Basic arithmetic operations
- `simple_loops.ctx` - While loops and factorial
- `hello_world.ctx` - String printing
- `function_demo.ctx` - Functions with parameters and return values

### Test Coverage

- **Python:** 4/4 examples working
- **Rust:** 4/4 examples working
- **Overall:** 100% of provided examples functional

## Future Development

### High Priority

1. **Array support** - Both implementations need array data structures
2. **Dictionary support** - Key-value pair data structures
3. **More built-in functions** - `len()`, `input()`, `type()`, etc.
4. **Complete for loops** - Range-based iteration
5. **String operations** - Better concatenation, substring, etc.

### Medium Priority

1. **Error recovery** - Better error handling and recovery
2. **Type system** - More robust type checking
3. **Standard library** - Common utility functions
4. **Performance optimization** - Both implementations

### Low Priority

1. **Advanced features** - Closures, generics, etc.
2. **Tooling** - Debugger, profiler, etc.
3. **Documentation** - More examples and tutorials
4. **Packaging** - Distribution and installation

## Contributing

When adding new functions or features:

1. **Update this document** with the new feature status
2. **Implement in both languages** for consistency
3. **Add test cases** to verify functionality
4. **Update documentation** with usage examples
5. **Consider performance implications** for both implementations

## Related Documentation

- [Shared Error Reference](ERRORS.md)
- [Python Implementation Guide](../python/README.md)
- [Rust Implementation Guide](../rust/README.md)
- [Development Guidelines](DEVELOPMENT.md)
- [Language Comparison](COMPARISON.md)
- [Language Comparison](COMPARISON.md)
