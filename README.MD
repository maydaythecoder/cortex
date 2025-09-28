# Cortex 🧠

**Cortex** is a **compiled programming language designed for AI and machine learning**.  
Its goal is to make building, training, and experimenting with models **simpler and faster** by combining:  

- **Readable, bracket-based syntax** → focused on clarity and flow  
- **Compiled performance (LLVM)** → optimized for numerical computing  
- **AI-first primitives** → tensors, datasets, models, training utilities  

Instead of writing AI in bloated frameworks, Cortex gives you a **language designed from the ground up for machine intelligence**.  

---

## ✨ Why Cortex?

- **AI-first design** → control flow, data structures, and math built with ML in mind  
- **Concise syntax** → `if [x > 5] | print[x] ^`  
- **Compiled performance** → safe, static, and fast  
- **Integrated ML stack** → arrays → tensors, objects → models, dicts → datasets  
- **Extendable** → pluggable backend (CPU now, GPU later)  

---

## 🚀 Quick Start

### Prerequisites

**Python Implementation:**

- Python 3.8+
- Git
- GCC or Clang (for binary compilation)
- LLVM (will be installed via pip)

**Rust Implementation:**

- Rust 1.70+
- Git
- Cargo (comes with Rust)

### Installation

**Python Implementation:**

```bash
git clone https://github.com/yourname/cortex
cd cortex
cd python
python3 -m venv venv
source venv/bin/activate
pip install llvmlite
```

**Rust Implementation:**

```bash
git clone https://github.com/yourname/cortex
cd cortex/rust
cargo build --release
```

### Hello World

**Python Implementation:**

```bash
# Create hello_world.ctx
echo 'print["Hello, Cortex!"]' > hello_world.ctx

# Run it (interpreted)
cd python
python3 cortexc.py run hello_world.ctx

# Compile to binary executable
python3 cortexc.py build hello_world.ctx -o hello

# Run the compiled executable
./hello
```

**Rust Implementation:**

```bash
# Create hello_world.ctx
echo 'print["Hello, Cortex!"]' > hello_world.ctx

# Run it (interpreted)
cd rust
cargo run -- run hello_world.ctx

# Check syntax
cargo run -- check hello_world.ctx
```

---

## 🎉 **WORKING COMPILER: Binary Compilation Available!**

Cortex now has a **fully functional compiler** that generates binary executables using LLVM!

### ✅ **Currently Working Features**

- **🔥 Binary Compilation**: Complete compilation pipeline from `.ctx` to executable
- **🔥 LLVM IR Generation**: Full AST to LLVM IR translation with proper target detection
- **🔥 Arithmetic Operations**: All math operations (+, -, *, /, %, **) with correct type handling
- **🔥 String & Number Printing**: Proper printf formatting for both strings and numbers
- **🔥 While Loops**: Complete loop implementation with condition evaluation
- **🔥 Functions**: Function definitions, calls, and parameter handling
- **🔥 Variables**: Variable assignment, scoping, and expression evaluation
- **🔥 Cross-Platform**: Automatic target detection (macOS ARM64/x86_64, Linux)

### 🚀 **Working Examples**

```bash
# Compile and run arithmetic operations
python3 cortexc.py build examples/simple_arithmetic.ctx -o arithmetic
./arithmetic
# Output: 13.000000, 7.000000, 30.000000, 3.333333

# Compile and run loops with factorial
python3 cortexc.py build examples/simple_loops.ctx -o loops  
./loops
# Output: 1.000000, 2.000000, 3.000000, 4.000000, 5.000000, 120.000000

# Compile and run hello world
python3 cortexc.py build examples/hello_world.ctx -o hello
./hello
# Output: Hello, World!
#         Welcome to Cortex - the AI/ML programming language!
```

---

## 📖 Example Code

### Basic Variables and Functions

```cortex
let greeting := "Hello"

func add[a, b] |
  return[a + b]
^

func main[] |
  print[greeting + " from Cortex!"]
  let sum := add[10, 20]
  print["Result: " + str[sum]]
^

main[]
```

### Working Arithmetic Example

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

### Working Loop Example

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

---

## 🧩 Core Features (Currently Implemented)

### ✅ **Working Language Features**

- **Variables**: `let x := 42` with type inference
- **Functions**: `func name[params] | body ^` with parameter handling
- **Arithmetic**: All basic operations (+, -, *, /, %, **)
- **Printing**: `print[value]` for strings and numbers
- **Loops**: `while [condition] | body ^` with proper condition evaluation
- **Control Flow**: `if [condition] | body ^` with else support
- **Comments**: `//` single-line and `/* */` multi-line

### 🔄 **In Development**

- **Arrays**: Basic array data structures
- **String Concatenation**: `"hello" + "world"`
- **For Loops**: `for [i in range[10]] | body ^`
- **Advanced Types**: Enhanced type system

### 📋 **Planned Features**

- **Numerical computing**: tensors, vectors, matrices
- **Data structures**: Arrays → tensors, Dictionaries → datasets
- **Built-in ML operators**: `∇[loss, params]`, `@` (dot product), `⊗` (outer product)
- **GPU compilation support**: CUDA/OpenCL backends
- **Package system**: Import and modules

---

## 📂 Project Structure

``` txt
cortex/
├─ python/          # Python implementation
│  ├─ compiler/     # Python-based compiler
│  │  ├─ parser/    # Lexer and parser
│  │  ├─ ast/       # Abstract syntax tree
│  │  ├─ codegen.py # LLVM IR generation
│  │  ├─ compiler.py# Binary compilation pipeline
│  │  └─ interpreter.py
│  ├─ cortexc.py    # Python command-line tool
│  ├─ test_compiler.py
│  └─ venv/         # Python virtual environment
├─ rust/            # Rust implementation
│  ├─ src/          # Rust source code
│  │  ├─ main.rs    # CLI interface
│  │  ├─ lexer.rs   # Tokenizer
│  │  ├─ parser.rs  # Parser
│  │  ├─ ast.rs     # AST definitions
│  │  ├─ codegen.rs # Interpreter
│  │  └─ error.rs   # Error handling
│  ├─ Cargo.toml    # Rust dependencies
│  └─ target/       # Rust build artifacts
├─ docs/            # Documentation
│  ├─ shared/       # Shared documentation
│  │  ├─ examples/  # Example .ctx files
│  │  ├─ syntax.md
│  │  ├─ functions.md
│  │  └─ *.md       # Core documentation
│  ├─ python/       # Python-specific docs
│  └─ rust/         # Rust-specific docs
└─ requirements.txt # Python dependencies
```

---

## 🏃‍♂️ Running Programs

### Python Implementation Commands

```bash
cd python

# Run a program (interpreter mode)
python3 cortexc.py run program.ctx

# Interactive REPL
python3 cortexc.py repl

# Build executable (WORKING! - LLVM backend)
python3 cortexc.py build program.ctx -o program

# Build with optimization
python3 cortexc.py build program.ctx -o program -O 2
```

### Rust Implementation Commands

```bash
cd rust

# Run a program (interpreter mode)
cargo run -- run program.ctx

# Check syntax
cargo run -- check program.ctx

# Build executable
cargo run -- build program.ctx -o program
```

### Try the Working Examples

**Python Implementation:**

```bash
cd python

# Basic hello world
python3 cortexc.py run ../docs/shared/examples/hello_world.ctx

# Functions and variables
python3 cortexc.py run ../docs/shared/examples/function_demo.ctx

# NEW: Compile examples to binary
python3 cortexc.py build ../docs/shared/examples/simple_arithmetic.ctx -o arithmetic
python3 cortexc.py build ../docs/shared/examples/simple_loops.ctx -o loops
python3 cortexc.py build ../docs/shared/examples/hello_world.ctx -o hello

# Run compiled binaries
./arithmetic
./loops
./hello
```

**Rust Implementation:**

```bash
cd rust

# Basic hello world
cargo run -- run ../docs/shared/examples/hello_world.ctx

# Functions and variables
cargo run -- run ../docs/shared/examples/function_demo.ctx

# Check syntax
cargo run -- check ../docs/shared/examples/hello_world.ctx
```

---

## 🛠️ Development Status

- [x] **Core Language**: Variables, functions, control flow
- [x] **Basic Interpreter**: Run Cortex programs directly
- [x] **Standard Library**: Math, string operations
- [x] **LLVM Backend**: ✅ **COMPILER IMPLEMENTED!** Compile to machine code
- [x] **Binary Compilation**: Generate executable files with `cortexc build`
- [ ] **Tensor Operations**: Full numerical computing stack
- [ ] **GPU Support**: CUDA/OpenCL backends
- [ ] **Package System**: Import and modules

---

## 🚀 Roadmap

### Phase 1: Core Language ✅ COMPLETED

- [x] Lexer and parser
- [x] AST and interpreter
- [x] Basic syntax and operations
- [x] Function calls and control flow
- [x] **LLVM compilation backend** ✅ **COMPLETED!**
- [x] **Binary executable generation** ✅ **COMPLETED!**

### Phase 2: AI/ML Features (In Progress)

- [ ] Tensor data types and operations
- [ ] Automatic differentiation
- [ ] Neural network primitives
- [ ] Training loop utilities

### Phase 3: Performance ✅ PARTIALLY COMPLETE

- [x] **LLVM compilation backend** ✅ **COMPLETED!**
- [x] **Binary executable generation** ✅ **COMPLETED!**
- [ ] Advanced memory optimization
- [ ] Parallel execution
- [ ] GPU acceleration

### Phase 4: Ecosystem

- [ ] Package manager
- [ ] Standard ML library
- [ ] IDE support and debugging
- [ ] Community and documentation

---

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/new-feature`
3. **Make your changes** and add tests
4. **Run the test suite**: `python3 -m pytest tests/`
5. **Submit a pull request**

### Development Setup

**Python Implementation:**

```bash
git clone https://github.com/yourname/cortex
cd cortex/python
python3 -m venv venv
source venv/bin/activate
pip install llvmlite

# Run tests
python3 test_compiler.py

# Run examples
python3 cortexc.py run ../docs/shared/examples/hello_world.ctx

# Compile examples
python3 cortexc.py build ../docs/shared/examples/hello_world.ctx -o hello
```

**Rust Implementation:**

```bash
git clone https://github.com/yourname/cortex
cd cortex/rust
cargo build

# Run examples
cargo run -- run ../docs/shared/examples/hello_world.ctx

# Check syntax
cargo run -- check ../docs/shared/examples/hello_world.ctx
```

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🔗 Links

- **Documentation**: [docs/shared/](docs/shared/)
- **Examples**: [docs/shared/examples/](docs/shared/examples/)  
- **Python Implementation**: [python/](python/)
- **Rust Implementation**: [rust/](rust/)
- **Issues**: [GitHub Issues](https://github.com/yourname/cortex/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourname/cortex/discussions)

---

**Ready to try Cortex?** Start with our [Getting Started Guide](docs/shared/run.md) and explore the [examples](docs/shared/examples/)!

**Both implementations are working!** Try the Python compiler for LLVM binary generation or the Rust interpreter for fast development! 🚀
