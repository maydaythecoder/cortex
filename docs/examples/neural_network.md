# Neural Network Example

This example demonstrates how to implement a multi-layer perceptron for classification in Cortex.

## Overview

The neural network example shows:

- **Network Architecture**: Multi-layer perceptron with configurable layers
- **Forward Pass**: Layer-by-layer computation with activation functions
- **Training Loop**: Batch processing with loss computation and parameter updates
- **Data Handling**: Synthetic data generation and one-hot encoding
- **Evaluation**: Training and validation metrics

## Key Features

### Network Structure

```cortex
struct NeuralNetwork |
  layers: list[Layer]
  optimizer: string
^

struct Layer |
  weights: tensor
  bias: tensor
  activation: string
^
```

### Activation Functions

- **ReLU**: `relu[output]`
- **Sigmoid**: `sigmoid[output]`
- **Softmax**: `softmax[output]`
- **Tanh**: `tanh[output]`

### Training Components

- **Cross-entropy loss**: `cross_entropy_loss[predictions, targets]`
- **One-hot encoding**: `one_hot[labels, num_classes]`
- **Gradient computation**: `∇[loss, model.parameters]`
- **Batch processing**: Configurable batch size

## Architecture

``` txt
Input (784) → Hidden (128) → Output (10)
     ↓              ↓            ↓
   Linear        ReLU        Softmax
```

## Usage

```bash
python3 cortexc.py run examples/neural_network.ctx
```

## Expected Output

``` txt
=== Neural Network in Cortex ===
Created neural network with 2 layers
Starting neural network training...
Training samples: 1000
Validation samples: 200
Epoch 0: Train Loss = 2.302, Val Loss = 2.301, Train Acc = 0.100, Val Acc = 0.100
Epoch 5: Train Loss = 1.845, Val Loss = 1.892, Train Acc = 0.350, Val Acc = 0.320
...
Training completed!
Test accuracy: 0.85
```

## Key Functions

### `create_neural_network[input_size, hidden_size, output_size]`

Creates a multi-layer perceptron with the specified architecture.

### `forward_pass[model, input]`

Performs forward propagation through all layers.

### `train_neural_network[model, train_data, val_data]`

Trains the network with validation monitoring.

### `generate_classification_data[n_samples, n_features, n_classes]`

Generates synthetic classification data for testing.

## Implementation Notes

- Uses simplified gradient computation (in practice, implement autograd)
- Synthetic data generation for demonstration
- Basic training loop with early stopping potential
- Configurable hyperparameters via constants

## Future Enhancements

- Implement proper autograd system
- Add more optimization algorithms (Adam, RMSprop)
- Support for different network architectures
- Real dataset loading capabilities
- GPU acceleration support
