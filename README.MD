# Cortex 🧠

![Cortex Logo](assets/cortex-with-text.svg)

A high-performance AI/ML programming language built in Rust

**Cortex** is a **compiled programming language designed for AI and machine learning**.  
Its goal is to make building, training, and experimenting with models **simpler and faster** by combining:  

- **Readable, bracket-based syntax** → focused on clarity and flow  
- **Rust-powered performance** → memory-safe and blazingly fast  
- **AI-first primitives** → tensors, datasets, models, training utilities  

Instead of writing AI in bloated frameworks, Cortex gives you a **language designed from the ground up for machine intelligence**.  

---

## ✨ Why Cortex?

- **AI-first design** → control flow, data structures, and math built with ML in mind  
- **Concise syntax** → `if [x > 5] | print[x] ^`  
- **Rust-powered** → memory-safe, zero-cost abstractions, and blazingly fast  
- **Integrated ML stack** → arrays → tensors, objects → models, dicts → datasets  
- **Extendable** → pluggable backend (CPU now, GPU later)  

---

## 🚀 Quick Start

### Prerequisites

**Rust Implementation (Recommended):**

- Rust 1.70+
- Git
- Cargo (comes with Rust)

### Installation

**Rust Implementation:**

```bash
git clone https://github.com/yourname/cortex
cd cortex/rust
cargo build --release
```

### Hello World

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
├─ cortex-vscode-extension/ # VS Code extension
│  ├─ package.json  # Extension manifest
│  ├─ extension.js  # Extension logic
│  ├─ syntaxes/     # Syntax highlighting
│  └─ language-configuration.json
├─ assets/          # Logos and branding
│  ├─ cortex.svg    # Logo icon
│  └─ cortex-with-text.svg # Logo with text
├─ examples/        # Example .ctx files
├─ docs/            # Documentation
│  ├─ shared/       # Shared documentation
│  │  ├─ examples/  # Example .ctx files
│  │  ├─ syntax.md
│  │  ├─ functions.md
│  │  └─ *.md       # Core documentation
│  └─ rust/         # Rust-specific docs
├─ .vscode/         # VS Code workspace settings
└─ format_cortex.py # Code formatter
```

---

## 🏃‍♂️ Running Programs

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

**Rust Implementation:**

```bash
cd rust

# Basic hello world
cargo run -- run ../examples/hello_world.ctx

# Functions and variables
cargo run -- run ../examples/function_demo.ctx

# Arithmetic operations
cargo run -- run ../examples/arithmetic.ctx

# Check syntax
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
- **Issues**: [GitHub Issues](https://github.com/yourname/cortex/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourname/cortex/discussions)

---

**Ready to try Cortex?** Start with our [Getting Started Guide](docs/shared/run.md) and explore the [examples](examples/)!

**Rust implementation is ready!** High-performance interpreter with VS Code support! 🚀
