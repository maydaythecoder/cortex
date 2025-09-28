# Data Structures

Cortex provides AI/ML-optimized data structures that seamlessly integrate with tensor operations and machine learning workflows.

## Core Data Structures

### Arrays and Tensors

```cortex
// 1D arrays (vectors)
let vector := [1, 2, 3, 4, 5]
let zeros := zeros[10]           // [0, 0, 0, ...]
let ones := ones[5]              // [1, 1, 1, 1, 1]
let range := arange[0, 10, 2]    // [0, 2, 4, 6, 8]

// 2D arrays (matrices)
let matrix := [[1, 2, 3], [4, 5, 6]]
let identity := eye[3]           // 3x3 identity matrix
let random := randn[2, 3]        // 2x3 random matrix

// Higher-dimensional tensors
let tensor3d := tensor[[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]]
```

### Dictionaries (Datasets)

```cortex
// Basic dictionary
let config := {
  "learning_rate": 0.01,
  "batch_size": 32,
  "epochs": 100
}

// Dataset structure
let dataset := {
  "features": [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
  "labels": [0, 1, 0],
  "metadata": {
    "size": 3,
    "feature_dim": 3,
    "num_classes": 2
  }
}

// Access patterns
let lr := config["learning_rate"]
let features := dataset["features"]
let size := dataset["metadata"]["size"]
```

### Objects (Models)

```cortex
// Model definition
let model := {
  "layers": [
    {"type": "dense", "input_size": 784, "output_size": 128},
    {"type": "relu"},
    {"type": "dense", "input_size": 128, "output_size": 10}
  ],
  "weights": [],
  "biases": [],
  "optimizer": "adam"
}

// Access and modification
let first_layer := model["layers"][0]
model["weights"] := initialize_weights[model]
```

## AI/ML Specific Structures

### Neural Network Layers

```cortex
// Layer definition
let dense_layer := {
  "type": "dense",
  "input_size": 784,
  "output_size": 128,
  "activation": "relu",
  "weights": randn[784, 128],
  "bias": zeros[128]
}

let conv_layer := {
  "type": "conv2d",
  "input_channels": 3,
  "output_channels": 32,
  "kernel_size": [3, 3],
  "stride": [1, 1],
  "padding": "same"
}
```

### Optimizer States

```cortex
let adam_optimizer := {
  "type": "adam",
  "learning_rate": 0.001,
  "beta1": 0.9,
  "beta2": 0.999,
  "epsilon": 1e-8,
  "m": {},  // First moment estimates
  "v": {}   // Second moment estimates
}
```

### Training History

```cortex
let history := {
  "loss": [],
  "accuracy": [],
  "val_loss": [],
  "val_accuracy": [],
  "epochs": 0,
  "best_epoch": 0,
  "best_loss": float["inf"]
}
```

## Advanced Data Structures

### Sparse Tensors

```cortex
// Sparse representation for large, mostly-zero tensors
let sparse_tensor := {
  "indices": [[0, 0], [1, 2], [2, 1]],
  "values": [1.0, 2.0, 3.0],
  "shape": [3, 3],
  "dense_size": 9
}
```

### Batched Data

```cortex
let batch := {
  "inputs": randn[32, 784],      // 32 samples, 784 features
  "targets": randint[32, 0, 10], // 32 labels, 0-9 classes
  "batch_id": 0,
  "size": 32
}
```

### Model Checkpoints

```cortex
let checkpoint := {
  "epoch": 50,
  "model_state": {
    "weights": model["weights"],
    "biases": model["biases"]
  },
  "optimizer_state": optimizer["state"],
  "loss": 0.123,
  "accuracy": 0.95,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Data Structure Operations

### Tensor Operations

```cortex
let a := [1, 2, 3]
let b := [4, 5, 6]

// Element-wise operations
let sum := a + b              // [5, 7, 9]
let product := a * b          // [4, 10, 18]
let scaled := a * 2           // [2, 4, 6]

// Matrix operations
let matrix_a := [[1, 2], [3, 4]]
let matrix_b := [[5, 6], [7, 8]]
let matmul := matrix_a @ matrix_b  // Matrix multiplication
let transpose := matrix_a.T        // Transpose
```

### Dictionary Operations

```cortex
let config := {"lr": 0.01, "batch": 32}

// Check existence
if ["lr" in config] |
  print["Learning rate found"]
^

// Get with default
let epochs := config.get["epochs", 100]

// Update multiple keys
config.update[{"epochs": 200, "patience": 10}]

// Merge dictionaries
let new_config := config.merge[{"optimizer": "adam"}]
```

### Object Methods

```cortex
let model := create_model[]

// Method chaining
let result := model
  .add_layer[dense[784, 128]]
  .add_activation[relu[]]
  .add_layer[dense[128, 10]]
  .compile[optimizer="adam", loss="categorical_crossentropy"]
```

## Memory-Efficient Structures

### Lazy Evaluation

```cortex
// Lazy tensor - computation deferred until needed
let lazy_tensor := lazy[
  large_matrix @ another_large_matrix
]

// Force evaluation
let result := force[lazy_tensor]
```

### Streaming Data

```cortex
let data_stream := {
  "generator": lambda[] -> next_batch[],
  "buffer_size": 1000,
  "current_batch": 0,
  "total_batches": 1000
}

// Process stream
for [batch in data_stream] |
  let loss := train_batch[model, batch]
  print["Batch loss: " + str[loss]]
^
```

## Type-Safe Structures

### Generic Containers

```cortex
// Type-annotated containers
let int_list: list[int] := [1, 2, 3, 4, 5]
let float_matrix: matrix[float] := [[1.0, 2.0], [3.0, 4.0]]
let string_dict: dict[string, int] := {"a": 1, "b": 2}
```

### Union Types

```cortex
// Can hold either type
let flexible_data: int | float | string := 42
flexible_data := 3.14
flexible_data := "hello"
```

## Best Practices

1. **Use appropriate data structures** for your use case:

   ```cortex
   // For sequential data
   let sequence := [1, 2, 3, 4, 5]
   
   // For key-value pairs
   let config := {"key": "value"}
   
   // For structured data
   let model := {layers: [], weights: []}
   ```

2. **Leverage tensor operations** for numerical computing:

   ```cortex
   // Good: vectorized
   let result := activation[weights @ inputs + bias]
   
   // Avoid: manual loops
   // for [i in range[0, len[inputs]]] | ... ^
   ```

3. **Use sparse structures** for large, sparse data:

   ```cortex
   let sparse_matrix := sparse[indices, values, shape]
   ```

4. **Implement proper error handling**:

   ```cortex
   let value := config.get["missing_key", default_value]
   ```

5. **Use type annotations** for complex structures:

   ```cortex
   let model: neural_network := create_model[]
   let dataset: dataset := load_data[]
   ```
