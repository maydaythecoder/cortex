# Cortex Syntax Overview

Cortex uses a **unique bracket-based syntax** designed for clarity and AI/ML workflows. The language emphasizes readability while maintaining the performance of compiled code with distinctive syntax elements.

## Core Syntax Elements

### Comments

```cortex
// Single-line comment
/* Multi-line
   comment */
```

### Variables and Assignment

```cortex
// Type inference with :=
let x := 42
let name := "Cortex"
let is_ready := true

// Explicit typing with : (optional)
let count: int := 0
let pi: float := 3.14159
let data: tensor := [1, 2, 3, 4]

// Immutable constants with ::
let MAX_EPOCHS :: 1000
let LEARNING_RATE :: 0.01
```

### Block Structure

Cortex uses `|` and `^` to define code blocks, with `[]` for parameter lists:

```cortex
if [condition] |
  // code block
  print["Hello"]
^

// Nested blocks
if [x > 0] |
  if [y < 10] |
    print["Both conditions met"]
  ^
^
```

### Unique Syntax Features

#### Function Calls with Brackets

```cortex
// All function calls use brackets instead of parentheses
let result := add[5, 3]
let prediction := model[input_data]
let loss := mse[predictions, targets]
```

#### Array/Tensor Indexing with Brackets

```cortex
let data := [1, 2, 3, 4, 5]
let first := data[0]        // 1
let last := data[-1]        // 5
let slice := data[1:3]      // [2, 3]
```

#### Dictionary Access with Brackets

```cortex
let config := {
  "learning_rate": 0.01,
  "batch_size": 32
}
let lr := config["learning_rate"]
```

## Data Types

### Primitive Types

- `int` - 64-bit integers
- `float` - 64-bit floating point
- `bool` - Boolean values (`true`/`false`)
- `string` - UTF-8 strings

### AI/ML Types

- `tensor` - Multi-dimensional arrays
- `matrix` - 2D tensors
- `vector` - 1D tensors
- `dataset` - Structured data containers

### Collections

```cortex
// Arrays
let numbers := [1, 2, 3, 4, 5]
let matrix := [[1, 2], [3, 4]]

// Dictionaries (for datasets)
let data := {
  "features": [1.0, 2.0, 3.0],
  "labels": [0, 1, 0],
  "size": 3
}
```

## Operators

### Arithmetic

```cortex
let a := 10
let b := 3

let sum := a + b        // 13
let diff := a - b       // 7
let prod := a * b       // 30
let quot := a / b       // 3.333...
let mod := a % b        // 1
let pow := a ** b       // 1000
```

### Comparison

```cortex
let x := 5
let y := 10

x == y    // false
x != y    // true
x < y     // true
x <= y    // true
x > y     // false
x >= y    // true
```

### Logical

```cortex
let a := true
let b := false

a && b    // false (AND)
a || b    // true (OR)
!a        // false (NOT)
```

### Tensor Operations (Unique to Cortex)

```cortex
let a := [1, 2, 3]
let b := [4, 5, 6]

let sum := a + b        // [5, 7, 9] (element-wise)
let dot := a @ b        // 32 (dot product)
let scale := a * 2      // [2, 4, 6] (scalar multiplication)
let outer := a ⊗ b      // Outer product
let norm := |a|         // L2 norm
```

### Unique AI/ML Operators

```cortex
let predictions := [0.8, 0.2, 0.9]
let targets := [1, 0, 1]

// Gradient operator (built-in)
let grad_w := ∇[loss, weights]

// Partial derivative
let ∂f_∂x := ∂[f, x]

// Tensor contraction
let result := a ⟨i,j⟩ b[j,k]  // Einstein notation
```

## String Operations

```cortex
let name := "Cortex"
let version := "1.0"

let full := name + " " + version    // "Cortex 1.0"
let length := len[name]             // 6
let upper := upper[name]            // "CORTEX"
let lower := lower[name]            // "cortex"

// String interpolation with {}
let message := "Hello {name}, version {version}"  // "Hello Cortex, version 1.0"
```

## Type Conversion

```cortex
let x := 42
let s := str[x]         // "42"
let f := float[x]       // 42.0
let b := bool[x]        // true (non-zero)

let y := "3.14"
let num := float[y]     // 3.14

// Tensor conversions
let list := tensor[[1, 2, 3]]       // Convert to tensor
let matrix := matrix[[[1, 2], [3, 4]]]  // 2D tensor
```

## Unique Control Flow Syntax

### Pattern Matching

```cortex
match [value] |
  case [0] -> print["Zero"]
  case [1] -> print["One"]
  case [n: int] -> print["Number: " + str[n]]
  case [s: string] -> print["String: " + s]
  default -> print["Unknown"]
^
```

### Guard Clauses

```cortex
func process[x] |
  guard [x > 0] else return[null]
  guard [x < 100] else return[null]
  
  return[x * 2]
^
```

## Scope and Lifetime

Variables are block-scoped and follow lexical scoping rules:

```cortex
let global := 100

func example[] |
  let local := 200
  print[global]    // 100
  print[local]     // 200
  
  if [true] |
    let nested := 300
    print[local]   // 200
    print[nested]  // 300
  ^
  // print[nested] // Error: out of scope
^
```

## Best Practices

1. **Use descriptive variable names** for AI/ML contexts:

   ```cortex
   let learning_rate := 0.01
   let batch_size := 32
   let num_epochs := 100
   ```

2. **Leverage type inference** but be explicit for complex types:

   ```cortex
   let x := 42                    // inferred as int
   let model: neural_network := create_model[]
   ```

3. **Use meaningful block structure** for readability:

   ```cortex
   if [loss < threshold] |
     print["Converged!"]
     return[model]
   ^
   ```

4. **Prefer tensor operations** over loops when possible:

   ```cortex
   // Good: vectorized
   let result := activation[weights @ inputs + bias]
   
   // Avoid: manual loops
   // for [i := 0; i < len[inputs]; i := i + 1] | ... ^
   ```

5. **Use constants for configuration**:

   ```cortex
   let MAX_EPOCHS :: 1000
   let CONVERGENCE_THRESHOLD :: 0.001
   ```

6. **Leverage unique operators** for mathematical expressions:

   ```cortex
   // Use gradient operator
   let gradients := ∇[loss, parameters]
   
   // Use Einstein notation for tensor operations
   let output := weights ⟨i,j⟩ inputs[j]
   ```
