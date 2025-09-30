# ![Cortex Logo](assets/cortex-with-text.svg)

A high-performance AI/ML programming language built in Rust

**Cortex** is a **compiled programming language designed for AI and machine learning**.  
Its goal is to make building, training, and experimenting with models **simpler and faster** by combining:  

- **Readable, bracket-based syntax** → focused on clarity and flow  
- **Rust-powered performance** → memory-safe and blazingly fast  
- **AI-first primitives** → tensors, datasets, models, training utilities  
- **Professional tooling** → CLI, VSCode extension, language server, debugger

Instead of writing AI in bloated frameworks, Cortex gives you a **language designed from the ground up for machine intelligence**.  

---

## ✨ Why Cortex?

- **AI-first design** → control flow, data structures, and math built with ML in mind  
- **Concise syntax** → `if [x > 5] | print[x] ^`  
- **Rust-powered** → memory-safe, zero-cost abstractions, and blazingly fast  
- **Integrated ML stack** → arrays → tensors, objects → models, dicts → datasets  
- **Professional tooling** → Complete development environment with debugging, IntelliSense, and testing
- **Extendable** → pluggable backend (CPU now, GPU later)  

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Node.js 16+ (for VSCode extension)
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/muhyadinmohamed/cortex
cd cortex

# Install dependencies
npm install

# Build all components
npm run build
```

### Hello World

```bash
# Create a new project
cortex new hello-world
cd hello-world

# Run the program
cortex run src/main.ctx

# Or run directly
cortex run examples/hello_world.ctx
```

---

## 🎉 **RUST IMPLEMENTATION: High-Performance Interpreter!**

Cortex now has a **fully functional Rust interpreter** with syntax highlighting and formatting support!

### ✅ **Currently Working Features**

- **🔥 High-Performance Interpreter**: Rust-powered execution with memory safety
- **🔥 Syntax Highlighting**: Beautiful VS Code extension with custom color palette
- **🔥 Code Formatting**: Automatic formatting with configurable indentation
- **🔥 Arithmetic Operations**: All math operations (+, -, *, /, %, **) with correct type handling
- **🔥 String & Number Printing**: Proper output formatting for both strings and numbers
- **🔥 While Loops**: Complete loop implementation with condition evaluation
- **🔥 Functions**: Function definitions, calls, and parameter handling
- **🔥 Variables**: Variable assignment, scoping, and expression evaluation
- **🔥 Cross-Platform**: Works on macOS, Linux, and Windows

### 🚀 **Working Examples**

```bash
# Run arithmetic operations
cd rust
cargo run -- run ../examples/arithmetic.ctx
# Output: Arithmetic Operations with all calculations

# Run loops with factorial
cargo run -- run ../examples/simple_loops.ctx
# Output: Loop iterations and factorial calculation

# Run hello world
cargo run -- run ../examples/hello_world.ctx
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
├─ packages/                    # Core packages
│  ├─ compiler/                # Rust compiler/interpreter
│  │  ├─ src/                 # Source code
│  │  │  ├─ main.rs           # CLI interface
│  │  │  ├─ lexer.rs          # Tokenizer
│  │  │  ├─ parser.rs         # Parser
│  │  │  ├─ ast.rs            # AST definitions
│  │  │  ├─ codegen.rs        # Interpreter
│  │  │  ├─ error.rs          # Error handling
│  │  │  └─ debugger.rs       # Debugger
│  │  └─ Cargo.toml           # Rust dependencies
│  ├─ cli/                     # Command-line interface
│  │  ├─ src/                 # CLI source code
│  │  └─ Cargo.toml           # CLI dependencies
│  ├─ language-server/         # Language Server Protocol
│  │  ├─ src/                 # LSP implementation
│  │  └─ Cargo.toml           # LSP dependencies
│  └─ vscode-extension/        # VSCode extension
│     ├─ src/                 # Extension source (TypeScript)
│     ├─ package.json         # Extension manifest
│     └─ syntaxes/            # Syntax highlighting
├─ tools/                      # Development tools
│  ├─ formatter/              # Code formatter
│  ├─ debugger/               # Debugging tools
│  └─ linter/                 # Code linter
├─ docs/                       # Documentation
│  ├─ api/                    # API reference
│  ├─ examples/               # Example programs
│  └─ tutorials/              # Step-by-step guides
├─ tests/                      # Test suites
│  ├─ unit/                   # Unit tests
│  ├─ integration/            # Integration tests
│  └─ benchmarks/             # Performance benchmarks
├─ assets/                     # Logos and branding
├─ .github/                    # GitHub workflows
└─ package.json               # Root package configuration
```

---

## 🏃‍♂️ Running Programs

### Rust Implementation Commands

```bash
# Navigate to the rust directory
cd rust

# Run a program (interpreter mode)
cargo run -- run program.ctx

# Check syntax
cargo run -- check program.ctx

# Build executable
cargo run -- build program.ctx -o program
```

### Running Examples from Project Root

```bash
# From the project root directory
cd /path/to/cortex

# Run any example
cd rust && cargo run -- run ../examples/hello_world.ctx

# Or use the provided shell script
./run_cortex.sh examples/hello_world.ctx
```

### VS Code Integration

If you have the Cortex VS Code extension installed:

1. **Open any `.ctx` file** in VS Code
2. **Press F5** or use "Run and Debug" to execute the file
3. **Use Ctrl+Shift+P** and search for "Cortex" commands
4. **Format on save** is automatically enabled for `.ctx` files

### Try the Working Examples

**Shorthand Commands (Recommended):**

```bash
# From project root - use the shorthand scripts
./ctx hello_world
./ctx function_demo
./ctx arithmetic
./ctx simple_algorithms
./ctx fibonacci
./ctx factorial
./ctx prime_numbers
./ctx bubble_sort
./ctx comprehensive_features
./ctx ai_ml_comprehensive
./ctx simple_loops
./ctx loops
./ctx printing
./ctx simple_arithmetic

# Check syntax
./ctxc hello_world

# Run any .ctx file
./ctx run examples/hello_world.ctx
./ctxc check examples/hello_world.ctx
```

**Full Commands:**

```bash
# Navigate to the rust directory
cd rust

# Basic hello world
cargo run -- run ../examples/hello_world.ctx

# Functions and variables
cargo run -- run ../examples/function_demo.ctx

# Arithmetic operations
cargo run -- run ../examples/arithmetic.ctx

# Simple algorithms (prime, factorial, fibonacci, GCD)
cargo run -- run ../examples/simple_algorithms.ctx

# Fibonacci sequence
cargo run -- run ../examples/fibonacci.ctx

# Factorial calculation
cargo run -- run ../examples/factorial.ctx

# Prime number checking
cargo run -- run ../examples/prime_numbers.ctx

# Bubble sort demo
cargo run -- run ../examples/bubble_sort.ctx

# Comprehensive features showcase
cargo run -- run ../examples/comprehensive_features.ctx

# AI/ML algorithms demo
cargo run -- run ../examples/ai_ml_comprehensive.ctx

# Loop examples
cargo run -- run ../examples/simple_loops.ctx
cargo run -- run ../examples/loops.ctx

# Printing examples
cargo run -- run ../examples/printing.ctx

# Simple arithmetic
cargo run -- run ../examples/simple_arithmetic.ctx

# Check syntax of any example
cargo run -- check ../examples/hello_world.ctx
```

---

## 🛠️ Development Status

- [x] **Core Language**: Variables, functions, control flow
- [x] **Rust Interpreter**: High-performance execution with memory safety
- [x] **VS Code Extension**: Syntax highlighting and code formatting
- [x] **Standard Library**: Math, string operations
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
- [x] **Rust interpreter** ✅ **COMPLETED!**
- [x] **VS Code extension** ✅ **COMPLETED!**

### Phase 2: AI/ML Features (In Progress)

- [ ] Tensor data types and operations
- [ ] Automatic differentiation
- [ ] Neural network primitives
- [ ] Training loop utilities

### Phase 3: Performance (In Progress)

- [x] **Rust interpreter** ✅ **COMPLETED!**
- [x] **Memory safety** ✅ **COMPLETED!**
- [ ] Advanced memory optimization
- [ ] Parallel execution
- [ ] GPU acceleration

### Phase 4: Ecosystem

- [ ] Package manager
- [ ] Standard ML library
- [ ] Enhanced IDE support and debugging
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

**Rust Implementation:**

```bash
git clone https://github.com/yourname/cortex
cd cortex/rust
cargo build

# Run examples
cargo run -- run ../examples/hello_world.ctx

# Check syntax
cargo run -- check ../examples/hello_world.ctx
```

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🔗 Links

- **Documentation**: [docs/shared/](docs/shared/)
- **Examples**: [docs/shared/examples/](docs/shared/examples/)  
- **Function Status**: [docs/shared/FUNCTION_STATUS.md](docs/shared/FUNCTION_STATUS.md)
- **Python Implementation**: [python/](python/)
- **Rust Implementation**: [rust/](rust/)
- **VS Code Marketplace**: [Cortex Language Extension](https://marketplace.visualstudio.com/items?itemName=muhyadinmohamed.cortex-language)
- **Issues**: [GitHub Issues](https://github.com/yourname/cortex/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourname/cortex/discussions)

---

**Ready to try Cortex?** Start with our [Getting Started Guide](docs/shared/run.md) and explore the [examples](examples/)!

**Rust implementation is ready!** High-performance interpreter with VS Code support! 🚀
