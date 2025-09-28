# Running Cortex Programs

This guide covers how to compile and run Cortex programs, from simple examples to complex applications. It includes both current working capabilities and planned future features.

## Table of Contents

1. [Current Working Features](#current-working-features)
2. [Planned Future Features](#planned-future-features)
3. [Troubleshooting](#troubleshooting)
4. [Performance](#performance)

---

## Current Working Features

### ✅ Prerequisites

#### System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Python**: 3.8 or higher
- **C Compiler**: GCC, Clang, or MSVC (for linking)
- **LLVM**: 12.0 or higher (for code generation)

#### Installation

```bash
# Clone the repository
git clone https://github.com/yourname/cortex
cd cortex

# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install Python dependencies
pip install llvmlite

# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install build-essential llvm-12-dev

# Install system dependencies (macOS)
brew install llvm

# Install system dependencies (Windows)
# Download and install LLVM from https://llvm.org/releases/
```

### ✅ Running Methods

#### Method 1: Interpreter Mode

Run Cortex programs directly using the interpreter:

```bash
# Run a program using the interpreter
python3 cortexc.py run examples/hello_world.ctx

# Expected output:
# Hello, World!
# Welcome to Cortex - the AI/ML programming language!
```

#### Method 2: Interactive REPL

Start an interactive Cortex session:

```bash
# Start interactive session
python3 cortexc.py repl

# Example session:
# cortex> let x := 42
# cortex> print[x]
# 42.000000
# cortex> let y := x + 8
# cortex> print[y]
# 50.000000
# cortex> exit
```

#### Method 3: Compile to Binary

Compile Cortex programs to native executables:

```bash
# Compile a Cortex program to binary
python3 cortexc.py build examples/hello_world.ctx -o hello

# Run the compiled executable
./hello
```

### ✅ Compilation Process

The Cortex compiler follows this pipeline:

1. **Parse**: Convert `.ctx` source to Abstract Syntax Tree (AST)
2. **Generate LLVM IR**: Convert AST to LLVM Intermediate Representation
3. **Compile to Object**: Use `llc` to compile LLVM IR to object file
4. **Link**: Use `gcc` to link object file to executable

```bash
# Manual compilation steps (what cortexc.py build does automatically)
python3 cortexc.py build examples/hello_world.ctx -o hello_world

# This generates:
# 1. hello_world.ll (LLVM IR)
# 2. hello_world.o (object file) 
# 3. hello_world (executable)
```

### ✅ Compilation Options

```bash
python3 cortexc.py build [options] <input_file> -o <output_file>

Options:
  -O0, -O1, -O2, -O3    Optimization level (default: -O1)
  -g                     Include debug information (planned)
  -v                     Verbose output
  --help                 Show help message
```

### ✅ Working Examples

#### Example 1: Hello World

**File**: `examples/hello_world.ctx`

```cortex
// Hello World in Cortex
func main[] |
  print["Hello, World!"]
  print["Welcome to Cortex - the AI/ML programming language!"]
^

main[]
```

**Compile and Run**:

```bash
python3 cortexc.py build examples/hello_world.ctx -o hello
./hello
```

**Output**:

``` txt
Hello, World!
Welcome to Cortex - the AI/ML programming language!
```

#### Example 2: Arithmetic Operations

**File**: `examples/simple_arithmetic.ctx`

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

**Compile and Run**:

```bash
python3 cortexc.py build examples/simple_arithmetic.ctx -o arithmetic
./arithmetic
```

**Output**:

``` txt
13.000000
7.000000
30.000000
3.333333
```

#### Example 3: Loops and Functions

**File**: `examples/simple_loops.ctx`

```cortex
func main[] |
  // While loop example - count from 1 to 5
  let counter := 1
  
  while [counter <= 5] |
    print[counter]
    let counter := counter + 1
  ^
  
  // Mathematical loop - factorial
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

**Compile and Run**:

```bash
python3 cortexc.py build examples/simple_loops.ctx -o loops
./loops
```

**Output**:

``` txt
1.000000
2.000000
3.000000
4.000000
5.000000
120.000000
```

#### Example 4: Functions

**File**: `examples/function_demo.ctx`

```cortex
func add[a, b] |
  return[a + b]
^

func factorial[n] |
  if [n <= 1] |
    return[1]
  ^
  return[n * factorial[n - 1]]
^

func main[] |
  let sum := add[10, 20]
  print[sum]
  
  let fact := factorial[5]
  print[fact]
^

main[]
```

**Compile and Run**:

```bash
python3 cortexc.py build examples/function_demo.ctx -o functions
./functions
```

**Output**:

``` txt
30.000000
120.000000
```

### ✅ Command Reference

#### cortexc.py run

Run a Cortex program using the interpreter:

```bash
python3 cortexc.py run <file.ctx> [options]

Options:
  -v, --verbose    Verbose output
```

#### cortexc.py build

Compile a Cortex program to binary:

```bash
python3 cortexc.py build <file.ctx> -o <output> [options]

Options:
  -o, --output     Output executable name (required)
  -O, --optimize   Optimization level (0-3, default: 1)
  -v, --verbose    Verbose output
```

#### cortexc.py repl

Start interactive REPL:

```bash
python3 cortexc.py repl [options]

Options:
  -v, --verbose    Verbose output
```

---

## Planned Future Features

### 🔄 Advanced Compilation

#### Just-In-Time (JIT) Compilation

```bash
# JIT compilation for faster development cycles
python3 cortexc.py jit examples/neural_network.ctx

# Hot reloading during development
python3 cortexc.py dev examples/experiment.ctx --watch
```

#### Cross-Compilation

```bash
# Compile for different architectures
python3 cortexc.py build program.ctx -o program --target x86_64-linux-gnu
python3 cortexc.py build program.ctx -o program --target arm64-apple-darwin
python3 cortexc.py build program.ctx -o program --target aarch64-linux-gnu
```

#### WebAssembly Compilation

```bash
# Compile to WebAssembly for browser execution
python3 cortexc.py build program.ctx -o program.wasm --target wasm32-unknown-unknown
```

### 🔄 Advanced Debugging

#### Debug Mode

```bash
# Compile with debug information
python3 cortexc.py build program.ctx -o program -g

# Run with debugger
gdb ./program
lldb ./program  # macOS
```

#### Profiling Support

```bash
# Compile with profiling information
python3 cortexc.py build program.ctx -o program --profile

# Profile execution
python3 cortexc.py profile program.ctx
```

#### Interactive Debugging

```bash
# Start debug session
python3 cortexc.py debug program.ctx

# Debug commands:
# (debug) break main
# (debug) step
# (debug) print variable_name
# (debug) continue
```

### 🔄 Package Management

#### Package Installation

```bash
# Install packages from registry
cortex install numpy
cortex install matplotlib
cortex install tensorflow

# Install from Git repository
cortex install github.com/user/package
```

#### Package Creation

```bash
# Create new package
cortex init my-package

# Build and publish package
cortex build my-package
cortex publish my-package
```

#### Dependency Management

```cortex
// cortex.toml
[package]
name = "my-project"
version = "1.0.0"

[dependencies]
numpy = "1.21.0"
matplotlib = "3.5.0"
tensorflow = "2.8.0"

[dev-dependencies]
cortex-test = "0.1.0"
```

### 🔄 Testing Framework

#### Unit Testing

```bash
# Run tests
cortex test

# Run specific test file
cortex test tests/test_arithmetic.ctx

# Run with coverage
cortex test --coverage
```

#### Test File Example

```cortex
// tests/test_arithmetic.ctx
import [cortex.test]

func test_addition[] |
  assert[add[2, 3] == 5]
  assert[add[-1, 1] == 0]
^

func test_factorial[] |
  assert[factorial[0] == 1]
  assert[factorial[5] == 120]
^

// Run all tests
test_addition[]
test_factorial[]
```

### 🔄 Build Systems Integration

#### Makefile Integration

```makefile
# Makefile for Cortex project
CORTEXC = python3 cortexc.py
CFLAGS = -O2
SOURCES = $(wildcard src/*.ctx)
TARGETS = $(SOURCES:.ctx=)

all: $(TARGETS)

%: %.ctx
 $(CORTEXC) build $< -o $@ $(CFLAGS)

test: $(TARGETS)
 cortex test

clean:
 rm -f $(TARGETS) *.ll *.o

.PHONY: all test clean
```

#### CMake Integration

```cmake
# CMakeLists.txt
cmake_minimum_required(VERSION 3.16)
project(cortex_project)

find_program(CORTEXC cortexc.py)

add_custom_target(cortex_build
  COMMAND ${CORTEXC} build src/main.ctx -o main -O2
  DEPENDS src/main.ctx
)

add_custom_target(cortex_test
  COMMAND ${CORTEXC} test
  DEPENDS cortex_build
)
```

#### Docker Integration

```dockerfile
# Dockerfile
FROM python:3.9-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    llvm-12-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Cortex
COPY . /app
WORKDIR /app

# Create virtual environment
RUN python3 -m venv venv
RUN venv/bin/pip install llvmlite

# Build Cortex project
RUN venv/bin/python cortexc.py build src/main.ctx -o main -O2

# Run the application
CMD ["./main"]
```

### 🔄 IDE Integration

#### VS Code Extension

```json
// .vscode/settings.json
{
  "cortex.compiler.path": "python3 cortexc.py",
  "cortex.build.command": "build",
  "cortex.test.command": "test",
  "files.associations": {
    "*.ctx": "cortex"
  }
}
```

#### Language Server Protocol

```bash
# Start Cortex language server
cortex-lsp --stdio

# Features:
# - Syntax highlighting
# - Error checking
# - Auto-completion
# - Go to definition
# - Hover information
```

### 🔄 Continuous Integration

#### GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Setup Python
      uses: actions/setup-python@v2
      with:
        python-version: 3.9
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install llvm-12-dev
        pip install llvmlite
    
    - name: Build project
      run: python3 cortexc.py build src/main.ctx -o main -O2
    
    - name: Run tests
      run: python3 cortexc.py test
    
    - name: Run benchmarks
      run: python3 cortexc.py benchmark
```

#### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - deploy

build:
  stage: build
  script:
    - apt-get update && apt-get install -y llvm-12-dev
    - pip install llvmlite
    - python3 cortexc.py build src/main.ctx -o main -O2
  artifacts:
    paths:
      - main

test:
  stage: test
  script:
    - python3 cortexc.py test
    - python3 cortexc.py benchmark
  dependencies:
    - build
```

---

## Troubleshooting

### Common Issues

#### 1. LLVM Not Found

**Error**: `LLVM not found` or `Unable to find target for this triple`

**Solution**:

```bash
# Install LLVM
brew install llvm                    # macOS
sudo apt-get install llvm-12-dev     # Ubuntu/Debian

# Set PATH (macOS with Homebrew)
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
```

#### 2. Python Import Errors

**Error**: `No module named 'llvmlite'`

**Solution**:

```bash
# Make sure you're in the virtual environment
source venv/bin/activate

# Install llvmlite
pip install llvmlite
```

#### 3. Compilation Errors

**Error**: `Syntax error in file`

**Common Causes**:

- Missing `^` to close blocks
- Using `()` instead of `[]` for function calls
- Incorrect variable assignment syntax

**Solution**: Check Cortex syntax:

- Use `|` and `^` for blocks
- Use `[]` for function calls and parameters
- Use `:=` for variable assignment

#### 4. Runtime Errors

**Error**: `Segmentation fault` or `Runtime error`

**Solution**:

- Check for division by zero
- Verify all variables are initialized
- Use the interpreter mode for debugging: `python3 cortexc.py run file.ctx`

 Debug Mode

For debugging, use the interpreter mode which provides better error messages:

```bash
# Run with interpreter (better error messages)
python3 cortexc.py run examples/hello_world.ctx

# Check the generated LLVM IR
python3 cortexc.py build examples/hello_world.ctx -o hello -v
cat hello.ll
```

### Verbose Output

Get detailed compilation information:

```bash
# Verbose compilation
python3 cortexc.py build examples/hello_world.ctx -o hello -v

# This will show:
# - LLVM IR generation
# - Compilation steps
# - Any warnings or errors
```

---

## Performance

### Current Optimization Levels

```bash
# No optimization (fastest compilation)
python3 cortexc.py build program.ctx -o program -O0

# Basic optimization (default)
python3 cortexc.py build program.ctx -o program -O1

# Aggressive optimization
python3 cortexc.py build program.ctx -o program -O2

# Maximum optimization
python3 cortexc.py build program.ctx -o program -O3
```

### Benchmarking

Compare interpreter vs compiled performance:

```bash
# Time interpreter execution
time python3 cortexc.py run examples/simple_loops.ctx

# Time compiled execution
python3 cortexc.py build examples/simple_loops.ctx -o loops
time ./loops
```

### Planned Performance Features

#### Advanced Optimization

```bash
# Profile-guided optimization
python3 cortexc.py build program.ctx -o program --pgo

# Link-time optimization
python3 cortexc.py build program.ctx -o program --lto

# Auto-vectorization
python3 cortexc.py build program.ctx -o program --vectorize
```

#### Parallel Compilation

```bash
# Compile multiple files in parallel
python3 cortexc.py build --parallel src/*.ctx

# Use multiple cores for optimization
python3 cortexc.py build program.ctx -o program --jobs 8
```

#### Caching

```bash
# Enable compilation cache
python3 cortexc.py build program.ctx -o program --cache

# Clear cache
python3 cortexc.py clean --cache
```

---

## Integration with Build Systems

### Makefile Example

```makefile
# Makefile for Cortex project
CORTEXC = python3 cortexc.py
CFLAGS = -O2
SOURCES = $(wildcard examples/*.ctx)
TARGETS = $(SOURCES:.ctx=)

all: $(TARGETS)

%: %.ctx
 $(CORTEXC) build $< -o $@ $(CFLAGS)

clean:
 rm -f $(TARGETS) *.ll *.o

.PHONY: all clean
```

### Shell Script Example

```bash
#!/bin/bash
# build_all.sh - Build all Cortex examples

echo "Building all Cortex examples..."

for file in examples/*.ctx; do
    if [ -f "$file" ]; then
        name=$(basename "$file" .ctx)
        echo "Building $name..."
        python3 cortexc.py build "$file" -o "$name" -O2
    fi
done

echo "Build complete!"
```

---

## Next Steps

1. **Explore Examples**: Try the examples in the `examples/` directory
2. **Read Documentation**: Check out `docs/syntax.md` for language details
3. **Write Your Own**: Create your first Cortex program
4. **Compile to Binary**: Use `cortexc.py build` to create executables
5. **Join Community**: Contribute to the project on GitHub

## Getting Help

- **Documentation**: [docs/](docs/)
- **Examples**: [examples/](examples/)
- **Issues**: [GitHub Issues](https://github.com/yourname/cortex/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourname/cortex/discussions)

For more information, visit the [Cortex documentation](docs/) or check out the [examples](examples/).
