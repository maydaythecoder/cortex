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

- Python 3.8+
- Git

### Installation

```bash
git clone https://github.com/yourname/cortex
cd cortex
pip install -r requirements.txt
```

### Hello World

```bash
# Create hello_world.ctx
echo 'print["Hello, Cortex!"]' > hello_world.ctx

# Run it
python3 cortexc.py run hello_world.ctx
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

### Linear Regression Training Loop

```cortex
// Load dataset
let data := read["dataset.csv"]

// Define model
func linear[x, w, b] |
  return[(x * w) + b]
^

// Training
let w := 0.5
let b := 0.0
let lr := 0.01

for [epoch := 0; epoch < 100; epoch := epoch + 1] |
  let preds := linear[data.x, w, b]
  let loss := mse[preds, data.y]
  print["epoch=" + epoch + ", loss=" + loss]

  // Update params
  w := w - lr * grad[loss, w]
  b := b - lr * grad[loss, b]
^
```

---

## 🧩 Core Features

- **Numerical computing**: tensors, vectors, matrices
- **Control flow**: `if`, `while`, `for` with `| ... ^` blocks
- **Functions** with static typing + inference
- **Data structures**:
  - Arrays → tensors
  - Dictionaries → datasets
  - Objects → models
- **I/O utilities**: `read`, `print`
- **Built-in ML operators**: `∇[loss, params]`, `@` (dot product), `⊗` (outer product)

---

## 📂 Project Structure

``` txt
cortex/
├─ examples/         # Working example programs
│  ├─ hello_world.ctx
│  └─ function_demo.ctx
├─ docs/            # Complete documentation
│  ├─ syntax.md
│  ├─ run.md
│  ├─ functions.md
│  ├─ data_structures.md
│  ├─ numerical.md
│  ├─ training.md
│  └─ examples/     # Example documentation
│      ├─ neural_network.md
│      ├─ linear_regression.md
│      ├─ gradient_descent.md
│      └─ data_processing.md
├─ compiler/        # Python-based interpreter
│  ├─ parser/       # Lexer and parser
│  ├─ ast/          # Abstract syntax tree
│  └─ interpreter.py
└─ cortexc.py       # Command-line tool
```

---

## 🎯 Language Examples

### Unique Syntax

```cortex
// Constants with ::
let MAX_EPOCHS :: 1000
let LEARNING_RATE :: 0.01

// Variables with :=
let counter := 0
let model := create_network[]

// Functions with brackets
func train[model, data] |
  for [epoch in range[0, MAX_EPOCHS]] |
    let loss := compute_loss[model, data]
    if [loss < 0.001] |
      print["Converged!"]
      break
    ^
  ^
^
```

### AI/ML Operations

```cortex
// Gradient computation
let gradients := ∇[loss, model.parameters]

// Matrix operations
let output := weights @ inputs + bias

// Tensor operations
let normalized := (data - mean[data]) / std[data]
```

---

## 📚 Documentation

- [**Getting Started**](docs/run.md) - How to install and run Cortex
- [**Syntax Guide**](docs/syntax.md) - Complete language syntax
- [**Functions**](docs/functions.md) - Function definition and usage
- [**Data Structures**](docs/data_structures.md) - Arrays, tensors, objects
- [**Numerical Computing**](docs/numerical.md) - Math and tensor operations
- [**Training Workflows**](docs/training.md) - ML training patterns

---

## 🏃‍♂️ Running Programs

### Available Commands

```bash
# Run a program (interpreter mode)
python3 cortexc.py run program.ctx

# Interactive REPL
python3 cortexc.py repl

# Build executable (future)
python3 cortexc.py build program.ctx -o program
```

### Try the Examples

```bash
# Basic hello world
python3 cortexc.py run examples/hello_world.ctx

# Functions and variables
python3 cortexc.py run examples/function_demo.ctx
```

---

## 🛠️ Development Status

- [x] **Core Language**: Variables, functions, control flow
- [x] **Basic Interpreter**: Run Cortex programs directly
- [x] **Standard Library**: Math, string operations
- [ ] **Tensor Operations**: Full numerical computing stack
- [ ] **LLVM Backend**: Compile to machine code
- [ ] **GPU Support**: CUDA/OpenCL backends
- [ ] **Package System**: Import and modules

---

## 🚀 Roadmap

### Phase 1: Core Language ✅

- [x] Lexer and parser
- [x] AST and interpreter
- [x] Basic syntax and operations
- [x] Function calls and control flow

### Phase 2: AI/ML Features (In Progress)

- [ ] Tensor data types and operations
- [ ] Automatic differentiation
- [ ] Neural network primitives
- [ ] Training loop utilities

### Phase 3: Performance

- [ ] LLVM compilation backend
- [ ] Memory optimization
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

```bash
git clone https://github.com/yourname/cortex
cd cortex
pip install -r requirements.txt

# Run tests
python3 -m pytest tests/

# Run examples
python3 cortexc.py run examples/hello_world.ctx
```

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **Examples**: [examples/](examples/)  
- **Issues**: [GitHub Issues](https://github.com/yourname/cortex/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourname/cortex/discussions)

---

**Ready to try Cortex?** Start with our [Getting Started Guide](docs/run.md) and explore the [examples](examples/)!
