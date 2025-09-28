# Numerical Computing

Cortex is designed from the ground up for numerical computing and AI/ML workloads, with built-in support for tensors, linear algebra, and mathematical operations.

## Tensor Operations

### Basic Tensor Creation

```cortex
// Scalar
let scalar := 42.0

// 1D tensor (vector)
let vector := [1.0, 2.0, 3.0, 4.0]
let zeros_vec := zeros[10]
let ones_vec := ones[5]
let range_vec := arange[0, 10, 2]  // [0, 2, 4, 6, 8]

// 2D tensor (matrix)
let matrix := [[1.0, 2.0], [3.0, 4.0]]
let identity := eye[3]              // 3x3 identity
let random := randn[2, 3]           // 2x3 random normal
let uniform := rand[2, 3]           // 2x3 uniform [0,1]

// Higher-dimensional tensors
let tensor3d := randn[2, 3, 4]      // 2x3x4 tensor
let tensor4d := randn[1, 2, 3, 4]   // 1x2x3x4 tensor (batch, channels, height, width)
```

### Tensor Properties

```cortex
let tensor := randn[3, 4, 5]

let shape := tensor.shape           // [3, 4, 5]
let ndim := tensor.ndim             // 3
let size := tensor.size             // 60
let dtype := tensor.dtype           // "float64"
```

### Element-wise Operations

```cortex
let a := [1.0, 2.0, 3.0]
let b := [4.0, 5.0, 6.0]

// Arithmetic
let sum := a + b                    // [5.0, 7.0, 9.0]
let diff := a - b                   // [-3.0, -3.0, -3.0]
let prod := a * b                   // [4.0, 10.0, 18.0]
let quot := a / b                   // [0.25, 0.4, 0.5]

// Scalar operations
let scaled := a * 2.0               // [2.0, 4.0, 6.0]
let shifted := a + 1.0              // [2.0, 3.0, 4.0]

// Mathematical functions
let exp_a := exp[a]                 // [e, e², e³]
let log_a := log[a]                 // [0, ln(2), ln(3)]
let sqrt_a := sqrt[a]               // [1, √2, √3]
let pow_a := a ** 2                 // [1, 4, 9]
```

## Linear Algebra Operations

### Matrix Operations

```cortex
let A := [[1.0, 2.0], [3.0, 4.0]]
let B := [[5.0, 6.0], [7.0, 8.0]]
let x := [1.0, 2.0]

// Matrix multiplication
let C := A @ B                      // [[19, 22], [43, 50]]

// Matrix-vector multiplication
let y := A @ x                      // [5.0, 11.0]

// Element-wise matrix operations
let sum := A + B                    // [[6, 8], [10, 12]]
let prod := A * B                   // [[5, 12], [21, 32]]

// Matrix properties
let det := det[A]                   // -2.0
let trace := trace[A]               // 5.0
let transpose := A.T                // [[1, 3], [2, 4]]
let inverse := inv[A]               // [[-2, 1], [1.5, -0.5]]
```

### Advanced Linear Algebra

```cortex
let matrix := randn[4, 4]

// Eigenvalue decomposition
let eigenvals, eigenvecs := eig[matrix]

// SVD decomposition
let U, S, V := svd[matrix]

// QR decomposition
let Q, R := qr[matrix]

// Cholesky decomposition (for positive definite matrices)
let L := cholesky[matrix @ matrix.T]
```

## Statistical Operations

### Descriptive Statistics

```cortex
let data := [1.0, 2.0, 3.0, 4.0, 5.0]

let mean_val := mean[data]          // 3.0
let median_val := median[data]      // 3.0
let std_val := std[data]            // 1.58
let var_val := var[data]            // 2.5
let min_val := min[data]            // 1.0
let max_val := max[data]            // 5.0
let sum_val := sum[data]            // 15.0
```

### Tensor Statistics

```cortex
let matrix := randn[3, 4]

// Along specific dimensions
let mean_axis0 := mean[matrix, axis=0]    // Mean along rows
let mean_axis1 := mean[matrix, axis=1]    // Mean along columns
let mean_all := mean[matrix]              // Overall mean

// Other operations
let max_axis0 := max[matrix, axis=0]
let sum_axis1 := sum[matrix, axis=1]
```

## Broadcasting

Cortex supports NumPy-style broadcasting for tensor operations:

```cortex
let matrix := [[1, 2, 3], [4, 5, 6]]  // 2x3
let vector := [10, 20, 30]             // 3

// Broadcasting: vector is broadcast to match matrix
let result := matrix + vector          // [[11, 22, 33], [14, 25, 36]]

// Scalar broadcasting
let scaled := matrix * 2               // [[2, 4, 6], [8, 10, 12]]
```

## Neural Network Operations

### Activation Functions

```cortex
let x := [-2.0, -1.0, 0.0, 1.0, 2.0]

// Common activations
let relu_out := relu[x]               // [0, 0, 0, 1, 2]
let sigmoid_out := sigmoid[x]         // [0.12, 0.27, 0.5, 0.73, 0.88]
let tanh_out := tanh[x]               // [-0.96, -0.76, 0, 0.76, 0.96]
let softmax_out := softmax[x]         // [0.012, 0.031, 0.085, 0.231, 0.631]
```

### Loss Functions

```cortex
let predictions := [0.8, 0.2, 0.9]
let targets := [1.0, 0.0, 1.0]

// Mean Squared Error
let mse_loss := mse[predictions, targets]

// Cross-entropy
let ce_loss := cross_entropy[predictions, targets]

// Binary cross-entropy
let bce_loss := binary_cross_entropy[predictions, targets]
```

### Gradient Operations

```cortex
// Automatic differentiation
let x := tensor[2.0, requires_grad=true]
let y := x ** 2 + 3 * x + 1
let grad_y := ∇[y, x]                // 2*x + 3 = 7

// Gradient of loss w.r.t. parameters
let loss := mse[predictions, targets]
let grad_weights := ∇[loss, weights]
let grad_bias := ∇[loss, bias]
```

## Optimization Operations

### Gradient Descent

```cortex
func gradient_descent[params, gradients, lr] |
  return[params - lr * gradients]
^

// Usage
let new_params := gradient_descent[old_params, grad, 0.01]
```

### Adam Optimizer

```cortex
func adam_step[params, gradients, optimizer_state, lr] |
  let m := optimizer_state["m"]
  let v := optimizer_state["v"]
  let beta1 := 0.9
  let beta2 := 0.999
  let epsilon := 1e-8
  
  // Update biased first moment estimate
  m := beta1 * m + (1 - beta1) * gradients
  
  // Update biased second raw moment estimate
  v := beta2 * v + (1 - beta2) * (gradients ** 2)
  
  // Compute bias-corrected first moment estimate
  let m_hat := m / (1 - beta1 ** optimizer_state["t"])
  
  // Compute bias-corrected second raw moment estimate
  let v_hat := v / (1 - beta2 ** optimizer_state["t"])
  
  // Update parameters
  let new_params := params - lr * m_hat / (sqrt[v_hat] + epsilon)
  
  return[new_params, {"m": m, "v": v, "t": optimizer_state["t"] + 1}]
^
```

## Performance Optimizations

### Vectorized Operations

```cortex
// Good: vectorized
let result := activation[weights @ inputs + bias]

// Avoid: manual loops
// let result := []
// for [i in range[0, len[inputs]]] |
//   result := append[result, activation[weights[i] * inputs[i] + bias[i]]]
// ^
```

### Memory-Efficient Operations

```cortex
// In-place operations (modify existing tensor)
let tensor := randn[1000, 1000]
tensor += 1.0                       // In-place addition
tensor *= 2.0                       // In-place multiplication

// Out-of-place operations (create new tensor)
let new_tensor := tensor + 1.0      // Creates new tensor
```

### Lazy Evaluation

```cortex
// Defer computation until needed
let lazy_result := lazy[expensive_operation[large_tensor]]

// Force computation
let result := force[lazy_result]
```

## Best Practices

1. **Use appropriate data types**:

   ```cortex
   let float_tensor := tensor[1.0, 2.0, 3.0]  // float64
   let int_tensor := tensor[1, 2, 3]           // int64
   ```

2. **Leverage broadcasting** for efficient operations:

   ```cortex
   let matrix := randn[100, 784]
   let bias := randn[784]
   let result := matrix + bias  // Broadcasting
   ```

3. **Use in-place operations** when possible:

   ```cortex
   tensor += 1.0  // In-place
   // vs
   tensor = tensor + 1.0  // Creates new tensor
   ```

4. **Profile numerical operations** for performance:

   ```cortex
   let start_time := time[]
   let result := expensive_computation[data]
   let end_time := time[]
   print["Computation took: " + str[end_time - start_time] + " seconds"]
   ```

5. **Use appropriate precision** for your use case:

   ```cortex
   let high_precision := tensor[1.0, dtype="float64"]
   let low_precision := tensor[1.0, dtype="float32"]  // Faster, less memory
   ```
