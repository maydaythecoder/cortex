# Cortex Python Implementation

The Python implementation of the Cortex compiler provides a complete LLVM-based compilation pipeline for the Cortex programming language.

## Features

- **LLVM Backend**: Full compilation to LLVM IR with binary generation
- **Complete Language Support**: Variables, functions, loops, conditionals, arithmetic
- **Cross-Platform**: Automatic target detection (macOS ARM64/x86_64, Linux)
- **Optimized Output**: Generates efficient machine code

## Installation

```bash
cd python
python3 -m venv venv
source venv/bin/activate
pip install -r ../requirements.txt
```

## Usage

### Running Programs (Interpreter Mode)

```bash
python cortexc.py run program.ctx
```

### Compiling to Binary

```bash
python cortexc.py build program.ctx -o program
```

### Building with Optimization

```bash
python cortexc.py build program.ctx -o program -O 2
```

## Architecture

### Core Components

1. **Lexer** (`compiler/parser/lexer.py`)
   - Tokenizes Cortex source code
   - Handles comments, whitespace, and special characters

2. **Parser** (`compiler/parser/parser.py`)
   - Converts tokens to Abstract Syntax Tree (AST)
   - Implements recursive descent parsing

3. **AST** (`compiler/ast/nodes.py`)
   - Defines all language constructs
   - Provides type-safe representation of programs

4. **Code Generator** (`compiler/codegen.py`)
   - Converts AST to LLVM IR
   - Handles type mapping and optimization

5. **Interpreter** (`compiler/interpreter.py`)
   - Direct execution of AST
   - Used for testing and development

### LLVM Integration

The Python implementation uses `llvmlite` for LLVM integration:

- **Target Detection**: Automatically detects platform (ARM64/x86_64)
- **Type System**: Maps Cortex types to LLVM types
- **Function Calls**: Supports both built-in and user-defined functions
- **Memory Management**: Handles global and local variables

## Supported Language Features

### ✅ Fully Implemented

- Variables and constants (`let x := 10`, `let y :: 20`)
- Functions with parameters and return values
- Arithmetic operations (+, -, *, /, %, **)
- Comparison operators (==, !=, <, <=, >, >=)
- Logical operators (&&, \|, !)
- Control flow (if/else, while loops)
- Built-in functions (`print`, `str`)
- String concatenation
- Comments (single-line `//` and multi-line `/* */`)

### 🔄 In Development

- For loops
- Array data structures
- Dictionary/map data structures
- Advanced type system
- Error handling improvements

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

### Functions and Variables

```cortex
let greeting := "Hello"

func add[a, b] |
  return[a + b]
^

func main[] |
  print[greeting + " from Cortex!"]
  let sum := add[10, 20]
  print["10 + 20 = " + str[sum]]
^

main[]
```

## Testing

Run the test suite:

```bash
python test_compiler.py
```

## Performance

The Python implementation generates optimized LLVM IR that compiles to efficient machine code. Performance characteristics:

- **Compilation Speed**: Fast AST generation and LLVM IR output
- **Runtime Performance**: Near-native speed with LLVM optimizations
- **Memory Usage**: Efficient memory management with LLVM's garbage collection

## Development

### Adding New Features

1. **Lexer**: Add new tokens in `lexer.py`
2. **Parser**: Add parsing rules in `parser.py`
3. **AST**: Define new node types in `nodes.py`
4. **Code Generator**: Implement LLVM IR generation in `codegen.py`
5. **Tests**: Add test cases in `test_compiler.py`

### Debugging

Enable debug output:

```bash
python cortexc.py run program.ctx --debug
```

View generated LLVM IR:

```bash
python cortexc.py build program.ctx -o program
cat program.ll
```

## Limitations

- Function parameters are currently limited to basic types
- String operations are basic (concatenation only)
- No advanced error recovery in parser
- Limited type checking (mostly runtime)

## Future Enhancements

- [ ] Advanced type system with compile-time checking
- [ ] Standard library with common functions
- [ ] Package/module system
- [ ] Advanced optimization passes
- [ ] Debug information generation
- [ ] Profiling and performance analysis tools
