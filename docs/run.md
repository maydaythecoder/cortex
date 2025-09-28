# Running Cortex Programs

This guide explains how to compile and run Cortex programs, from simple Hello World examples to complex AI/ML applications.

## Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Python**: 3.8 or higher
- **C Compiler**: GCC, Clang, or MSVC
- **LLVM**: 12.0 or higher (for code generation)

### Dependencies

Install the required dependencies:

```bash
# Install Python dependencies
pip install -r requirements.txt

# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install build-essential llvm-12-dev

# Install system dependencies (macOS)
brew install llvm

# Install system dependencies (Windows)
# Download and install LLVM from https://llvm.org/releases/
```

## Building the Runtime

Before running Cortex programs, you need to build the runtime library:

```bash
# Navigate to the runtime directory
cd runtime

# Build the shared library
make

# Install the library (optional)
sudo make install
```

## Compiling Cortex Programs

### Basic Compilation

Compile a Cortex program to machine code:

```bash
# Compile hello_world.ctx
cortexc build examples/hello_world.ctx -o hello_world

# Compile with optimization
cortexc build examples/hello_world.ctx -o hello_world -O2

# Compile with debug information
cortexc build examples/hello_world.ctx -o hello_world -g
```

### Compilation Options

```bash
cortexc build [options] <input_file> -o <output_file>

Options:
  -O0, -O1, -O2, -O3    Optimization level (default: -O1)
  -g                     Include debug information
  -v                     Verbose output
  -I <path>              Add include path
  -L <path>              Add library path
  -l <library>           Link with library
  --target <target>      Target architecture (x86_64, arm64, etc.)
  --help                 Show help message
```

## Running Programs

### Direct Execution

Run the compiled executable:

```bash
# Run the hello_world program
./hello_world

# Expected output:
# Hello, World!
# Welcome to Cortex - the AI/ML programming language!
```

### Interactive Mode

Run Cortex programs interactively:

```bash
# Start interactive session
cortexc repl

# Or run a specific file interactively
cortexc run examples/hello_world.ctx
```

## Example: Hello World

### 1. Create the Program

Create `hello_world.ctx`:

```cortex
// Hello World in Cortex
func main[] |
  print["Hello, World!"]
  print["Welcome to Cortex!"]
^

main[]
```

### 2. Compile the Program

```bash
cortexc build hello_world.ctx -o hello_world
```

### 3. Run the Program

```bash
./hello_world
```

### 4. Expected Output

```
Hello, World!
Welcome to Cortex!
```

## Example: Basic Syntax Demo

### 1. Compile the Demo

```bash
cortexc build examples/basic_syntax.ctx -o basic_demo
```

### 2. Run the Demo

```bash
./basic_demo
```

### 3. Expected Output

```
=== Cortex Basic Syntax Demo ===

Message: Hello from Cortex!
Counter: 0

Counter is less than 5
Counting up:
  0
  1
  2
  3
  4
Counting down:
  5
  4
  3
  2
  1
Math operations:
  10 + 3 = 13
  10 * 3 = 30
  10 / 3 = 3.333333
Array operations:
  Original: [1, 2, 3, 4, 5]
  Doubled: [2, 4, 6, 8, 10]
Configuration:
  Language: Cortex
  Version: 0.1.0
  Compiled: true

=== Demo Complete ===
```

## Advanced Examples

### Linear Regression

```bash
# Compile and run linear regression example
cortexc build examples/linear_regression.ctx -o linear_regression
./linear_regression
```

### Neural Network

```bash
# Compile and run neural network example
cortexc build examples/neural_network.ctx -o neural_network
./neural_network
```

### Data Processing

```bash
# Compile and run data processing example
cortexc build examples/data_processing.ctx -o data_processing
./data_processing
```

## Troubleshooting

### Common Issues

#### 1. Runtime Library Not Found

**Error**: `error while loading shared libraries: libcortex.so: cannot open shared object file`

**Solution**:

```bash
# Add runtime directory to library path
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/runtime

# Or install the library system-wide
cd runtime && sudo make install
```

#### 2. LLVM Not Found

**Error**: `LLVM not found`

**Solution**:

```bash
# Install LLVM development packages
sudo apt-get install llvm-12-dev  # Ubuntu/Debian
brew install llvm                 # macOS

# Set LLVM path
export LLVM_CONFIG=/usr/bin/llvm-config-12
```

#### 3. Compilation Errors

**Error**: `Syntax error in file`

**Solution**:

- Check Cortex syntax (use `|` and `^` for blocks, `[]` for parameters)
- Ensure all brackets are properly matched
- Verify function definitions and calls

#### 4. Runtime Errors

**Error**: `Segmentation fault` or `Runtime error`

**Solution**:

- Compile with debug information: `cortexc build -g`
- Use a debugger: `gdb ./program`
- Check for null pointer access and array bounds

### Debug Mode

Compile with debug information for easier troubleshooting:

```bash
# Compile with debug info
cortexc build examples/hello_world.ctx -o hello_world -g

# Run with debugger
gdb ./hello_world
(gdb) run
```

### Verbose Output

Get detailed compilation information:

```bash
# Verbose compilation
cortexc build examples/hello_world.ctx -o hello_world -v
```

## Performance Optimization

### Optimization Levels

```bash
# No optimization (fastest compilation)
cortexc build program.ctx -o program -O0

# Basic optimization (default)
cortexc build program.ctx -o program -O1

# Aggressive optimization
cortexc build program.ctx -o program -O2

# Maximum optimization
cortexc build program.ctx -o program -O3
```

### Profiling

Profile your programs for performance analysis:

```bash
# Compile with profiling support
cortexc build program.ctx -o program -g -pg

# Run and generate profile
./program
gprof program gmon.out > profile.txt
```

## Cross-Platform Compilation

### Target Different Architectures

```bash
# Compile for x86_64 (default)
cortexc build program.ctx -o program --target x86_64

# Compile for ARM64
cortexc build program.ctx -o program --target arm64

# Compile for WebAssembly
cortexc build program.ctx -o program.wasm --target wasm32
```

## Integration with Build Systems

### Makefile Example

```makefile
# Makefile for Cortex project
CORTEXC = cortexc
CFLAGS = -O2
SOURCES = $(wildcard *.ctx)
TARGETS = $(SOURCES:.ctx=)

all: $(TARGETS)

%: %.ctx
 $(CORTEXC) build $< -o $@ $(CFLAGS)

clean:
 rm -f $(TARGETS)

.PHONY: all clean
```

### CMake Example

```cmake
# CMakeLists.txt for Cortex project
cmake_minimum_required(VERSION 3.10)
project(CortexProject)

# Find Cortex compiler
find_program(CORTEXC cortexc REQUIRED)

# Add executable
add_custom_target(hello_world
    COMMAND ${CORTEXC} build examples/hello_world.ctx -o hello_world -O2
    DEPENDS examples/hello_world.ctx
)

# Add custom command
add_custom_command(
    OUTPUT hello_world
    COMMAND ${CORTEXC} build examples/hello_world.ctx -o hello_world -O2
    DEPENDS examples/hello_world.ctx
)
```

## Next Steps

1. **Explore Examples**: Try the examples in the `examples/` directory
2. **Read Documentation**: Check out `docs/syntax.md` for language details
3. **Write Your Own**: Create your first Cortex program
4. **Join Community**: Contribute to the project on GitHub

For more information, visit the [Cortex documentation](docs/) or check out the [examples](examples/).
