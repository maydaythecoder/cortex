# Functions

Functions in Cortex are first-class citizens designed for AI/ML workflows. They support static typing, type inference, and functional programming patterns.

## Basic Function Syntax

### Function Declaration

```cortex
func add[a: int, b: int] -> int |
  return[a + b]
^

// Type inference
func multiply[a, b] |
  return[a * b]
^

// No return value
func print_hello[] |
  print["Hello, Cortex!"]
^
```

### Function Calls

```cortex
let result := add[5, 3]        // 8
let product := multiply[4, 7]  // 28
print_hello[]                  // "Hello, Cortex!"
```

## AI/ML Function Patterns

### Model Definition

```cortex
func linear_layer[inputs: tensor, weights: tensor, bias: tensor] -> tensor |
  return[inputs @ weights + bias]
^

func relu[x: tensor] -> tensor |
  return[max[x, 0]]
^

func sigmoid[x: tensor] -> tensor |
  return[1 / (1 + exp[-x])]
^
```

### Loss Functions

```cortex
func mse[predictions: tensor, targets: tensor] -> float |
  let diff := predictions - targets
  return[mean[diff ** 2]]
^

func cross_entropy[logits: tensor, labels: tensor] -> float |
  let probs := softmax[logits]
  return[-mean[labels * log[probs + 1e-8]]]
^
```

### Training Functions

```cortex
func train_epoch[model: neural_network, data: dataset, lr: float] -> float |
  let total_loss := 0.0
  let num_batches := 0
  
  for [batch in data.batches] |
    let predictions := model[batch.inputs]
    let loss := mse[predictions, batch.targets]
    
    // Backward pass
    let gradients := grad[loss, model.parameters]
    update_parameters[model, gradients, lr]
    
    total_loss := total_loss + loss
    num_batches := num_batches + 1
  ^
  
  return[total_loss / num_batches]
^
```

## Advanced Function Features

### Default Parameters

```cortex
func create_model[input_size: int, hidden_size: int := 128, output_size: int := 10] -> neural_network |
  let model := neural_network[]
  model.add_layer[dense[input_size, hidden_size]]
  model.add_layer[relu[]]
  model.add_layer[dense[hidden_size, output_size]]
  return[model]
^

// Usage
let model1 := create_model[784]                    // hidden=128, output=10
let model2 := create_model[784, 256]               // output=10
let model3 := create_model[784, 256, 5]            // all specified
```

### Variadic Functions

```cortex
func sum_all[numbers: ...int] -> int |
  let total := 0
  for [num in numbers] |
    total := total + num
  ^
  return[total]
^

let result := sum_all[1, 2, 3, 4, 5]    // 15
```

### Higher-Order Functions

```cortex
func apply_function[func, data: tensor] -> tensor |
  let result := []
  for [item in data] |
    result := append[result, func[item]]
  ^
  return[result]
^

// Usage with lambda functions
let squared := apply_function[lambda[x] -> x * x, [1, 2, 3, 4]]    // [1, 4, 9, 16]
```

### Function Composition

```cortex
func compose[f, g] |
  return[lambda[x] -> f[g[x]]]
^

let add_one := lambda[x] -> x + 1
let multiply_two := lambda[x] -> x * 2
let add_one_then_double := compose[multiply_two, add_one]

let result := add_one_then_double[5]    // (5 + 1) * 2 = 12
```

## Closures and Lexical Scoping

```cortex
func create_counter[initial: int] |
  let count := initial
  
  return[lambda[] |
    count := count + 1
    return[count]
  ^]
^

let counter := create_counter[0]
print[counter[]]    // 1
print[counter[]]    // 2
print[counter[]]    // 3
```

## Recursive Functions

### Factorial

```cortex
func factorial[n: int] -> int |
  if [n <= 1] |
    return[1]
  ^
  return[n * factorial[n - 1]]
^
```

### Neural Network Forward Pass (Recursive)

```cortex
func forward_pass[layer: layer, input: tensor] -> tensor |
  if [layer == null] |
    return[input]
  ^
  
  let output := layer.activation[layer.weights @ input + layer.bias]
  return[forward_pass[layer.next, output]]
^
```

## Function Overloading

```cortex
// Different parameter types
func process[x: int] -> int |
  return[x * 2]
^

func process[x: float] -> float |
  return[x * 2.0]
^

func process[x: tensor] -> tensor |
  return[x * 2]
^

// Usage
let a := process[5]           // int version
let b := process[3.14]        // float version
let c := process[[1, 2, 3]]   // tensor version
```

## Generic Functions

```cortex
func map[T, U][func: T -> U, data: list[T]] -> list[U] |
  let result := []
  for [item in data] |
    result := append[result, func[item]]
  ^
  return[result]
^

// Usage
let numbers := [1, 2, 3, 4, 5]
let strings := map[int, string][lambda[x] -> str[x], numbers]    // ["1", "2", "3", "4", "5"]
```

## Error Handling in Functions

```cortex
func safe_divide[a: float, b: float] -> float |
  if [b == 0.0] |
    print["Error: Division by zero"]
    return[0.0]
  ^
  return[a / b]
^

func load_model[path: string] -> neural_network |
  if [not exists[path]] |
    print["Error: Model file not found: " + path]
    return[null]
  ^
  
  try |
    return[deserialize[read_file[path]]]
  ^ catch [error] |
    print["Error loading model: " + str[error]]
    return[null]
  ^
^
```

## Performance Optimization

### Inline Functions

```cortex
inline func square[x: float] -> float |
  return[x * x]
^

// Compiler will inline this function call
let result := square[5.0]
```

### Memoization

```cortex
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

// Usage
let expensive_func := memoize[lambda[x] -> 
  // Expensive computation
  sleep[1]
  return[x * x]
^]

let result1 := expensive_func[5]    // Takes 1 second
let result2 := expensive_func[5]    // Returns cached result instantly
```

## Best Practices

1. **Use descriptive function names** for AI/ML operations:

   ```cortex
   func compute_gradient_descent[model, data, lr]  // Good
   func func1[a, b, c]                            // Bad
   ```

2. **Leverage type annotations** for complex AI/ML types:

   ```cortex
   func train_model[model: neural_network, data: dataset] -> float
   ```

3. **Keep functions focused** on single responsibilities:

   ```cortex
   // Good: focused functions
   func normalize_data[data: tensor] -> tensor
   func compute_loss[predictions: tensor, targets: tensor] -> float
   
   // Bad: doing too much
   func process_everything[data, model, optimizer, ...]
   ```

4. **Use functional patterns** for data transformations:

   ```cortex
   let processed_data := data
     |> normalize
     |> add_noise[0.1]
     |> augment[flip, rotate]
   ```
