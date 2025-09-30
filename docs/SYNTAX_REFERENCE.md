# Cortex Language Syntax Reference

Complete syntax documentation matching the actual .ctx file format implementation.

## Table of Contents

1. [Program Structure](#program-structure)
2. [Variables & Constants](#variables--constants)
3. [Data Types](#data-types)
4. [Expressions](#expressions)
5. [Control Flow](#control-flow)
6. [Functions](#functions)
7. [Data Structures](#data-structures)
8. [Comments](#comments)
9. [Operators](#operators)
10. [Advanced Features](#advanced-features)

---

## Program Structure

### Basic Program

```cortex
// Any valid Cortex source file contains statements
let greeting := "Hello, Cortex!"
func main[] |
  print[greeting]
^
main[]
```

### Statement Types

- Variable declarations
- Function definitions  
- Expression statements
- Control flow statements

---

## Variables & Constants

### Variable Declaration (Mutable)

```cortex
let variable_name := value
let x := 42
let name := "Alice"
let is_active := true
let data := null
```

### Constant Declaration (Immutable)

```cortex
let constant_name :: value
let PI :: 3.14159
let CONFIG_NAME :: "production"
let MAX_SIZE :: 1000
```

### Type Annotations (Optional)

```cortex
let x: number := 42
let name: string := "Alice"
let flag: boolean := true
func compute[a: number, b: number]: number |
  return[a + b]
^
```

---

## Data Types

### Numbers

```cortex
let integer := 42
let float := 3.14159
let scientific := 1e5
let negative := 1e-10
let large := 2.5e12
```

### Strings

```cortex
let simple := "Hello world"
let with_escape := "He said \"Hello\""
let multiline // Not directly supported in literals, use concatenation
let empty := ""
```

### Booleans

```cortex
let truth := true
let falsity := false
```

### Null

```cortex
let empty := null
```

---

## Expressions

### Arithmetic Expressions

```cortex
let x := 10 + 5
let y := 20 - 8
let z := 4 * 6
let quotient := 15 / 3
let remainder := 17 % 5
let power := 2 ** 8
```

### Comparison Expressions

```cortex
let equal := a == b
let not_equal := a != b
let less_than := a < b
let less_equal := a <= b
let greater_than := a > b
let greater_equal := a >= b
```

### Logical Expressions

```cortex
let and_result := condition1 && condition2
let or_result := condition1 || condition2
let not_result := !condition
```

### Complex Expressions

```cortex
let complex := (a + b) * (c - d) + e ** f
let ternary_logic := (a > 5) && (b < 10) || (c == 42)
```

---

## Control Flow

### If Statements

```cortex
// Simple if
if [condition] |
  // then block
^

// If-else
if [condition] |
  // then block
^ else |
  // else block
^

// Nested if-else (if-else-if)
if [score >= 90] |
  print["Grade A"]
^ else |
  if [score >= 80] |
    print["Grade B"]
  ^ else |
    if [score >= 70] |
      print["Grade C"]
    ^ else |
      print["Grade F"]
    ^
  ^
^
```

### While Loops

```cortex
// Basic while loop
while [condition] |
  // loop body
^

// Counting loop
let counter := 1
while [counter <= 10] |
  print[counter]
  let counter := counter + 1
^

// Complex condition loop
while [data != null && counter < 100] |
  // process data
  let counter := counter + 1
^
```

### For Loops

```cortex
// Basic for loop (simplified implementation)
for [variable_name] |
  // loop body
^

// Note: Current implementation runs loop body once
// TODO: Implement proper iteration semantics
```

---

## Functions

### Function Definition

```cortex
func function_name[parameters] |
  // body
^

// Example
func greet[] |
  print["Hello, World!"]
^
```

### Function with Parameters

```cortex
func add[a, b] |
  return[a + b]
^

func multiply[x, y, z] |
  return[x * y * z]
^
```

### Function with Return Type

```cortex
func calculate[value]: number |
  return[value ** 2]
^

func get_message[]: string |
  return["Hello from function"]
^
```

### Function with Type Annotations

```cortex
func typed_add[a: number, b: number]: number |
  return[a + b]
^

func process_data[input: string, flag: boolean]: number |
  // function body
  return[42]
^
```

### Function Calls

```cortex
// Call with no arguments
greet[]

// Call with arguments
let result := add[5, 3]
let big_result := multiply[2, 4, 6]

// Call in expressions
let final := calculate[10] + get_message[]
```

### Recursive Functions

```cortex
func factorial[n] |
  if [n <= 1] |
    return[1]
  ^ else |
    return[n * factorial[n - 1]]
  ^
^

func fibonacci[n] |
  if [n <= 1] |
    return[n]
  ^ else |
    return[fibonacci[n - 1] + fibonacci[n - 2]]
  ^
^
```

---

## Data Structures

### Arrays

```cortex
// Simple arrays
let numbers := [1, 2, 3, 4, 5]
let decimals := [1.1, 2.2, 3.3]
let mixed := [1, "hello", true, 4.5]

// Nested arrays
let matrix := [[1, 2], [3, 4]]
let tensor := [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]

// Empty array
let empty_array := []

// Note: Array access and manipulation functions need implementation
```

### Dictionaries

```cortex
// Simple dictionary
let person := {
  "name": "Alice",
  "age": 30,
  "active": true
}

// Mixed value types
let config := {
  "host": "localhost",
  "port": 8080,
  "debug": false,
  "data": [1, 2, 3]
}

// Empty dictionary
let empty_dict := {}

// Note: Dictionary access syntax needs to be implemented
```

---

## Comments

### Single Line Comments

```cortex
// This is a single line comment
let value := 42  // This is an inline comment
```

### Multi-line Comments

```cortex
/*
This is a multi-line comment
It can span multiple lines
Good for documentation
*/

func documented_function[] |
  /*
    Function documentation:
    This function does important work
    Parameters: none
    Returns: void
  */
  print["Documented function"]
^
```

---

## Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |
| `%` | Modulo | `a % b` |
| `**` | Exponentiation | `a ** b` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal to | `a == b` |
| `!=` | Not equal to | `a != b` |
| `<` | Less than | `a < b` |
| `<=` | Less than or equal | `a <= b` |
| `>` | Greater than | `a > b` |
| `>=` | Greater than or equal | `a >= b` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|

`&&` - Logical AND - `a && b`
`||` - Logical OR - `a || b`
`!` - Logical NOT - `!a`

### Assignment Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `:=` | Mutable assignment | `let x := 42` |
| `::` | Constant assignment | `let x :: 42` |

---

## Advanced Features

### Special AI/ML Operators (Tokenized)

```cortex
// Matrix multiplication (token @ exists)
// let result := matrix_a @ matrix_b

// Gradient operator (token ∇ exists)  
// let gradient := ∇[loss_function]

// Partial derivative (token ∂ exists)
// let partial := ∂[f, x]
```

### Unary Operators

```cortex
let positive := +value
let negative := -value
let logical_not := !condition
```

### Operator Precedence

1. **Primary**: `(`, literals, identifiers, function calls
2. **Unary**: `+`, `-`, `!`
3. **Multiplicative**: `*`, `/`, `%`, `**`
4. **Additive**: `+`, `-`
5. **Comparison**: `<`, `<=`, `>`, `>=`
6. **Equality**: `==`, `!=`
7. **Logical AND**: `&&`
8. **Logical OR**: `||`

### String Concatenation

```cortex
let greeting := "Hello" + ", " + "World"
let message := "Value: " + value
let expression := "Result: " + (a + b)
```

### Null Handlings

```cortex
let data := null
if [data == null] |
  print["No data available"]
^ else |
  print["Data: " + data]
^
```

---

## Lexical Elements

### Identifiers

- Start with letter or underscore: `variable`, `_private`, `temp123`
- Contain letters, numbers, underscores: `myVariable`, `test_123`, `UPPERCASE`

### Keywords

- `let` - variable declaration
- `func` - function definition
- `if`, `else` - conditional statements
- `while`, `for` - loops
- `return` - function return
- `true`, `false` - boolean literals
- `null` - null literal

### Delimiters

- `[`, `]` - parameter lists, arrays
- `{`, `}` - dictionaries
- `|`, `^` - block delimiters
- `:`, `;` - type annotation, statement terminator
- `,` - parameter separator
- `//` - comment delimiter

---

## Complete Example Programs

### Hello World

```cortex
func main[] |
  print["Hello, Cortex!"]
^
main[]
```

### Arithmetic Calculator

```cortex
func add[a, b] |
  return[a + b]
^

func multiply[a, b] |
  return[a * b]
^

func main[] |
  let sum := add[10, 20]
  let product := multiply[3, 7]
  
  print["Sum: " + sum]
  print["Product: " + product]
^
main[]
```

### Factorial Calculator

```cortex
func factorial[n]: number |
  if [n <= 1] |
    return[1]
  ^ else |
    return[n * factorial[n - 1]]
  ^
^

func main[] |
  let n := 5
  let result := factorial[n]
  
  print["Factorial of " + n + " = " + result]
^
main[]
```

### Data Processing

```cortex
func process_numbers[data]: number |
  let sum := 0
  let count := 0
  
  while [count < len[data]] |
    let sum := sum + data[count]
    let count := count + 1
  ^
  
  return[sum / len[data]]
^

func main[] |
  let numbers := [1, 2, 3, 4, 5]
  let average := process_numbers[numbers]
  
  print["Average: " + average]
^
main[]
```

---

## Implementation Status

### ✅ Fully Implemented

- Variable declarations (`let...:=` vs `let...::`)
- Type annotations (`: type`)
- All arithmetic and logical operators
- Control flow (if/else, while loops)
- Function definitions and calls
- Basic data structures parsing
- Comments handling

### 🔄 Partially Implemented

- Arrays (parsed but code generation TODO)
- Dictionaries (parsed but code generation TODO)
- For loops (parsed but simple implementation)
- Return values (parsed but not fully handled)

### ❌ Not Yet Implemented

- Array/dictionary access operations
- Built-in function library
- AI/ML operator implementations
- Runtime type checking
- Advanced error handling

---

This syntax reference provides complete coverage of the Cortex language as implemented, matching the exact format used in the .ctx example files.
