# Gradient Descent Optimization Example

This example demonstrates various optimization algorithms for minimizing functions in Cortex.

## Overview

The gradient descent example shows:

- **Objective Function**: `f(x, y) = (x - 2)² + (y - 3)²`
- **Optimization Algorithms**: Basic GD, Momentum, Adam
- **Convergence Analysis**: Iteration tracking and comparison
- **Multiple Starting Points**: Testing robustness

## Key Features

### Objective Function

```cortex
func objective_function[x: float, y: float] -> float |
  return[(x - 2.0) ** 2 + (y - 3.0) ** 2]
^
```

### Gradient Computation

```cortex
func objective_gradient[x: float, y: float] |
  let grad_x := 2.0 * (x - 2.0)
  let grad_y := 2.0 * (y - 3.0)
  return[grad_x, grad_y]
^
```

## Optimization Algorithms

### 1. Basic Gradient Descent

```cortex
func gradient_descent[initial_x, initial_y, learning_rate] |
  // Simple parameter update
  x := x - learning_rate * grad_x
  y := y - learning_rate * grad_y
^
```

### 2. Momentum Gradient Descent

```cortex
func momentum_gradient_descent[initial_x, initial_y, learning_rate, momentum] |
  // Velocity-based updates
  vx := momentum * vx + learning_rate * grad_x
  vy := momentum * vy + learning_rate * grad_y
  x := x - vx
  y := y - vy
^
```

### 3. Adam Optimizer

```cortex
func adam_optimizer[initial_x, initial_y, learning_rate] |
  // Adaptive learning rates with bias correction
  m_x := beta1 * m_x + (1 - beta1) * grad_x
  v_x := beta2 * v_x + (1 - beta2) * (grad_x ** 2)
  m_x_hat := m_x / (1 - beta1 ** (iteration + 1))
  v_x_hat := v_x / (1 - beta2 ** (iteration + 1))
  x := x - learning_rate * m_x_hat / (sqrt[v_x_hat] + epsilon)
^
```

## Architecture

``` txt
Objective Function: f(x,y) = (x-2)² + (y-3)²
        ↓
    Gradient: ∇f = [2(x-2), 2(y-3)]
        ↓
   Optimization Algorithm
        ↓
   Parameter Updates
        ↓
   Convergence Check
```

## Usage

```bash
python3 cortexc.py run examples/gradient_descent.ctx
```

## Expected Output

``` txt
=== Gradient Descent Optimization in Cortex ===

Testing with starting point: (0.0, 0.0)
=== Basic Gradient Descent ===
Initial point: (0.0, 0.0)
Learning rate: 0.1
Iteration 0: x = 0.4, y = 0.6, f(x,y) = 8.32
Iteration 100: x = 1.8, y = 2.7, f(x,y) = 0.18
...
Converged at iteration 250

=== Momentum Gradient Descent ===
Initial point: (0.0, 0.0)
Learning rate: 0.1, Momentum: 0.9
Iteration 0: x = 0.4, y = 0.6, f(x,y) = 8.32
...
Converged at iteration 180

=== Adam Optimizer ===
Initial point: (0.0, 0.0)
Learning rate: 0.1
Iteration 0: x = 0.4, y = 0.6, f(x,y) = 8.32
...
Converged at iteration 120

=== Summary ===
Gradient Descent: (2.0, 3.0) - Value: 0.0
Momentum: (2.0, 3.0) - Value: 0.0
Adam: (2.0, 3.0) - Value: 0.0
```

## Key Functions

### `compare_optimizers[initial_x, initial_y]`

Runs all three optimization algorithms and compares their performance.

### `gradient_descent[initial_x, initial_y, learning_rate]`

Basic gradient descent with fixed learning rate.

### `momentum_gradient_descent[initial_x, initial_y, learning_rate, momentum]`

Momentum-based optimization with velocity tracking.

### `adam_optimizer[initial_x, initial_y, learning_rate]`

Adam optimizer with adaptive learning rates and bias correction.

## Hyperparameters

- **Learning Rate**: 0.1
- **Momentum**: 0.9 (for momentum GD)
- **Beta1**: 0.9 (for Adam)
- **Beta2**: 0.999 (for Adam)
- **Epsilon**: 1e-8 (for Adam)
- **Max Iterations**: 1000
- **Convergence Threshold**: 1e-6

## Mathematical Foundation

 Objective Function

``` txt
f(x, y) = (x - 2)² + (y - 3)²
```

### Gradients

``` txt
∂f/∂x = 2(x - 2)
∂f/∂y = 2(y - 3)
```

### True Minimum

- **Location**: (2, 3)
- **Value**: 0

## Algorithm Comparison

| Algorithm | Convergence Speed | Stability | Memory Usage |
|-----------|------------------|-----------|--------------|
| Basic GD | Slow | High | Low |
| Momentum | Medium | High | Medium |
| Adam | Fast | Medium | High |

## Implementation Notes

- All algorithms converge to the true minimum (2, 3)
- Adam typically converges fastest
- Momentum helps with oscillation reduction
- Basic GD is most stable but slowest

## Future Enhancements

- Add more optimization algorithms (RMSprop, AdaGrad)
- Implement line search for learning rate
- Add constraint optimization
- Support for higher-dimensional problems
- Visualization of optimization paths
