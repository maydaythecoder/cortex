# Linear Regression Example

This example demonstrates basic machine learning concepts using linear regression in Cortex.

## Overview

The linear regression example shows:

- **Model Definition**: Simple linear function `y = wx + b`
- **Loss Function**: Mean Squared Error (MSE)
- **Training Loop**: Gradient descent optimization
- **Data Generation**: Synthetic dataset creation
- **Convergence**: Early stopping based on loss threshold

## Key Features

### Model Function

```cortex
func linear_model[X: tensor, w: float, b: float] -> tensor |
  return[X * w + b]
^
```

### Loss Function

```cortex
func mse_loss[predictions: tensor, targets: tensor] -> float |
  let diff := predictions - targets
  return[mean[diff ** 2]]
^
```

### Training Process

- **Forward pass**: Compute predictions
- **Loss computation**: Calculate MSE
- **Gradient computation**: `∇[loss, w]` and `∇[loss, b]`
- **Parameter updates**: Gradient descent step
- **Convergence check**: Early stopping

## Architecture

``` txt
Input X → Linear Model → Predictions
   ↓           ↓            ↓
  [1,2,3]   y = wx + b   [pred1, pred2, pred3]
              ↓
            Loss (MSE)
              ↓
         Gradient Update
```

## Usage

```bash
python3 cortexc.py run examples/linear_regression.ctx
```

## Expected Output

``` txt
=== Linear Regression in Cortex ===
Generated 100 training samples
Starting training...
Initial parameters: w=0.1, b=0.0
Epoch 0: Loss = 8.45, w = 0.15, b = 0.02
Epoch 100: Loss = 2.34, w = 1.89, b = 0.67
Epoch 200: Loss = 0.45, w = 2.23, b = 0.89
...
Converged at epoch 450
Training completed!
Final parameters: w=2.48, b=1.02
Predictions on test data:
X=[1.0] -> y=3.50
X=[2.0] -> y=5.98
X=[3.0] -> y=8.46
```

## Key Functions

### `generate_data[n_samples]`

Creates synthetic training data with known parameters for testing.

### `train_linear_regression[X, y]`

Implements the complete training loop with gradient descent.

### `linear_model[X, w, b]`

Computes linear predictions using current parameters.

### `mse_loss[predictions, targets]`

Calculates mean squared error between predictions and targets.

## Hyperparameters

- **Learning Rate**: 0.01
- **Max Epochs**: 1000
- **Convergence Threshold**: 0.001
- **True Parameters**: w=2.5, b=1.0 (for synthetic data)

## Implementation Notes

- Uses automatic differentiation for gradient computation
- Implements early stopping for efficiency
- Generates synthetic data with known ground truth
- Shows parameter convergence over training

## Mathematical Foundation

The linear regression model minimizes:

``` txt
L(w,b) = (1/n) * Σ(y_pred - y_true)²
```

Where:

- `y_pred = wx + b`
- Gradients: `∂L/∂w = (2/n) * Σ(y_pred - y_true) * x`
- Gradients: `∂L/∂b = (2/n) * Σ(y_pred - y_true)`

## Future Enhancements

- Add regularization (L1/L2)
- Support for multiple features
- Different optimization algorithms
- Real dataset loading
- Model evaluation metrics
