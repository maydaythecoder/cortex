# Cortex Language Syntax

Cortex is a functional programming language designed specifically for AI/ML workflows. This document covers the complete syntax with clear separation between **current working features** and **planned future capabilities**.

## Table of Contents

1. [Current Working Features](#current-working-features)
2. [Planned Future Features](#planned-future-features)
3. [Language Design Philosophy](#language-design-philosophy)
4. [Migration Guide](#migration-guide)

---

## Current Working Features

### ✅ Variables and Assignment

**Syntax**: `let variable_name := expression`

```cortex
let x := 42
let y := 3.14
let name := "Hello"
let result := x + y
```

**Features**:

- Type inference (all variables are currently numbers)
- Variable shadowing in different scopes
- Assignment with arithmetic expressions

### ✅ Arithmetic Operations

**Operators**: `+`, `-`, `*`, `/`, `%`, `**`

```cortex
let a := 10
let b := 3

let sum := a + b        // 13.0
let diff := a - b       // 7.0
let product := a * b    // 30.0
let quotient := a / b   // 3.333...
let remainder := a % b  // 1.0
let power := a ** 2     // 100.0
```

**Features**:

- All arithmetic operations work with numbers
- Automatic type promotion to floating point
- Standard operator precedence

### ✅ Comparison Operations

**Operators**: `==`, `!=`, `<`, `<=`, `>`, `>=`

```cortex
let x := 10
let y := 5

let is_equal := x == y      // false
let is_not_equal := x != y  // true
let is_less := x < y        // false
let is_less_equal := x <= y // false
let is_greater := x > y     // true
let is_greater_equal := x >= y // true
```

### ✅ Logical Operations

**Operators**: `&&`, `||`, `!`

```cortex
let a := true
let b := false

let result1 := a && b     // false
let result2 := a || b     // true
let result3 := !a         // false
```

### ✅ Conditional Statements

**Syntax**: `if [condition] | statements ^`

```cortex
let x := 10

if [x > 5] |
  print["x is greater than 5"]
^

if [x == 10] |
  print["x equals 10"]
^ else |
  print["x does not equal 10"]
^
```

**Features**:

- `if` statements with conditions
- `else` clauses supported
- Block syntax with `|` and `^`

### ✅ While Loops

**Syntax**: `while [condition] | statements ^`

```cortex
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
```

### ✅ Function Definitions

**Syntax**: `func function_name[parameters] | statements ^`

```cortex
// Function with parameters
func add[a, b] |
  return[a + b]
^

// Function without parameters
func greet[] |
  print["Hello, World!"]
^

// Function with no return value
func main[] |
  print["Program starting"]
^
```

**Features**:

- Multiple parameters supported
- Return statements with `return[expression]`
- Recursive function calls
- Function calls with `function_name[args]`

### ✅ Function Calls

**Syntax**: `function_name[argument1, argument2, ...]`

```cortex
let result := add[5, 3]        // 8.0
greet[]                         // "Hello, World!"
main[]                          // "Program starting"
```

### ✅ Built-in Functions

#### Print Function

**Syntax**: `print[expression]`

```cortex
print[42]           // Output: 42.000000
print["Hello"]      // Output: Hello
print[variable]     // Output: variable value
```

#### String Conversion (Basic)

**Syntax**: `str[expression]`

```cortex
let x := 42
let s := str[x]     // Currently returns the input as-is
print[s]            // Output: 42.000000
```

### ✅ Comments

**Syntax**: `// comment text`

```cortex
// This is a single-line comment
let x := 42  // Inline comment
```

### ✅ Program Structure

**Standard Structure**:

```cortex
// Function definitions
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

---

## Planned Future Features

### 🔄 Advanced Type System

**Type Annotations**:

```cortex
let x: int := 42
let y: float := 3.14
let name: string := "Hello"
let data: tensor := [1, 2, 3, 4]
```

**Type Inference**:

```cortex
let x := 42          // Inferred as int
let y := 3.14        // Inferred as float
let z := x + y       // Inferred as float
```

### 🔄 Complex Data Types

**Arrays**:

```cortex
let numbers := [1, 2, 3, 4, 5]
let matrix := [[1, 2], [3, 4]]
let empty_array := []
```

**Strings with Concatenation**:

```cortex
let name := "John"
let greeting := "Hello, " + name + "!"  // "Hello, John!"
let multiline := """
This is a
multiline string
"""
```

**Dictionaries**:

```cortex
let config := {
  "learning_rate": 0.01,
  "batch_size": 32,
  "epochs": 100
}
```

### 🔄 Advanced Control Flow

**For Loops**:

```cortex
for [i in range[10]] |
  print[i]
^

for [item in data] |
  process[item]
^
```

**Pattern Matching**:

```cortex
match [value] |
  case [0] -> print["Zero"]
  case [1] -> print["One"]
  case [n] -> print["Number: " + str[n]]
^
```

### 🔄 Advanced Functions

**Default Parameters**:

```cortex
func create_model[input_size, hidden_size := 128] |
  // Implementation
^
```

**Variadic Functions**:

```cortex
func sum_all[numbers...] |
  let total := 0
  for [num in numbers] |
    total := total + num
  ^
  return[total]
^

let result := sum_all[1, 2, 3, 4, 5]  // 15
```

**Higher-Order Functions**:

```cortex
func apply_function[func, data] |
  return[map[func, data]]
^

let squared := apply_function[lambda[x] -> x * x, [1, 2, 3]]
```

**Function Overloading**:

```cortex
func process[x: int] -> int |
  return[x * 2]
^

func process[x: float] -> float |
  return[x * 2.0]
^

func process[x: tensor] -> tensor |
  return[x * 2]
^
```

### 🔄 AI/ML Specific Features

**Tensor Operations**:

```cortex
let tensor1 := tensor[[1, 2, 3]]
let tensor2 := tensor[[4, 5, 6]]
let result := tensor1 + tensor2  // Element-wise addition
```

**Neural Network Definitions**:

```cortex
let model := neural_network[
  input[784],
  dense[128, activation="relu"],
  dropout[0.2],
  dense[10, activation="softmax"]
]
```

**Model Training**:

```cortex
train[model, 
  data=training_data,
  labels=training_labels,
  optimizer=adam[learning_rate=0.001],
  epochs=10,
  batch_size=32
]
```

### 🔄 Error Handling

**Try-Catch Blocks**:

```cortex
try |
  let result := divide[a, b]
  print[result]
^ catch [error] |
  print["Error: " + str[error]]
^
```

**Optional Types**:

```cortex
let maybe_value: optional[int] := some[42]
let no_value: optional[int] := none

match [maybe_value] |
  case [some[x]] -> print["Value: " + str[x]]
  case [none] -> print["No value"]
^
```

### 🔄 Modules and Imports

**Module System**:

```cortex
import [math, statistics]
import [neural_networks as nn]

let result := math.sqrt[16]
let model := nn.sequential[
  nn.dense[128],
  nn.relu[],
  nn.dense[10]
]
```

### 🔄 Advanced Syntax

**List Comprehensions**:

```cortex
let squares := [x * x for x in range[10]]
let evens := [x for x in range[20] if x % 2 == 0]
```

**Lambda Functions**:

```cortex
let square := lambda[x] -> x * x
let add := lambda[x, y] -> x + y
```

**Decorators**:

```cortex
@memoize
func fibonacci[n] |
  if [n <= 1] |
    return[n]
  ^
  return[fibonacci[n - 1] + fibonacci[n - 2]]
^
```

---

## Language Design Philosophy

### Core Principles

1. **Functional First**: Functions are first-class citizens
2. **AI/ML Focused**: Built-in support for common ML patterns
3. **Type Safety**: Strong typing with inference where possible
4. **Performance**: Compiled to native code via LLVM
5. **Readability**: Clean, mathematical syntax

### Syntax Choices

**Square Brackets `[]`**:

- Used for function calls and parameters
- Distinguishes from mathematical grouping
- Consistent with functional programming conventions

**Pipe Characters `|` and `^`**:

- Clear block delimiters
- Easy to type and read
- Consistent throughout the language

**Assignment Operator `:=`**:

- Distinguishes from equality comparison
- Common in functional languages
- Clear semantic meaning

### Comparison with Other Languages

| Feature | Cortex | Python | JavaScript | Haskell |
|---------|--------|--------|------------|---------|
| Function Calls | `func[args]` | `func(args)` | `func(args)` | `func args` |
| Blocks | `\| ... \^` | Indentation | `{ ... }` | Indentation |
| Assignment | `:=` | `=` | `=` | `=` |
| Types | Annotated | Dynamic | Dynamic | Static |

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

x = 10
result = factorial(x)
print(result)
```

**Cortex**:

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

let x := 10
let result := factorial[x]
print[result]
```

### From JavaScript

**JavaScript**:

```javascript
function calculateArea(length, width) {
    return length * width;
}

const x = 10;
const area = calculateArea(x, 5);
console.log(area);
```

**Cortex**:

```cortex
func calculate_area[length, width] |
  return[length * width]
^

let x := 10
let area := calculate_area[x, 5]
print[area]
```

### From Haskell

**Haskell**:

```haskell
add :: Int -> Int -> Int
add a b = a + b

factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)

main = print (factorial 5)
```

**Cortex** (Current):

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
  print[factorial[5]]
^

main[]
```

**Cortex** (Future with pattern matching):

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

func main[] |
  print[factorial[5]]
^

main[]
```

---

## Current Limitations

1. **No Type Annotations**: Cannot specify types explicitly yet
2. **No Arrays**: Cannot create or manipulate arrays
3. **No String Operations**: Limited string handling
4. **No For Loops**: Only while loops available
5. **No Modules**: No import system yet
6. **No Error Handling**: No try-catch mechanisms
7. **Limited Built-ins**: Only print and basic str functions

## Getting Started

1. **Install Dependencies**: See [run.md](run.md) for setup instructions
2. **Try Examples**: Check out [examples/](../examples/) directory
3. **Read Functions Guide**: See [functions.md](functions.md) for function details
4. **Compile Programs**: Use `python3 cortexc.py build` to create binaries

For more information, visit the [main documentation](README.md) or explore the [examples](../examples/).
