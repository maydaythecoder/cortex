# Control Flow and Loops

Cortex provides powerful control flow constructs designed for AI/ML workflows, with emphasis on numerical computing and data processing.

## Conditional Statements

### Basic If-Else

```cortex
let x := 10

if [x > 5] |
  print["x is greater than 5"]
^ else |
  print["x is 5 or less"]
^
```

### If-Else If Chains

```cortex
let score := 85

if [score >= 90] |
  print["Grade: A"]
^ else if [score >= 80] |
  print["Grade: B"]
^ else if [score >= 70] |
  print["Grade: C"]
^ else |
  print["Grade: F"]
^
```

### Ternary Operator

```cortex
let x := 10
let result := x > 5 ? "high" : "low"    // "high"
```

## Loop Constructs

### For Loops

```cortex
// Traditional for loop
for [i := 0; i < 10; i := i + 1] |
  print["Iteration: " + str[i]]
^

// Range-based for loop
for [i in range[0, 10]] |
  print["Value: " + str[i]]
^

// Step-based range
for [i in range[0, 10, 2]] |
  print["Even: " + str[i]]    // 0, 2, 4, 6, 8
^
```

### While Loops

```cortex
let count := 0
while [count < 5] |
  print["Count: " + str[count]]
  count := count + 1
^
```

### Do-While Loops

```cortex
let x := 0
do |
  print["Executed at least once"]
  x := x + 1
^ while [x < 3]
```

## AI/ML Specific Loops

### Training Epochs

```cortex
let num_epochs := 100
let learning_rate := 0.01

for [epoch in range[0, num_epochs]] |
  let loss := train_epoch[model, data, learning_rate]
  print["Epoch " + str[epoch] + ": Loss = " + str[loss]]
  
  if [loss < 0.001] |
    print["Converged early at epoch " + str[epoch]]
    break
  ^
^
```

### Batch Processing

```cortex
let batch_size := 32
let num_batches := len[dataset] / batch_size

for [batch_idx in range[0, num_batches]] |
  let start := batch_idx * batch_size
  let end := start + batch_size
  let batch := dataset[start:end]
  
  let loss := train_batch[model, batch]
  print["Batch " + str[batch_idx] + ": Loss = " + str[loss]]
^
```

### Hyperparameter Grid Search

```cortex
let learning_rates := [0.001, 0.01, 0.1]
let batch_sizes := [16, 32, 64]

for [lr in learning_rates] |
  for [batch_size in batch_sizes] |
    let model := create_model[]
    let score := train_and_evaluate[model, lr, batch_size]
    print["LR=" + str[lr] + ", Batch=" + str[batch_size] + ", Score=" + str[score]]
  ^
^
```

## Loop Control Statements

### Break and Continue

```cortex
for [i in range[0, 10]] |
  if [i == 3] |
    continue    // Skip iteration 3
  ^
  
  if [i == 7] |
    break       // Exit loop at iteration 7
  ^
  
  print["Processing: " + str[i]]
^
```

### Early Exit Patterns

```cortex
// Early convergence detection
for [epoch in range[0, 1000]] |
  let loss := train_epoch[model, data]
  
  if [loss < convergence_threshold] |
    print["Converged at epoch " + str[epoch]]
    break
  ^
  
  if [loss > divergence_threshold] |
    print["Model diverging, stopping training"]
    break
  ^
^
```

## Vectorized Operations (Preferred)

Instead of manual loops, prefer vectorized operations when possible:

```cortex
// Instead of this:
let result := []
for [i in range[0, len[data]]] |
  result := append[result, data[i] * 2]
^

// Do this:
let result := data * 2    // Vectorized multiplication
```

### Tensor Operations

```cortex
let weights := [[1, 2], [3, 4]]
let inputs := [5, 6]

// Matrix-vector multiplication
let outputs := weights @ inputs    // [17, 39]

// Element-wise operations
let activations := relu[outputs]   // [17, 39]
```

## Nested Loops and Complex Control Flow

### Cross-Validation

```cortex
let k_folds := 5
let scores := []

for [fold in range[0, k_folds]] |
  let train_data, val_data := split_fold[dataset, fold, k_folds]
  let model := create_model[]
  
  // Training loop
  for [epoch in range[0, 100]] |
    let loss := train_epoch[model, train_data]
    
    if [epoch % 10 == 0] |
      print["Fold " + str[fold] + ", Epoch " + str[epoch] + ": " + str[loss]]
    ^
  ^
  
  let score := evaluate[model, val_data]
  scores := append[scores, score]
^

let avg_score := mean[scores]
print["Cross-validation score: " + str[avg_score]]
```

### Adaptive Learning Rate

```cortex
let base_lr := 0.01
let patience := 10
let no_improvement := 0
let best_loss := float["inf"]

for [epoch in range[0, 1000]] |
  let current_lr := base_lr * (0.9 ** (epoch / 100))
  let loss := train_epoch[model, data, current_lr]
  
  if [loss < best_loss] |
    best_loss := loss
    no_improvement := 0
  ^ else |
    no_improvement := no_improvement + 1
  ^
  
  if [no_improvement >= patience] |
    print["No improvement for " + str[patience] + " epochs, stopping"]
    break
  ^
^
```

## Performance Considerations

1. **Prefer vectorized operations** over explicit loops
2. **Use break/continue** to avoid unnecessary computation
3. **Consider parallel processing** for independent iterations
4. **Profile nested loops** - they can be performance bottlenecks

```cortex
// Efficient: vectorized
let predictions := model[all_inputs]

// Less efficient: loop-based
let predictions := []
for [input in all_inputs] |
  predictions := append[predictions, model[input]]
^
```
