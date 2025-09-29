# Cortex Rust Architecture

## Overview

The Cortex Rust implementation follows a traditional compiler architecture with clear separation of concerns:

``` txt
Source Code → Lexer → Parser → AST → Interpreter → Output
```

## Components

### 1. Lexer (`lexer.rs`)

**Purpose**: Convert source code into tokens

**Key Features**:

- Tokenizes Cortex syntax
- Handles comments, strings, numbers, keywords
- Provides position information for error reporting

**Token Types**:

- Keywords: `func`, `let`, `if`, `while`, `return`
- Operators: `+`, `-`, `*`, `/`, `%`, `**`
- Delimiters: `[`, `]`, `|`, `^`
- Literals: numbers, strings, booleans
- Identifiers: variable and function names

### 2. Parser (`parser.rs`)

**Purpose**: Convert tokens into Abstract Syntax Tree (AST)

**Key Features**:

- Recursive descent parsing
- Error recovery and reporting
- Expression precedence handling

**AST Nodes**:

- `Program` - Root node containing statements
- `Function` - Function definitions
- `Statement` - Various statement types
- `Expression` - Expression evaluation

### 3. AST (`ast.rs`)

**Purpose**: Define the structure of parsed code

**Key Types**:

- `Program` - Top-level program structure
- `Function` - Function with parameters and body
- `Statement` - Control flow and assignments
- `Expression` - Values and operations

### 4. Interpreter (`codegen.rs`)

**Purpose**: Execute the AST

**Key Features**:

- Variable scope management
- Function call handling
- Expression evaluation
- Built-in function support

**Execution Model**:

- Tree-walking interpreter
- Stack-based variable storage
- Function call stack

### 5. Error Handling (`error.rs`)

**Purpose**: Comprehensive error reporting

**Error Types**:

- Lexical errors (invalid tokens)
- Parse errors (syntax violations)
- Runtime errors (execution failures)
- Type errors (type mismatches)

## Memory Management

The Rust implementation uses:

- **Ownership**: Automatic memory management
- **Borrowing**: Zero-copy string handling
- **Smart Pointers**: `Rc` for shared AST nodes
- **Arena Allocation**: Efficient AST storage

## Performance Optimizations

1. **String Interning**: Reduce memory usage for identifiers
2. **AST Caching**: Reuse parsed ASTs when possible
3. **Lazy Evaluation**: Defer expensive operations
4. **Zero-Copy Parsing**: Minimize string allocations

## Extension Points

The architecture supports easy extension:

1. **New Token Types**: Add to `lexer.rs`
2. **New AST Nodes**: Extend `ast.rs`
3. **New Built-ins**: Add to `codegen.rs`
4. **New Error Types**: Extend `error.rs`

## Testing

Each component has comprehensive tests:

```bash
cargo test
```

Tests cover:

- Lexer tokenization
- Parser correctness
- AST construction
- Interpreter execution
- Error handling
