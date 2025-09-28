# Cortex Rust Implementation

The Rust implementation of the Cortex compiler provides a high-performance interpreter for the Cortex programming language, built with modern Rust tooling and best practices.

## Features

- **High Performance**: Fast interpretation with Rust's zero-cost abstractions
- **Memory Safety**: Rust's ownership system prevents memory leaks and crashes
- **Modern Tooling**: Built with Cargo, clap, and other modern Rust crates
- **Comprehensive Error Handling**: Detailed error messages with line/column information

## Installation

```bash
cd rust
cargo build --release
```

## Usage

### Running Programs (Interpreter Mode)

```bash
cargo run -- run program.ctx
```

### Syntax Checking

```bash
cargo run -- check program.ctx
```

### Building Executable

```bash
cargo run -- build program.ctx -o program
```

## Architecture

### Core Components

1. **Lexer** (`src/lexer.rs`)
   - Uses `logos` crate for efficient tokenization
   - Handles all Cortex language tokens
   - Provides position tracking for error reporting

2. **Parser** (`src/parser.rs`)
   - Recursive descent parser
   - Comprehensive error handling with `thiserror`
   - Generates well-structured AST

3. **AST** (`src/ast.rs`)
   - Complete type definitions for all language constructs
   - Visitor pattern support for future extensions
   - Clone and debug implementations

4. **Interpreter** (`src/codegen.rs`)
   - Direct AST interpretation
   - HashMap-based variable and function storage
   - Support for all basic operations

5. **Error Handling** (`src/error.rs`)
   - Custom error types with `thiserror`
   - Detailed error messages with context
   - Integration with `anyhow` for error propagation

### Dependencies

- **logos**: High-performance lexer generation
- **anyhow**: Error handling and context
- **thiserror**: Custom error types
- **clap**: Command-line argument parsing
- **regex**: String processing
- **indexmap**: Ordered hash maps
- **log/env_logger**: Logging support

## Supported Language Features

### ✅ Fully Implemented

- Variables and constants (`let x := 10`, `let y :: 20`)
- Functions with parameters
- Arithmetic operations (+, -, *, /, %, **)
- Comparison operators (==, !=, <, <=, >, >=)
- Logical operators (&&, ||, !)
- Control flow (if/else, while loops)
- Built-in functions (`print`)
- String operations
- Comments (single-line `//` and multi-line `/* */`)

### 🔄 In Development

- Function return values
- For loops
- Array data structures
- Dictionary/map data structures
- Advanced type system
- String concatenation improvements

## Example Usage

### Basic Arithmetic

```cortex
func main[] |
  let a := 10
  let b := 3
  
  let sum := a + b
  let diff := a - b
  let product := a * b
  let quotient := a / b
  
  print[sum]
  print[diff]
  print[product]
  print[quotient]
^

main[]
```

### Functions and Loops

```cortex
func main[] |
  let counter := 1
  
  while [counter <= 5] |
    print[counter]
    let counter := counter + 1
  ^
  
  // Factorial calculation
  let n := 5
  let factorial := 1
  let temp := 1
  
  while [temp <= n] |
    let factorial := factorial * temp
    let temp := temp + 1
  ^
  
  print[factorial]
^

main[]
```

## Performance

The Rust implementation provides excellent performance characteristics:

- **Startup Time**: Fast binary startup with minimal overhead
- **Memory Usage**: Efficient memory management with Rust's ownership system
- **Interpretation Speed**: Fast AST traversal and execution
- **Error Handling**: Zero-cost error handling with `Result` types

## Development

### Adding New Features

1. **Lexer**: Add new tokens in `lexer.rs` using `logos` attributes
2. **Parser**: Add parsing methods in `parser.rs`
3. **AST**: Define new node types in `ast.rs`
4. **Interpreter**: Implement interpretation logic in `codegen.rs`
5. **Tests**: Add test cases using Rust's built-in testing

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Error Handling

The Rust implementation provides comprehensive error handling:

```rust
// Custom error types with context
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, got {actual} at line {line}:{column}")]
    UnexpectedToken { expected: String, actual: String, line: usize, column: usize },
    // ... more error types
}
```

## Memory Management

Rust's ownership system ensures memory safety:

- **No Garbage Collection**: Predictable memory usage
- **No Memory Leaks**: Compile-time guarantees
- **No Dangling Pointers**: Ownership and borrowing prevent use-after-free
- **Efficient Allocation**: Stack allocation where possible

## Limitations

- Function parameters are not fully implemented
- String concatenation needs improvement
- No advanced type checking
- Limited error recovery in parser

## Future Enhancements

- [ ] Complete function parameter handling
- [ ] Advanced type system
- [ ] Standard library implementation
- [ ] JIT compilation for better performance
- [ ] Advanced optimization passes
- [ ] Debug information generation
- [ ] Profiling and performance analysis
- [ ] WebAssembly target support

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

MIT License - see LICENSE file for details.
