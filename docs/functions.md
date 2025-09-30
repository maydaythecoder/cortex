# Functions in Cortex

Functions are first-class citizens in Cortex, designed specifically for AI/ML workflows. This guide covers both current working features and planned future capabilities.

## Table of Contents

1. [Current Working Features](#current-working-features)
2. [Planned Future Features](#planned-future-features)
3. [Best Practices](#best-practices)
4. [Migration Guide](#migration-guide)

---

## Current Working Features

### ✅ Basic Function Declaration

**Syntax**: `func function_name[parameters] | statements ^`

```cortex
// Function with parameters
func add[a, b] |
  return[a + b]
^

// Function without parameters
func print_hello[] |
  print["Hello, Cortex!"]
^

// Function with no return value
func main[] |
  print["Program starting"]
^
```

**Features**:

- Multiple parameters supported
- Optional parameters (all must be provided when calling)
- Return statements with `return[expression]`
- Recursive function calls

### ✅ Function Calls

**Syntax**: `function_name[argument1, argument2, ...]`

```cortex
let result := add[5, 3]        // 8.0
print_hello[]                  // "Hello, Cortex!"
main[]                         // "Program starting"
```

### ✅ Working Function Examples

#### Simple Calculator Functions

```cortex
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

func main[] |
  let x := 10
  let y := 3
  
  let sum := add[x, y]
  let diff := subtract[x, y]
  let product := multiply[x, y]
  let quotient := divide[x, y]
  
  print[sum]
  print[diff]
  print[product]
  print[quotient]
^

main[]
```

**Output**:

``` txt
13.000000
7.000000
30.000000
3.333333
```

#### Factorial Function

```cortex
func factorial[n] |
  let result := 1
  let temp := 1
  
  while [temp <= n] |
    let result := result * temp
    let temp := temp + 1
  ^
  
  return[result]
^

func main[] |
  let n := 5
  let fact := factorial[n]
  print[fact]  // Output: 120.000000
^

main[]
```

#### Recursive Functions

```cortex
func factorial_recursive[n] |
  if [n <= 1] |
    return[1]
  ^
  return[n * factorial_recursive[n - 1]]
^

func fibonacci[n] |
  if [n <= 1] |
    return[n]
  ^
  return[fibonacci[n - 1] + fibonacci[n - 2]]
^

func main[] |
  let fact := factorial_recursive[5]
  let fib := fibonacci[10]
  
  print[fact]  // 120.000000
  print[fib]   // 55.000000
^

main[]
```

#### Function with Conditional Logic

```cortex
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
  let x := 10
  let y := 20
  
  let maximum := max[x, y]
  let minimum := min[x, y]
  
  print[maximum]  // 20.000000
  print[minimum]  // 10.000000
^

main[]
```

### ✅ Function Scope and Variables

#### Local Variables

```cortex
func example_function[x] |
  let local_var := x * 2
  print[local_var]
  return[local_var]
^

func main[] |
  let global_var := 10
  let result := example_function[global_var]
  print[result]  // 20.000000
^

main[]
```

#### Variable Shadowing

```cortex
func shadow_example[x] |
  let x := x * 2  // Shadows parameter
  print[x]        // Prints doubled value
  return[x]
^

func main[] |
  let x := 5
  let result := shadow_example[x]
  print[x]        // Original x unchanged: 5.000000
  print[result]   // Function result: 10.000000
^

main[]
```

### ✅ Main Function Pattern

```cortex
// Define helper functions
func helper_function[x] |
  return[x * 2]
^

// Main function as entry point
func main[] |
  let value := 10
  let result := helper_function[value]
  print[result]
^

// Call main function to start program
main[]
```

### ✅ Built-in Functions

#### Print Function

```cortex
func main[] |
  // Print numbers
  print[42]        // Output: 42.000000
  
  // Print strings
  print["Hello"]   // Output: Hello
  
  // Print variables
  let x := 100
  print[x]         // Output: 100.000000
^

main[]
```

#### String Conversion (Basic)

```cortex
func main[] |
  let x := 42
  let s := str[x]  // Basic string conversion
  print[s]         // Output: 42.000000 (currently returns number)
^

main[]
```

**Note**: The `str` function currently returns the input value as-is. Proper string conversion will be implemented in future versions.

---

## Planned Future Features

### 🔄 Advanced Type System

#### Type Annotations

```cortex
// Function with explicit type annotations
func add[a: int, b: int] -> int |
  return[a + b]
^

func process_data[data: tensor] -> tensor |
  return[data * 2]
^

func create_model[input_size: int, hidden_size: int] -> neural_network |
  return[neural_network[input_size, hidden_size]]
^
```

#### Generic Functions

```cortex
// Generic function for any numeric type
func double[T: numeric][x: T] -> T |
  return[x * 2]
^

// Usage with different types
let int_result := double[int][5]        // 10
let float_result := double[float][3.14] // 6.28
```

### 🔄 Advanced Parameters

#### Default Parameters

```cortex
func create_model[input_size: int, hidden_size: int := 128, activation: string := "relu"] |
  return[neural_network[input_size, hidden_size, activation]]
^

// Usage
let model1 := create_model[784]                    // Uses defaults
let model2 := create_model[784, 256]               // Custom hidden_size
let model3 := create_model[784, 256, "tanh"]       // All custom
```

#### Variadic Functions

```cortex
func sum_all[numbers: ...int] -> int |
  let total := 0
  for [num in numbers] |
    total := total + num
  ^
  return[total]
^

func concatenate[strings: ...string] -> string |
  let result := ""
  for [s in strings] |
    result := result + s
  ^
  return[result]
^

// Usage
let total := sum_all[1, 2, 3, 4, 5]  // 15
let greeting := concatenate["Hello", " ", "World"]  // "Hello World"
```

#### Keyword Arguments

```cortex
func train_model[model: neural_network, data: tensor, labels: tensor, 
                 learning_rate: float := 0.001, epochs: int := 10] |
  // Training implementation
^

// Usage with keyword arguments
train_model[model=my_model, data=train_data, labels=train_labels, epochs=20]
```

### 🔄 Higher-Order Functions

#### Functions as Parameters

```cortex
func apply_function[func, data: tensor] -> tensor |
  let result := []
  for [item in data] |
    result := result + [func[item]]
  ^
  return[tensor[result]]
^

func square[x: float] -> float |
  return[x * x]
^

// Usage
let data := tensor[[1, 2, 3, 4, 5]]
let squared_data := apply_function[square, data]  // [1, 4, 9, 16, 25]
```

#### Lambda Functions

```cortex
func process_data[data: tensor] -> tensor |
  return[apply_function[lambda[x] -> x * x + 1, data]]
^

// Or store lambdas in variables
let square_func := lambda[x] -> x * x
let cube_func := lambda[x] -> x * x * x
```

#### Function Composition

```cortex
func compose[f, g] |
  return[lambda[x] -> f[g[x]]]
^

let add_one := lambda[x] -> x + 1
let double := lambda[x] -> x * 2
let add_one_then_double := compose[double, add_one]

let result := add_one_then_double[5]  // (5 + 1) * 2 = 12
```

### 🔄 Function Overloading

```cortex
// Multiple functions with same name but different signatures
func process[x: int] -> int |
  return[x * 2]
^

func process[x: float] -> float |
  return[x * 2.0]
^

func process[x: tensor] -> tensor |
  return[x * 2]
^

func process[x: string] -> string |
  return[x + x]
^

// Usage - compiler chooses based on argument types
let int_result := process[5]        // 10
let float_result := process[3.14]   // 6.28
let tensor_result := process[tensor[[1, 2, 3]]]  // [2, 4, 6]
let string_result := process["Hi"]  // "HiHi"
```

### 🔄 Advanced Return Types

#### Multiple Return Values

```cortex
func divide_with_remainder[a: int, b: int] -> (int, int) |
  return[(a / b, a % b)]
^

// Usage with destructuring
let (quotient, remainder) := divide_with_remainder[10, 3]
print[quotient]   // 3
print[remainder]  // 1
```

#### Optional Return Types

```cortex
func safe_divide[a: float, b: float] -> optional[float] |
  if [b == 0.0] |
    return[none]
  ^
  return[some[a / b]]
^

// Usage
let result := safe_divide[10.0, 2.0]
match [result] |
  case [some[value]] -> print["Result: " + str[value]]
  case [none] -> print["Error: Division by zero"]
^
```

### 🔄 Error Handling

#### Try-Catch Functions

```cortex
func safe_divide[a: float, b: float] -> float |
  if [b == 0.0] |
    throw[division_by_zero_error["Cannot divide by zero"]]
  ^
  return[a / b]
^

func load_model[path: string] -> neural_network |
  try |
    return[deserialize[read_file[path]]]
  ^ catch [file_error] |
    print["Error loading model: " + str[file_error]]
    return[default_model[]]
  ^ catch [parse_error] |
    print["Error parsing model: " + str[parse_error]]
    throw[parse_error]  // Re-throw
  ^
^
```

### 🔄 Performance Features

#### Inline Functions

```cortex
// Compiler will inline this function call
inline func square[x: float] -> float |
  return[x * x]
^

// Usage - no function call overhead
let result := square[5.0]
```

#### Memoization

```cortex
// Automatic memoization for expensive functions
@memoize
func fibonacci[n: int] -> int |
  if [n <= 1] |
    return[n]
  ^
  return[fibonacci[n - 1] + fibonacci[n - 2]]
^

// Or manual memoization
func memoize[f] |
  let cache := {}
  
  return[lambda[args...] |
    let key := str[args]
    if [key in cache] |
      return[cache[key]]
    ^
    
    let result := f[args...]
    cache[key] := result
    return[result]
  ^]
^

let memoized_factorial := memoize[factorial]
```

#### Tail Call Optimization

```cortex
// Optimized tail-recursive function
func factorial_tail[n: int, acc: int := 1] -> int |
  if [n <= 1] |
    return[acc]
  ^
  return[factorial_tail[n - 1, n * acc]]
^

// Compiler optimizes this to a loop
```

### 🔄 AI/ML Specific Functions

#### Model Definition Functions

```cortex
func create_sequential_model[layers: ...layer] -> neural_network |
  return[neural_network[sequential, layers]]
^

func dense_layer[size: int, activation: string := "linear"] -> layer |
  return[dense[size, activation]]
^

// Usage
let model := create_sequential_model[
  dense_layer[128, "relu"],
  dense_layer[64, "relu"],
  dense_layer[10, "softmax"]
]
```

#### Training Functions

```cortex
func train_model[model: neural_network, 
                 data: tensor, 
                 labels: tensor,
                 optimizer: optimizer,
                 epochs: int,
                 batch_size: int] -> training_history |
  return[train[model, data, labels, optimizer, epochs, batch_size]]
^

// Usage
let history := train_model[
  model=my_model,
  data=training_data,
  labels=training_labels,
  optimizer=adam[learning_rate=0.001],
  epochs=10,
  batch_size=32
]
```

#### Loss Functions

```cortex
func mean_squared_error[predictions: tensor, targets: tensor] -> float |
  let diff := predictions - targets
  return[mean[diff * diff]]
^

func categorical_crossentropy[predictions: tensor, targets: tensor] -> float |
  return[-sum[targets * log[predictions]]]
^
```

---

## Best Practices

### ✅ Current Best Practices

1. **Use descriptive function names**:

   ```cortex
   func calculate_gradient_descent[model, data, lr]  // Good
   func func1[a, b, c]                            // Bad
   ```

2. **Keep functions focused**:

   ```cortex
   // Good: focused functions
   func calculate_loss[predictions, targets]
   func update_weights[weights, gradients, lr]
   
   // Bad: doing too much
   func process_everything[data, model, optimizer, ...]
   ```

3. **Structure programs with main function**:

   ```cortex
   func main[] |
     // Your program logic here
     print["Program starting"]
     // ... rest of program
   ^
   
   main[]
   ```

4. **Use meaningful parameter names**:

   ```cortex
   func train_model[model, training_data, learning_rate]  // Good
   func train_model[a, b, c]                             // Bad
   ```

5. **Handle edge cases when possible**:

   ```cortex
   func safe_divide[a, b] |
     if [b == 0] |
       print["Error: Division by zero"]
       return[0]
     ^
     return[a / b]
   ^
   ```

### 🔄 Future Best Practices

1. **Use type annotations for clarity**:

   ```cortex
   func process_data[data: tensor[float], labels: tensor[int]] -> float |
     // Clear what types are expected
   ^
   ```

2. **Leverage function overloading**:

   ```cortex
   // Handle different input types gracefully
   func normalize[data: tensor[float]] -> tensor[float]
   func normalize[data: tensor[int]] -> tensor[float]
   ```

3. **Use higher-order functions for reusability**:

   ```cortex
   // Generic processing function
   func apply_transformation[data: tensor, transform: function] -> tensor
   
   // Specific transformations
   let normalized := apply_transformation[data, normalize]
   let scaled := apply_transformation[data, scale]
   ```

4. **Implement proper error handling**:

   ```cortex
   func robust_load_model[path: string] -> optional[neural_network] |
     try |
       return[some[load_model[path]]]
     ^ catch [error] |
       log_error["Failed to load model: " + str[error]]
       return[none]
     ^
   ^
   ```

---

## Migration Guide

### From Python

**Python**:

```python
def add(a, b):
    return a + b

def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def safe_divide(a, b):
    if b == 0:
        return None
    return a / b
```

**Cortex (Current)**:

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

func safe_divide[a, b] |
  if [b == 0] |
    return[0]  // No None yet
  ^
  return[a / b]
^
```

**Cortex (Future)**:

```cortex
func add[a: int, b: int] -> int |
  return[a + b]
^

func factorial[n: int] -> int |
  if [n <= 1] |
    return[1]
  ^
  return[n * factorial[n - 1]]
^

func safe_divide[a: float, b: float] -> optional[float] |
  if [b == 0.0] |
    return[none]
  ^
  return[some[a / b]]
^
```

### From JavaScript

**JavaScript**:

```javascript
function calculateArea(length, width) {
    return length * width;
}

function greet(name, greeting = "Hello") {
    return greeting + ", " + name;
}

const square = (x) => x * x;
```

**Cortex (Current)**:

```cortex
func calculate_area[length, width] |
  return[length * width]
^

func greet[name] |
  print["Hello, " + name]  // No default params yet
^

// No lambda functions yet
```

**Cortex (Future)**:

```cortex
func calculate_area[length: float, width: float] -> float |
  return[length * width]
^

func greet[name: string, greeting: string := "Hello"] -> string |
  return[greeting + ", " + name]
^

let square := lambda[x: float] -> x * x
```

### From Haskell

**Haskell**:

```haskell
add :: Int -> Int -> Int
add a b = a + b

factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)

safeDivide :: Float -> Float -> Maybe Float
safeDivide _ 0 = Nothing
safeDivide a b = Just (a / b)
```

**Cortex (Current)**:

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

// No Maybe/Optional types yet
```

**Cortex (Future)**:

```cortex
func add[a: int, b: int] -> int |
  return[a + b]
^

func factorial[n: int] -> int |
  match [n] |
    case [0] -> 1
    case [n] -> n * factorial[n - 1]
  ^
^

func safe_divide[a: float, b: float] -> optional[float] |
  if [b == 0.0] |
    return[none]
  ^
  return[some[a / b]]
^
```

---

## Current Limitations

1. **No Type Annotations**: Function parameters and return types cannot be explicitly specified
2. **No Default Parameters**: All parameters must be provided when calling functions
3. **No Variadic Functions**: Functions cannot take variable numbers of arguments
4. **No Function Overloading**: Cannot have multiple functions with the same name
5. **No Lambda Functions**: Cannot create anonymous functions
6. **No Higher-Order Functions**: Cannot pass functions as parameters
7. **Limited Error Handling**: No try-catch blocks or error handling mechanisms
8. **No Optional Return Types**: Cannot return optional or nullable values

## Getting Started

1. **Try Basic Functions**: Start with simple arithmetic functions
2. **Use Recursion**: Implement recursive algorithms like factorial
3. **Structure with Main**: Always use a main function as entry point
4. **Read Examples**: Check out [examples/](../examples/) for working code
5. **Compile and Run**: Use `python3 cortexc.py build` to create executables

For more information, see [syntax.md](syntax.md) for language syntax or [run.md](run.md) for compilation instructions.
