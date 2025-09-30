# Tensor Primitives

Cortex's tensor primitives provide a comprehensive foundation for numerical computing and machine learning, designed from the ground up for AI/ML workloads with performance and expressiveness as core principles.

## Overview

Tensor primitives in Cortex enable:

- **Multi-dimensional arrays** with automatic shape inference
- **Memory-efficient storage** with contiguous layouts and lazy evaluation
- **Device-agnostic operations** supporting CPU, GPU, and TPU
- **Automatic differentiation** with computational graphs
- **High-performance operations** with BLAS/LAPACK integration

## Core Tensor Architecture

### Runtime Representation

```rust
pub enum Value {
    // Existing primitives
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Dictionary(HashMap<String, Value>),
    
    // NEW: Tensor primitives
    Tensor(Tensor),           // Multi-dimensional arrays
    SparseTensor(SparseTensor), // Memory-efficient sparse tensors
    LazyTensor(LazyTensor),   // Deferred computation tensors
}

pub struct Tensor {
    data: Vec<f64>,           // Contiguous memory storage
    shape: Vec<usize>,        // Dimensions [batch, height, width, channels]
    strides: Vec<usize>,      // Memory layout optimization
    dtype: DataType,          // float32, float64, int32, int64
    device: Device,           // CPU, GPU, TPU
    requires_grad: bool,      // Automatic differentiation flag
    grad_fn: Option<Box<dyn GradientFunction>>, // Gradient computation graph
}
```

### AST Representation

```rust
pub enum Expression {
    // Existing expressions
    Literal(Literal),
    Identifier(Identifier),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Array(Array),
    Dictionary(Dictionary),
    Index(IndexExpression),
    Range(RangeExpression),
    
    // NEW: Tensor expressions
    TensorLiteral(TensorLiteral),     // Tensor creation
    TensorOperation(TensorOperation), // Tensor operations
    TensorSlice(TensorSlice),         // Tensor slicing
    TensorBroadcast(TensorBroadcast), // Broadcasting operations
}
```

## Tensor Creation

### Basic Tensor Creation

```cortex
// 1D tensor (vector)
let vector := tensor[1, 2, 3, 4, 5]
let zeros_vec := zeros[10]           // [0, 0, 0, ...]
let ones_vec := ones[5]              // [1, 1, 1, 1, 1]
let range_vec := arange[0, 10, 2]    // [0, 2, 4, 6, 8]

// 2D tensor (matrix)
let matrix := tensor[[1, 2], [3, 4]]
let identity := eye[3]               // 3x3 identity matrix
let random := randn[2, 3]           // 2x3 random normal
let uniform := rand[2, 3]           // 2x3 uniform [0,1]

// Higher-dimensional tensors
let tensor3d := tensor[[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]]
let tensor4d := randn[1, 2, 3, 4]   // 1x2x3x4 tensor (batch, channels, height, width)
```

### Advanced Tensor Creation

```cortex
// With explicit shape and data type
let tensor := tensor[[1, 2, 3, 4], shape: [2, 2], dtype: "float32"]

// With device specification
let gpu_tensor := tensor[[1, 2, 3, 4], device: "cuda"]

// With gradient tracking enabled
let grad_tensor := tensor[[1, 2, 3, 4], requires_grad: true]

// From existing arrays
let array_data := [1, 2, 3, 4, 5, 6]
let reshaped := tensor[array_data, shape: [2, 3]]
```

### Sparse Tensor Creation

```cortex
// Sparse representation for large, mostly-zero tensors
let sparse_tensor := sparse[
  indices: [[0, 0], [1, 2], [2, 1]],
  values: [1.0, 2.0, 3.0],
  shape: [3, 3]
]

// Convert dense to sparse
let dense := randn[1000, 1000]
let sparse := to_sparse[dense, threshold: 1e-6]
```

## Tensor Properties and Operations

### Tensor Properties

```cortex
let tensor := randn[3, 4, 5]

let shape := tensor.shape           // [3, 4, 5]
let ndim := tensor.ndim             // 3
let size := tensor.size             // 60
let dtype := tensor.dtype           // "float64"
let device := tensor.device         // "cpu"
let requires_grad := tensor.requires_grad // false
```

### Element-wise Operations

```cortex
let a := tensor[1, 2, 3]
let b := tensor[4, 5, 6]

// Arithmetic operations
let sum := a + b                    // [5, 7, 9]
let diff := a - b                   // [-3, -3, -3]
let prod := a * b                   // [4, 10, 18]
let quot := a / b                   // [0.25, 0.4, 0.5]

// Scalar operations
let scaled := a * 2.0               // [2, 4, 6]
let shifted := a + 1.0              // [2, 3, 4]

// Mathematical functions
let exp_a := exp[a]                 // [e, e², e³]
let log_a := log[a]                 // [0, ln(2), ln(3)]
let sqrt_a := sqrt[a]               // [1, √2, √3]
let pow_a := a ** 2                 // [1, 4, 9]
```

## Linear Algebra Operations

### Matrix Operations

```cortex
let A := tensor[[1, 2], [3, 4]]
let B := tensor[[5, 6], [7, 8]]
let x := tensor[1, 2]

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

## Broadcasting

Cortex supports NumPy-style broadcasting for tensor operations:

```cortex
let matrix := tensor[[[1, 2, 3], [4, 5, 6]]]  // 2x3
let vector := tensor[10, 20, 30]              // 3

// Broadcasting: vector is broadcast to match matrix
let result := matrix + vector          // [[11, 22, 33], [14, 25, 36]]

// Scalar broadcasting
let scaled := matrix * 2               // [[2, 4, 6], [8, 10, 12]]

// Shape broadcasting
let tensor_a := randn[1, 3, 4]        // 1x3x4
let tensor_b := randn[2, 1, 4]        // 2x1x4
let broadcasted := tensor_a + tensor_b // 2x3x4
```

## Tensor Manipulation

### Reshaping and Slicing

```cortex
let tensor := randn[2, 3, 4]

// Reshaping
let reshaped := reshape[tensor, [6, 4]]
let flattened := flatten[tensor]       // [24] (1D)

// Transposing
let transposed := transpose[tensor]    // [4, 3, 2]
let permuted := permute[tensor, [2, 0, 1]] // [4, 2, 3]

// Slicing
let slice1 := tensor[0, :, :]         // First matrix [3, 4]
let slice2 := tensor[:, 1, :]         // Second row of each matrix [2, 4]
let slice3 := tensor[0:1, 1:3, 2:4]   // Specific slice [1, 2, 2]

// Indexing
let element := tensor[0, 1, 2]        // Single element
let row := tensor[0, 1]               // Row vector [4]
```

### Concatenation and Splitting

```cortex
let a := tensor[[1, 2], [3, 4]]       // 2x2
let b := tensor[[5, 6], [7, 8]]       // 2x2

// Concatenation
let concat_0 := concat[a, b, axis: 0] // [[1, 2], [3, 4], [5, 6], [7, 8]] (4x2)
let concat_1 := concat[a, b, axis: 1] // [[1, 2, 5, 6], [3, 4, 7, 8]] (2x4)

// Stacking
let stacked := stack[a, b, axis: 0]   // [[[1, 2], [3, 4]], [[5, 6], [7, 8]]] (2x2x2)

// Splitting
let split_tensors := split[concat_0, 2, axis: 0] // [a, b]
```

## Reduction Operations

### Statistical Operations

```cortex
let tensor := tensor[[[1, 2, 3], [4, 5, 6]]]  // 2x3

// Global reductions
let sum_all := sum[tensor]            // 21
let mean_all := mean[tensor]          // 3.5
let max_all := max[tensor]            // 6
let min_all := min[tensor]            // 1

// Axis-wise reductions
let sum_axis0 := sum[tensor, axis: 0] // [5, 7, 9] (sum along rows)
let sum_axis1 := sum[tensor, axis: 1] // [6, 15] (sum along columns)

// Other reductions
let norm := norm[tensor]              // Frobenius norm
let argmax := argmax[tensor]          // Index of maximum element
let argmin := argmin[tensor]          // Index of minimum element
```

## Automatic Differentiation

### Gradient Computation

```cortex
// Enable gradient tracking
let x := tensor[2.0, requires_grad: true]
let y := x ** 2 + 3 * x + 1

// Compute gradients
let grad_y := ∇[y, x]                // 2*x + 3 = 7

// Multiple variables
let a := tensor[1.0, requires_grad: true]
let b := tensor[2.0, requires_grad: true]
let c := a * b + a ** 2

let grad_c := ∇[c, a, b]             // [b + 2*a, a] = [4, 1]
```

### Computational Graphs

```cortex
// Complex computation with automatic differentiation
let weights := randn[784, 128, requires_grad: true]
let bias := zeros[128, requires_grad: true]
let inputs := randn[32, 784]

let hidden := relu[inputs @ weights + bias]
let output := softmax[hidden @ output_weights + output_bias]
let loss := cross_entropy[output, targets]

// Compute all gradients
let gradients := ∇[loss, weights, bias]
```

## Performance Optimizations

### Memory-Efficient Operations

```cortex
// In-place operations (modify existing tensor)
let tensor := randn[1000, 1000]
tensor += 1.0                       // In-place addition
tensor *= 2.0                       // In-place multiplication

// Out-of-place operations (create new tensor)
let new_tensor := tensor + 1.0      // Creates new tensor

// Memory layout optimization
let contiguous := tensor.contiguous[] // Ensure contiguous memory layout
```

### Lazy Evaluation

```cortex
// Defer computation until needed
let large_matrix := randn[10000, 10000]
let another_large := randn[10000, 10000]
let lazy_result := lazy[large_matrix @ another_large]

// Force computation
let result := force[lazy_result]

// Chain lazy operations
let chain := lazy[
  large_matrix @ another_large @ yet_another_large
]
```

### Device Management

```cortex
// Move tensors between devices
let cpu_tensor := randn[1000, 1000]
let gpu_tensor := cpu_tensor.to[device: "cuda"]

// Device-aware operations
let result := cpu_tensor + gpu_tensor.to[device: "cpu"]

// Multi-device operations
let distributed := distribute[tensor, devices: ["cuda:0", "cuda:1"]]
```

## Best Practices

### 1. Use Appropriate Data Types

```cortex
let float_tensor := tensor[1.0, 2.0, 3.0, dtype: "float32"]  // Faster, less memory
let double_tensor := tensor[1.0, 2.0, 3.0, dtype: "float64"] // Higher precision
```

### 2. Leverage Broadcasting

```cortex
let matrix := randn[100, 784]
let bias := randn[784]
let result := matrix + bias  // Broadcasting is more efficient than loops
```

### 3. Use In-place Operations When Possible

```cortex
tensor += 1.0  // In-place (efficient)
// vs
tensor = tensor + 1.0  // Creates new tensor (less efficient)
```

### 4. Profile Tensor Operations

```cortex
let start_time := time[]
let result := expensive_computation[data]
let end_time := time[]
print["Computation took: " + str[end_time - start_time] + " seconds"]
```

### 5. Use Lazy Evaluation for Large Computations

```cortex
let lazy_result := lazy[expensive_operation[large_tensor]]
// Computation is deferred until explicitly needed
let final_result := force[lazy_result]
```

## Future Enhancements

- **Quantum Tensors**: Support for quantum computing tensor operations
- **Distributed Tensors**: Multi-node tensor operations
- **Graph Tensors**: Support for graph neural network operations
- **Sparse Operations**: Advanced sparse tensor operations and algorithms
- **Custom Kernels**: User-defined tensor operations with CUDA/OpenCL
- **Automatic Optimization**: Compiler-driven tensor operation optimization

---

**Tensor primitives form the foundation of Cortex's AI/ML capabilities, providing high-performance, memory-efficient, and developer-friendly numerical computing primitives designed specifically for machine learning workflows.**
