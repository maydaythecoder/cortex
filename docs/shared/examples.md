# Cortex Examples

This document provides comprehensive examples of Cortex programs, organized by complexity and functionality. Each example includes working code, compilation instructions, and expected output.

## Table of Contents

1. [Basic Examples](#basic-examples)
2. [Intermediate Examples](#intermediate-examples)
3. [Advanced Examples](#advanced-examples)
4. [AI/ML Examples](#aiml-examples)
5. [Compilation and Running](#compilation-and-running)

---

## Basic Examples

### Hello World

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

### Arithmetic Operations

**File**: `examples/simple_arithmetic.ctx`

```cortex
func main[] |
  let a := 10
  let b := 3
  
  let sum := a + b
  let diff := a - b
  let product := a * b
  let quotient := a / b
  let remainder := a % b
  let power := a ** 2
  
  print[sum]
  print[diff]
  print[product]
  print[quotient]
  print[remainder]
  print[power]
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
1.000000
100.000000
```

### Variables and Assignment

**File**: `examples/variables.ctx`

```cortex
func main[] |
  // Basic variable assignment
  let x := 42
  let y := 3.14
  let name := "Hello"
  
  print[x]
  print[y]
  print[name]
  
  // Variable reassignment
  let x := x + 8
  print[x]  // 50.000000
  
  // Variable shadowing
  if [x > 45] |
    let x := 100  // Shadows outer x
    print[x]      // 100.000000
  ^
  
  print[x]        // Back to 50.000000
^

main[]
```

### Comparison Operations

**File**: `examples/comparisons.ctx`

```cortex
func main[] |
  let x := 10
  let y := 5
  let z := 10
  
  // Equality comparisons
  print[x == y]    // 0.000000 (false)
  print[x == z]    // 1.000000 (true)
  print[x != y]    // 1.000000 (true)
  
  // Order comparisons
  print[x < y]     // 0.000000 (false)
  print[x > y]     // 1.000000 (true)
  print[x <= z]    // 1.000000 (true)
  print[x >= z]    // 1.000000 (true)
^

main[]
```

### Logical Operations

**File**: `examples/logical.ctx`

```cortex
func main[] |
  let a := 1  // true
  let b := 0  // false
  
  // Logical operations
  print[a and b]    // 0.000000 (false)
  print[a or b]     // 1.000000 (true)
  print[not a]      // 0.000000 (false)
  print[not b]      // 1.000000 (true)
  
  // Complex logical expressions
  print[a and not b]        // 1.000000 (true)
  print[a or b and not a]   // 1.000000 (true)
^

main[]
```

---

## Intermediate Examples

### Conditional Statements

**File**: `examples/conditionals.ctx`

```cortex
func main[] |
  let score := 85
  
  // Simple if statement
  if [score >= 90] |
    print["Grade: A"]
  ^
  
  // If-else statement
  if [score >= 80] |
    print["Grade: B"]
  ^ else |
    print["Grade: C or below"]
  ^
  
  // Nested conditionals
  if [score >= 70] |
    if [score >= 80] |
      print["Good job!"]
    ^ else |
      print["Keep improving!"]
    ^
  ^ else |
    print["Need more practice"]
  ^
^

main[]
```

### While Loops

**File**: `examples/simple_loops.ctx`

```cortex
func main[] |
  // Count from 1 to 5
  let counter := 1
  
  while [counter <= 5] |
    print[counter]
    let counter := counter + 1
  ^
  
  // Calculate factorial
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

**Output**:

``` txt
1.000000
2.000000
3.000000
4.000000
5.000000
120.000000
```

### Function Definitions

**File**: `examples/function_demo.ctx`

```cortex
// Simple arithmetic functions
func add[a, b] |
  return[a + b]
^

func subtract[a, b] |
  return[a - b]
^

func multiply[a, b] |
  return[a * b]
^

func divide[a, b] |
  return[a / b]
^

// Mathematical functions
func factorial[n] |
  if [n <= 1] |
    return[1]
  ^
  return[n * factorial[n - 1]]
^

func fibonacci[n] |
  if [n <= 1] |
    return[n]
  ^
  return[fibonacci[n - 1] + fibonacci[n - 2]]
^

// Main function
func main[] |
  // Test arithmetic functions
  let sum := add[10, 20]
  let diff := subtract[10, 20]
  let product := multiply[10, 20]
  let quotient := divide[10, 20]
  
  print[sum]      // 30.000000
  print[diff]     // -10.000000
  print[product]  // 200.000000
  print[quotient] // 0.500000
  
  // Test mathematical functions
  let fact := factorial[5]
  let fib := fibonacci[10]
  
  print[fact]     // 120.000000
  print[fib]      // 55.000000
^

main[]
```

### Function with Parameters

**File**: `examples/functions_with_params.ctx`

```cortex
// Function with multiple parameters
func calculate_area[length, width] |
  return[length * width]
^

func calculate_perimeter[length, width] |
  return[2 * (length + width)]
^

func is_square[length, width] |
  return[length == width]
^

// Function with conditional logic
func max[a, b] |
  if [a > b] |
    return[a]
  ^ else |
    return[b]
  ^
^

func min[a, b] |
  if [a < b] |
    return[a]
  ^ else |
    return[b]
  ^
^

func main[] |
  let length := 5
  let width := 3
  
  let area := calculate_area[length, width]
  let perimeter := calculate_perimeter[length, width]
  let square_check := is_square[length, width]
  
  print[area]        // 15.000000
  print[perimeter]   // 16.000000
  print[square_check] // 0.000000 (false)
  
  let maximum := max[length, width]
  let minimum := min[length, width]
  
  print[maximum]     // 5.000000
  print[minimum]     // 3.000000
^

main[]
```

### Variable Scope

**File**: `examples/scope.ctx`

```cortex
func outer_function[x] |
  print["Outer function, x = " + str[x]]
  
  let local_var := x * 2
  print["Local variable = " + str[local_var]]
  
  if [x > 5] |
    let shadowed_x := x * 3  // Shadows parameter
    print["Shadowed x = " + str[shadowed_x]]
  ^
  
  return[local_var]
^

func main[] |
  let global_x := 10
  
  let result := outer_function[global_x]
  print["Result = " + str[result]]
  
  // global_x unchanged
  print["Global x = " + str[global_x]]
^

main[]
```

---

## Advanced Examples

### Recursive Functions

**File**: `examples/recursion.ctx`

```cortex
// Recursive factorial
func factorial[n] |
  if [n <= 1] |
    return[1]
  ^
  return[n * factorial[n - 1]]
^

// Recursive fibonacci
func fibonacci[n] |
  if [n <= 1] |
    return[n]
  ^
  return[fibonacci[n - 1] + fibonacci[n - 2]]
^

// Recursive greatest common divisor
func gcd[a, b] |
  if [b == 0] |
    return[a]
  ^
  return[gcd[b, a % b]]
^

// Recursive power function
func power[base, exp] |
  if [exp == 0] |
    return[1]
  ^
  if [exp == 1] |
    return[base]
  ^
  return[base * power[base, exp - 1]]
^

func main[] |
  // Test factorial
  print["Factorial of 5: " + str[factorial[5]]]
  
  // Test fibonacci
  print["Fibonacci of 10: " + str[fibonacci[10]]]
  
  // Test GCD
  print["GCD of 48 and 18: " + str[gcd[48, 18]]]
  
  // Test power
  print["2^8: " + str[power[2, 8]]]
^

main[]
```

### Complex Mathematical Functions

**File**: `examples/math_functions.ctx`

```cortex
// Approximate square root using Newton's method
func sqrt[n] |
  if [n < 0] |
    return[-1]  // Error case
  ^
  
  if [n == 0] |
    return[0]
  ^
  
  let guess := n / 2
  let iterations := 0
  
  while [iterations < 10] |
    let new_guess := (guess + n / guess) / 2
    if [new_guess == guess] |
      return[new_guess]
    ^
    let guess := new_guess
    let iterations := iterations + 1
  ^
  
  return[guess]
^

// Check if a number is prime
func is_prime[n] |
  if [n < 2] |
    return[0]
  ^
  
  let i := 2
  while [i * i <= n] |
    if [n % i == 0] |
      return[0]
    ^
    let i := i + 1
  ^
  
  return[1]
^

// Find next prime number
func next_prime[n] |
  let candidate := n + 1
  while [not is_prime[candidate]] |
    let candidate := candidate + 1
  ^
  return[candidate]
^

func main[] |
  // Test square root
  let number := 25
  let root := sqrt[number]
  print["Square root of " + str[number] + ": " + str[root]]
  
  // Test prime checking
  let test_numbers := [17, 18, 19, 20]
  let i := 0
  while [i < 4] |
    let num := test_numbers[i]
    let prime_check := is_prime[num]
    print["Is " + str[num] + " prime? " + str[prime_check]]
    let i := i + 1
  ^
  
  // Test next prime
  let start := 15
  let next := next_prime[start]
  print["Next prime after " + str[start] + ": " + str[next]]
^

main[]
```

### Algorithm Implementations

**File**: `examples/algorithms.ctx`

```cortex
// Bubble sort implementation
func bubble_sort[arr, length] |
  let i := 0
  while [i < length - 1] |
    let j := 0
    while [j < length - i - 1] |
      if [arr[j] > arr[j + 1]] |
        // Swap elements
        let temp := arr[j]
        let arr[j] := arr[j + 1]
        let arr[j + 1] := temp
      ^
      let j := j + 1
    ^
    let i := i + 1
  ^
  return[arr]
^

// Linear search
func linear_search[arr, length, target] |
  let i := 0
  while [i < length] |
    if [arr[i] == target] |
      return[i]
    ^
    let i := i + 1
  ^
  return[-1]  // Not found
^

// Binary search (requires sorted array)
func binary_search[arr, length, target] |
  let left := 0
  let right := length - 1
  
  while [left <= right] |
    let mid := (left + right) / 2
    if [arr[mid] == target] |
      return[mid]
    ^
    if [arr[mid] < target] |
      let left := mid + 1
    ^ else |
      let right := mid - 1
    ^
  ^
  
  return[-1]  // Not found
^

func main[] |
  // Test sorting
  let unsorted := [64, 34, 25, 12, 22, 11, 90]
  let length := 7
  
  print["Unsorted array: " + str[unsorted]]
  let sorted := bubble_sort[unsorted, length]
  print["Sorted array: " + str[sorted]]
  
  // Test searching
  let target := 25
  let linear_result := linear_search[sorted, length, target]
  let binary_result := binary_search[sorted, length, target]
  
  print["Linear search for " + str[target] + ": index " + str[linear_result]]
  print["Binary search for " + str[target] + ": index " + str[binary_result]]
^

main[]
```

---

## AI/ML Examples

### Simple Neural Network

**File**: `examples/neural_network.ctx`

```cortex
// Simple perceptron implementation
func sigmoid[x] |
  // Approximate sigmoid function: 1 / (1 + e^(-x))
  // For simplicity, using a basic approximation
  if [x > 5] |
    return[1]
  ^
  if [x < -5] |
    return[0]
  ^
  return[x / (1 + abs[x])]
^

// Forward pass through a single layer
func forward_pass[inputs, weights, bias] |
  let weighted_sum := bias
  let i := 0
  let input_length := 3  // Assuming 3 inputs
  
  while [i < input_length] |
    let weighted_sum := weighted_sum + inputs[i] * weights[i]
    let i := i + 1
  ^
  
  return[sigmoid[weighted_sum]]
^

// Training step for perceptron
func train_step[inputs, weights, bias, target, learning_rate] |
  let prediction := forward_pass[inputs, weights, bias]
  let error := target - prediction
  
  // Update weights
  let i := 0
  let input_length := 3
  
  while [i < input_length] |
    let weights[i] := weights[i] + learning_rate * error * inputs[i]
    let i := i + 1
  ^
  
  // Update bias
  let bias := bias + learning_rate * error
  
  return[prediction]
^

func main[] |
  // Initialize weights and bias
  let weights := [0.1, -0.2, 0.3]
  let bias := 0.0
  let learning_rate := 0.1
  
  // Training data (XOR problem simplified)
  let inputs1 := [0, 0, 1]
  let inputs2 := [0, 1, 1]
  let inputs3 := [1, 0, 1]
  let inputs4 := [1, 1, 1]
  
  let target1 := 0
  let target2 := 1
  let target3 := 1
  let target4 := 0
  
  // Training loop
  let epoch := 0
  while [epoch < 100] |
    let pred1 := train_step[inputs1, weights, bias, target1, learning_rate]
    let pred2 := train_step[inputs2, weights, bias, target2, learning_rate]
    let pred3 := train_step[inputs3, weights, bias, target3, learning_rate]
    let pred4 := train_step[inputs4, weights, bias, target4, learning_rate]
    
    let epoch := epoch + 1
  ^
  
  // Test predictions
  print["Final predictions:"]
  print["Input [0,0,1] -> " + str[forward_pass[inputs1, weights, bias]]]
  print["Input [0,1,1] -> " + str[forward_pass[inputs2, weights, bias]]]
  print["Input [1,0,1] -> " + str[forward_pass[inputs3, weights, bias]]]
  print["Input [1,1,1] -> " + str[forward_pass[inputs4, weights, bias]]]
^

main[]
```

### Linear Regression

**File**: `examples/linear_regression.ctx`

```cortex
// Mean function
func mean[values, length] |
  let sum := 0.0
  let i := 0
  
  while [i < length] |
    let sum := sum + values[i]
    let i := i + 1
  ^
  
  return[sum / length]
^

// Calculate linear regression coefficients
func linear_regression[x_values, y_values, length] |
  let x_mean := mean[x_values, length]
  let y_mean := mean[y_values, length]
  
  let numerator := 0.0
  let denominator := 0.0
  let i := 0
  
  while [i < length] |
    let x_diff := x_values[i] - x_mean
    let y_diff := y_values[i] - y_mean
    
    let numerator := numerator + x_diff * y_diff
    let denominator := denominator + x_diff * x_diff
    
    let i := i + 1
  ^
  
  let slope := numerator / denominator
  let intercept := y_mean - slope * x_mean
  
  return[slope, intercept]
^

// Predict value using linear regression model
func predict[x, slope, intercept] |
  return[slope * x + intercept]
^

func main[] |
  // Sample data
  let x_data := [1, 2, 3, 4, 5]
  let y_data := [2, 4, 5, 4, 5]
  let data_length := 5
  
  // Fit linear regression model
  let slope, intercept := linear_regression[x_data, y_data, data_length]
  
  print["Linear Regression Results:"]
  print["Slope: " + str[slope]]
  print["Intercept: " + str[intercept]]
  
  // Make predictions
  print["Predictions:")
  let test_x := 6
  let prediction := predict[test_x, slope, intercept]
  print["x = " + str[test_x] + " -> y = " + str[prediction]]
^

main[]
```

---

## Compilation and Running

### Basic Compilation

```bash
# Compile any example to binary
python3 cortexc.py build examples/hello_world.ctx -o hello
python3 cortexc.py build examples/simple_arithmetic.ctx -o arithmetic
python3 cortexc.py build examples/function_demo.ctx -o functions

# Run the compiled executables
./hello
./arithmetic
./functions
```

### Optimization Levels

```bash
# No optimization (fastest compilation)
python3 cortexc.py build examples/recursion.ctx -o recursion -O0

# Basic optimization (default)
python3 cortexc.py build examples/recursion.ctx -o recursion -O1

# Aggressive optimization
python3 cortexc.py build examples/recursion.ctx -o recursion -O2

# Maximum optimization
python3 cortexc.py build examples/recursion.ctx -o recursion -O3
```

### Verbose Output

```bash
# Get detailed compilation information
python3 cortexc.py build examples/neural_network.ctx -o nn -v

# This will show:
# - LLVM IR generation
# - Compilation steps
# - Any warnings or errors
```

### Batch Compilation

```bash
#!/bin/bash
# build_all_examples.sh

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

### Testing Examples

```bash
# Run examples with interpreter (for debugging)
python3 cortexc.py run examples/hello_world.ctx
python3 cortexc.py run examples/simple_arithmetic.ctx

# Interactive testing
python3 cortexc.py repl
```

---

## Next Steps

1. **Try the Examples**: Start with basic examples and work your way up
2. **Modify and Experiment**: Change values and see how programs behave
3. **Create Your Own**: Write your first Cortex program
4. **Read Documentation**: Check out [syntax.md](syntax.md) and [functions.md](functions.md)
5. **Join Community**: Contribute examples or improvements

## Getting Help

- **Documentation**: [docs/](docs/)
- **Examples**: [examples/](examples/)
- **Issues**: [GitHub Issues](https://github.com/yourname/cortex/issues)

For more information, visit the [main documentation](README.md) or explore the [language syntax](syntax.md).
