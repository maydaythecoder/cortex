# Cortex Rust API Reference

## CLI Interface

### Commands

#### `run`

Execute a Cortex program

```bash
cargo run -- run <file.ctx>
```

**Options**:

- `--verbose` - Enable verbose output
- `--debug` - Enable debug information

#### `check`

Check syntax without execution

```bash
cargo run -- check <file.ctx>
```

**Options**:

- `--strict` - Enable strict mode
- `--warnings` - Show warnings

#### `build`

Build executable (placeholder)

```bash
cargo run -- build <file.ctx> -o <output>
```

**Options**:

- `-o, --output` - Output file name
- `--optimize` - Optimization level (0-3)

## Core API

### Lexer

```rust
use cortex::lexer::Lexer;

let mut lexer = Lexer::new(source_code);
let tokens = lexer.tokenize()?;
```

**Methods**:

- `new(source: &str)` - Create new lexer
- `tokenize()` - Tokenize source code
- `peek()` - Look ahead at next token
- `consume()` - Consume current token

### Parser

```rust
use cortex::parser::Parser;

let mut parser = Parser::new(tokens);
let ast = parser.parse()?;
```

**Methods**:

- `new(tokens: Vec<Token>)` - Create new parser
- `parse()` - Parse tokens into AST
- `parse_expression()` - Parse single expression
- `parse_statement()` - Parse single statement

### Interpreter

```rust
use cortex::codegen::Interpreter;

let mut interpreter = Interpreter::new();
interpreter.interpret(&ast)?;
```

**Methods**:

- `new()` - Create new interpreter
- `interpret(ast: &Program)` - Execute AST
- `evaluate(expr: &Expression)` - Evaluate expression
- `call_function(name: &str, args: Vec<Value>)` - Call function

## Error Handling

### Error Types

```rust
use cortex::error::CompilerError;

match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(CompilerError::LexicalError { message, line, column }) => {
        eprintln!("Lexical error at {}:{} - {}", line, column, message);
    }
    Err(CompilerError::ParseError { message, line, column }) => {
        eprintln!("Parse error at {}:{} - {}", line, column, message);
    }
    Err(CompilerError::RuntimeError { message }) => {
        eprintln!("Runtime error: {}", message);
    }
}
```

### Error Recovery

The parser includes error recovery mechanisms:

- **Skip invalid tokens** - Continue parsing after errors
- **Insert missing tokens** - Add expected tokens
- **Synchronize** - Find next valid statement

## Built-in Functions

### Print Functions

```cortex
print[value]     // Print value without newline
println[value]   // Print value with newline
```

### String Functions

```cortex
str[value]       // Convert value to string
len[string]      // Get string length
```

### Math Functions

```cortex
abs[number]      // Absolute value
sqrt[number]     // Square root
pow[base, exp]   // Power function
```

## Configuration

### Environment Variables

- `CORTEX_DEBUG` - Enable debug output
- `CORTEX_VERBOSE` - Enable verbose output
- `CORTEX_COLOR` - Enable colored output

### Configuration File

Create `.cortexrc` in project root:

```json
{
    "debug": false,
    "verbose": false,
    "color": true,
    "optimize": 2
}
```

## Extending the Language

### Adding Built-in Functions

1. Add function to `Interpreter` in `codegen.rs`
2. Register in `builtin_functions` map
3. Add tests

```rust
impl Interpreter {
    fn builtin_sqrt(&mut self, args: &[Expression]) -> Result<Value> {
        let value = self.evaluate(&args[0])?;
        match value {
            Value::Number(n) => Ok(Value::Number(n.sqrt())),
            _ => Err(CompilerError::TypeError {
                message: "sqrt expects a number".to_string(),
            }),
        }
    }
}
```

### Adding New Types

1. Extend `Value` enum in `ast.rs`
2. Update lexer for new literals
3. Update parser for new expressions
4. Update interpreter for new operations

## Performance Tuning

### Compilation Flags

```bash
# Release build
cargo build --release

# Optimize for size
cargo build --release -- -C opt-level=s

# Optimize for speed
cargo build --release -- -C opt-level=3
```

### Runtime Optimization

- Use `--release` flag for production
- Enable LTO (Link Time Optimization)
- Use `jemalloc` for better memory allocation

## Debugging

### Debug Output

```bash
RUST_LOG=debug cargo run -- run program.ctx
```

### Profiling

```bash
# Install perf
cargo install cargo-profdata

# Profile execution
cargo profdata run -- run program.ctx
```

### Memory Analysis

```bash
# Install valgrind (Linux)
valgrind --tool=memcheck cargo run -- run program.ctx

# Use heaptrack (Linux)
heaptrack cargo run -- run program.ctx
```
