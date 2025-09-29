# Running Cortex Programs

This guide explains how to run Cortex programs using the Rust implementation.

## Prerequisites

- Rust 1.70+
- Cargo (comes with Rust)

## Basic Usage

### Running a Program

```bash
# Navigate to the rust directory
cd rust

# Run a program (interpreter mode)
cargo run -- run program.ctx
```

### Checking Syntax

```bash
cd rust
cargo run -- check program.ctx
```

### Building Executable

```bash
cd rust
cargo run -- build program.ctx -o program
```

## Running Examples

### All Available Examples

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
```

### Running from Project Root

```bash
# From the project root directory
cd /path/to/cortex

# Run any example
cd rust && cargo run -- run ../examples/hello_world.ctx

# Or use the provided shell script
./run_cortex.sh examples/hello_world.ctx
```

## VS Code Integration

If you have the Cortex VS Code extension installed:

1. **Open any `.ctx` file** in VS Code
2. **Press F5** or use "Run and Debug" to execute the file
3. **Use Ctrl+Shift+P** and search for "Cortex" commands
4. **Format on save** is automatically enabled for `.ctx` files

## Expected Output

### Hello World Example

``` txt
Hello, World!
Welcome to Cortex - the AI/ML programming language!
```

### Arithmetic Example

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

### Function Demo Example

``` txt
Hello from Cortex!
10 + 20 = [30]
```

## Troubleshooting

### Common Issues

1. **File not found**: Make sure you're in the correct directory
2. **Syntax errors**: Use `cargo run -- check` to validate syntax
3. **Runtime errors**: Check the error message for details
4. **Function return values**: Currently function calls return arrays (e.g., `[30]` instead of `30`)

### Getting Help

- Check the [syntax reference](syntax.md)
- Review the [examples](examples.md)
- See [function status](FUNCTION_STATUS.md)
- Read the [comprehensive features guide](ADVANCED_FEATURES.md)
