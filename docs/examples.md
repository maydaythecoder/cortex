# Cortex Examples

This guide provides detailed instructions for running all Cortex examples.

## Prerequisites

- Rust 1.70+
- Cargo (comes with Rust)

## Running Examples

### Basic Setup

```bash
# Navigate to the rust directory
cd rust
```

### All Available Examples

#### 1. Hello World

**Shorthand:**

```bash
./ctx hello_world
```

**Full Command:**

```bash
cargo run -- run ../examples/hello_world.ctx
```

**Expected Output:**

``` txt
Hello, World!
Welcome to Cortex - the AI/ML programming language!
```

#### 2. Function Demo

**Shorthand:**

```bash
./ctx function_demo
```

**Full Command:**

```bash
cargo run -- run ../examples/function_demo.ctx
```

**Expected Output:**

``` txt
Hello from Cortex!
10 + 20 = [30]
```

#### 3. Arithmetic Operations

**Shorthand:**

```bash
./ctx arithmetic
```

**Full Command:**

```bash
cargo run -- run ../examples/arithmetic.ctx
```

**Expected Output:**

``` txt
Arithmetic Operations:
a = 10
b = 3
a + b = 13
a - b = 7
a * b = 30
a / b = 3.3333333333333335
a % b = 1
a ** b = 1000
```

#### 4. Simple Algorithms

**Shorthand:**

```bash
./ctx simple_algorithms
```

**Full Command:**

```bash
cargo run -- run ../examples/simple_algorithms.ctx
```

#### 5. Fibonacci Sequence

**Shorthand:**

```bash
./ctx fibonacci
```

**Full Command:**

```bash
cargo run -- run ../examples/fibonacci.ctx
```

#### 6. Factorial Calculation

**Shorthand:**

```bash
./ctx factorial
```

**Full Command:**

```bash
cargo run -- run ../examples/factorial.ctx
```

#### 7. Prime Number Checking

**Shorthand:**

```bash
./ctx prime_numbers
```

**Full Command:**

```bash
cargo run -- run ../examples/prime_numbers.ctx
```

#### 8. Bubble Sort Demo

**Shorthand:**

```bash
./ctx bubble_sort
```

**Full Command:**

```bash
cargo run -- run ../examples/bubble_sort.ctx
```

#### 9. Comprehensive Features

**Shorthand:**

```bash
./ctx comprehensive_features
```

**Full Command:**

```bash
cargo run -- run ../examples/comprehensive_features.ctx
```

#### 10. AI/ML Comprehensive

**Shorthand:**

```bash
./ctx ai_ml_comprehensive
```

**Full Command:**

```bash
cargo run -- run ../examples/ai_ml_comprehensive.ctx
```

#### 11. Simple Loops

**Shorthand:**

```bash
./ctx simple_loops
```

**Full Command:**

```bash
cargo run -- run ../examples/simple_loops.ctx
```

#### 12. Loops

**Shorthand:**

```bash
./ctx loops
```

**Full Command:**

```bash
cargo run -- run ../examples/loops.ctx
```

#### 13. Printing

**Shorthand:**

```bash
./ctx printing
```

**Full Command:**

```bash
cargo run -- run ../examples/printing.ctx
```

#### 14. Simple Arithmetic

**Shorthand:**

```bash
./ctx simple_arithmetic
```

**Full Command:**

```bash
cargo run -- run ../examples/simple_arithmetic.ctx
```

## Syntax Checking

You can check the syntax of any example without running it:

**Shorthand:**

```bash
./ctxc hello_world
```

**Full Command:**

```bash
cargo run -- check ../examples/hello_world.ctx
```

## Running from Project Root

```bash
# From the project root directory
cd /path/to/cortex

# Use shorthand commands (recommended)
./ctx hello_world
./ctxc hello_world

# Run any .ctx file
./ctx run examples/hello_world.ctx
./ctxc check examples/hello_world.ctx

# Or use the provided shell script
./run_cortex.sh examples/hello_world.ctx

# Or use full commands
cd rust && cargo run -- run ../examples/hello_world.ctx
```

## VS Code Integration

If you have the Cortex VS Code extension installed:

1. **Open any `.ctx` file** in VS Code
2. **Press F5** or use "Run and Debug" to execute the file
3. **Use Ctrl+Shift+P** and search for "Cortex" commands
4. **Format on save** is automatically enabled for `.ctx` files

## Troubleshooting

### Common Issues

1. **File not found**: Make sure you're in the correct directory
2. **Syntax errors**: Use `cargo run -- check` to validate syntax
3. **Runtime errors**: Check the error message for details
4. **Function return values**: Currently function calls return arrays (e.g., `[30]` instead of `30`)

### Getting Help

- Check the [syntax reference](syntax.md)
- Review the [function status](FUNCTION_STATUS.md)
- Read the [comprehensive features guide](ADVANCED_FEATURES.md)
