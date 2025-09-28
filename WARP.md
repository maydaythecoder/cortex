# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## About This Project

Cortex is a compiled programming language designed specifically for AI and machine learning. It features a unique bracket-based syntax, LLVM-based compilation, and AI-first primitives like tensors, gradients, and training utilities.

## Development Commands

### Core Development Workflow

```bash
# Install dependencies
pip install -r requirements.txt

# Run a Cortex program (interpreter mode)
python3 cortexc.py run examples/hello_world.ctx

# Compile a Cortex program to binary
python3 cortexc.py build examples/hello_world.ctx -o hello_world

# Start interactive REPL
python3 cortexc.py repl

# Run tests
python3 test_compiler.py

# Run specific example programs
python3 cortexc.py run examples/arithmetic.ctx
python3 cortexc.py run examples/function_demo.ctx
```

### Testing and Validation

```bash
# Test the compiler infrastructure
python3 test_compiler.py

# Compile and run all examples
python3 cortexc.py build examples/arithmetic.ctx -o arithmetic && ./arithmetic
python3 cortexc.py build examples/loops.ctx -o loops && ./loops
python3 cortexc.py build examples/printing.ctx -o printing && ./printing

# Verbose compilation (useful for debugging)
python3 cortexc.py build examples/hello_world.ctx -o hello -v
```

## Architecture Overview

### High-Level Architecture

**Three-Stage Compilation Pipeline:**

1. **Source → AST**: Lexer/Parser (`compiler/parser/`) tokenizes and parses `.ctx` files
2. **AST → LLVM IR**: Code generator (`compiler/codegen.py`) produces LLVM intermediate representation
3. **LLVM IR → Binary**: LLVM compiler creates optimized native executables

**Key Components:**

- **`cortexc.py`**: Main CLI tool with `run`, `build`, and `repl` commands
- **`compiler/parser/`**: Lexical analysis and parsing to AST
- **`compiler/ast/`**: AST node definitions with visitor pattern
- **`compiler/interpreter.py`**: Direct AST interpreter for rapid development
- **`compiler/compiler.py`**: LLVM-based compiler to native binaries
- **`compiler/codegen.py`**: LLVM IR generation from AST

### Language Design

**Distinctive Syntax Elements:**

- **Brackets for function calls**: `print["hello"]`, `add[x, y]`
- **Block delimiters**: `| ... ^` instead of `{ ... }`
- **Assignment operators**: `:=` (mutable), `::` (constants)
- **AI/ML operators**: `@` (dot product), `∇[loss, params]` (gradients)

**Type System:**

- Primitive: `int`, `float`, `bool`, `string`
- AI-focused: `tensor`, `matrix`, `vector`, `dataset`
- Type inference with optional annotations: `let x: int := 42`

### Parser Architecture

The parser uses recursive descent with precedence climbing for expressions:

- **TokenType enum**: Defines all language tokens in `compiler/parser/lexer.py`
- **AST nodes**: Visitor pattern implementation in `compiler/ast/nodes.py`
- **Expression precedence**: Handles operator precedence (logical OR → AND → equality → comparison → term → factor → unary → primary)

### Runtime System

**Interpreter Runtime** (`CortexRuntime`):

- Variable/constant scopes with built-in function support
- Built-ins: `print`, `str`, `len`, `range`
- Function call stack management with parameter binding

**Compiler Runtime**:

- LLVM IR generation with optimization passes
- Native code generation with GCC linking
- Support for math library integration (`-lm`)

## Important Files and Directories

### Core Implementation

- `cortexc.py`: Main entry point and CLI
- `compiler/compiler.py`: LLVM compilation pipeline
- `compiler/parser/parser.py`: Recursive descent parser
- `compiler/ast/nodes.py`: AST node definitions
- `compiler/interpreter.py`: Direct AST execution

### Examples and Testing

- `examples/*.ctx`: Working Cortex programs demonstrating syntax
- `test_compiler.py`: Compiler functionality validation
- `docs/`: Comprehensive language documentation

### Configuration

- `requirements.txt`: Python dependencies including `llvmlite`, `numpy`
- System requirements: Python 3.8+, GCC/Clang, LLVM 12+

## Language-Specific Notes

### Syntax Patterns

```cortex
// Function definitions
func name[param1, param2] |
  // body
^

// Variable assignment
let x := value          // mutable
let CONST :: value      // immutable

// Control flow
if [condition] |
  // then block
^ else |
  // else block
^

while [condition] |
  // loop body
^
```

### Built-in Functions

- `print[value]`: Output to console
- `str[value]`: Convert to string
- `len[collection]`: Get length
- `range[start, stop, step]`: Generate ranges

### AI/ML Extensions (Planned)

- Tensor operations: `a @ b` (matrix multiply), `∇[f, x]` (gradient)
- Dataset primitives: `read["data.csv"]`, `split[data, ratio]`
- Model definitions: Object-oriented approach with training loops

## Development Guidelines

### Parser Extensions

When adding new syntax:

1. Add token types to `TokenType` enum in `lexer.py`
2. Implement lexing rules in `Lexer.tokenize()`
3. Add AST nodes in `ast/nodes.py` with visitor methods
4. Implement parsing logic in `parser.py`
5. Add interpreter support in `interpreter.py`
6. Add LLVM codegen in `codegen.py`

### Testing New Features

1. Write example `.ctx` files in `examples/`
2. Add test cases to `test_compiler.py`
3. Test both interpreter and compiler modes
4. Verify LLVM IR generation and binary execution

### Error Handling

- Parser errors include line/column information
- Runtime errors provide context and variable state
- Compiler errors show LLVM compilation issues

The codebase is designed for extensibility, with clear separation between parsing, interpretation, and compilation phases.
