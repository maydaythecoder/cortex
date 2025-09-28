# Training Workflows

Cortex provides comprehensive support for machine learning training workflows, from simple linear models to complex neural networks.

## Basic Training Loop

### Simple Linear Regression

```cortex
// Define model parameters
let w := randn[1]  // weight
let b := 0.0       // bias
let lr := 0.01     // learning rate

// Training data
let X := [[1.0], [2.0], [3.0], [4.0]]
let y := [2.0, 4.0, 6.0, 8.0]

// Training loop
for [epoch in range[0, 100]] |
  // Forward pass
  let predictions := X @ w + b
  
  // Compute loss
  let loss := mse[predictions, y]
  
  // Backward pass (gradients)
  let grad_w := ∇[loss, w]
  let grad_b := ∇[loss, b]
  
  // Update parameters
  w := w - lr * grad_w
  b := b - lr * grad_b
  
  if [epoch % 10 == 0] |
    print["Epoch " + str[epoch] + ": Loss = " + str[loss]]
  ^
^
```

### Neural Network Training

```cortex
// Define network architecture
let model := {
  "layers": [
    {"type": "dense", "input_size": 784, "output_size": 128, "activation": "relu"},
    {"type": "dense", "input_size": 128, "output_size": 64, "activation": "relu"},
    {"type": "dense", "input_size": 64, "output_size": 10, "activation": "softmax"}
  ],
  "weights": [],
  "biases": []
}

// Initialize parameters
model := initialize_model[model]

// Training configuration
let config := {
  "learning_rate": 0.001,
  "batch_size": 32,
  "epochs": 100,
  "optimizer": "adam"
}

// Training loop
for [epoch in range[0, config["epochs"]]] |
  let epoch_loss := 0.0
  let num_batches := 0
  
  // Process batches
  for [batch in data_loader[config["batch_size"]]] |
    // Forward pass
    let predictions := forward_pass[model, batch["inputs"]]
    
    // Compute loss
    let loss := cross_entropy[predictions, batch["targets"]]
    
    // Backward pass
    let gradients := backward_pass[model, loss]
    
    // Update parameters
    model := update_parameters[model, gradients, config["learning_rate"]]
    
    epoch_loss := epoch_loss + loss
    num_batches := num_batches + 1
  ^
  
  let avg_loss := epoch_loss / num_batches
  print["Epoch " + str[epoch] + ": Loss = " + str[avg_loss]]
^
```

## Advanced Training Patterns

### Early Stopping

```cortex
func train_with_early_stopping[model, train_data, val_data, config] |
  let best_loss := float["inf"]
  let patience := config["patience"]
  let no_improvement := 0
  
  for [epoch in range[0, config["max_epochs"]]] |
    // Train epoch
    let train_loss := train_epoch[model, train_data, config]
    
    // Validate
    let val_loss := validate[model, val_data]
    
    // Check for improvement
    if [val_loss < best_loss] |
      best_loss := val_loss
      no_improvement := 0
      save_checkpoint[model, epoch, val_loss]
    ^ else |
      no_improvement := no_improvement + 1
    ^
    
    // Early stopping
    if [no_improvement >= patience] |
      print["Early stopping at epoch " + str[epoch]]
      break
    ^
    
    print["Epoch " + str[epoch] + ": Train=" + str[train_loss] + ", Val=" + str[val_loss]]
  ^
  
  return[load_best_checkpoint[]]
^
```

### Learning Rate Scheduling

```cortex
func train_with_scheduler[model, data, config] |
  let scheduler := create_scheduler[config["scheduler_type"], config["initial_lr"]]
  
  for [epoch in range[0, config["epochs"]]] |
    let current_lr := scheduler.get_lr[epoch]
    
    // Train with current learning rate
    let loss := train_epoch[model, data, current_lr]
    
    // Update scheduler
    scheduler.step[loss]
    
    print["Epoch " + str[epoch] + ": LR=" + str[current_lr] + ", Loss=" + str[loss]]
  ^
^
```

### Gradient Clipping

```cortex
func train_with_gradient_clipping[model, data, config] |
  let max_grad_norm := config["max_grad_norm"]
  
  for [epoch in range[0, config["epochs"]]] |
    let loss := 0.0
    
    for [batch in data] |
      // Forward pass
      let predictions := model[batch["inputs"]]
      let batch_loss := compute_loss[predictions, batch["targets"]]
      
      // Backward pass
      let gradients := ∇[batch_loss, model.parameters]
      
      // Gradient clipping
      let grad_norm := norm[gradients]
      if [grad_norm > max_grad_norm] |
        gradients := gradients * (max_grad_norm / grad_norm)
      ^
      
      // Update parameters
      model := update_parameters[model, gradients, config["lr"]]
      loss := loss + batch_loss
    ^
    
    print["Epoch " + str[epoch] + ": Loss = " + str[loss]]
  ^
^
```

## Distributed Training

### Data Parallel Training

```cortex
func distributed_train[model, data, config] |
  let world_size := config["world_size"]
  let rank := config["rank"]
  
  // Initialize distributed training
  init_distributed[world_size, rank]
  
  // Create distributed data loader
  let dist_data_loader := create_distributed_loader[data, world_size, rank]
  
  for [epoch in range[0, config["epochs"]]] |
    let epoch_loss := 0.0
    
    for [batch in dist_data_loader] |
      // Forward pass
      let predictions := model[batch["inputs"]]
      let loss := compute_loss[predictions, batch["targets"]]
      
      // Backward pass
      let gradients := ∇[loss, model.parameters]
      
      // All-reduce gradients across processes
      all_reduce[gradients]
      
      // Update parameters
      model := update_parameters[model, gradients, config["lr"]]
      epoch_loss := epoch_loss + loss
    ^
    
    // Synchronize processes
    barrier[]
    
    if [rank == 0] |
      print["Epoch " + str[epoch] + ": Loss = " + str[epoch_loss]]
    ^
  ^
^
```

## Hyperparameter Optimization

### Grid Search

```cortex
func grid_search[model_factory, data, param_grid] |
  let best_score := 0.0
  let best_params := {}
  let results := []
  
  for [params in param_grid] |
    let model := model_factory[params]
    let score := cross_validate[model, data, params]
    
    results := append[results, {"params": params, "score": score}]
    
    if [score > best_score] |
      best_score := score
      best_params := params
    ^
    
    print["Params: " + str[params] + ", Score: " + str[score]]
  ^
  
  return[best_params, best_score, results]
^
```

### Random Search

```cortex
func random_search[model_factory, data, param_space, n_trials] |
  let best_score := 0.0
  let best_params := {}
  
  for [trial in range[0, n_trials]] |
    let params := sample_params[param_space]
    let model := model_factory[params]
    let score := cross_validate[model, data, params]
    
    if [score > best_score] |
      best_score := score
      best_params := params
    ^
    
    print["Trial " + str[trial] + ": Score = " + str[score]]
  ^
  
  return[best_params, best_score]
^
```

## Model Evaluation

### Cross-Validation

```cortex
func cross_validate[model_factory, data, params, k_folds] |
  let scores := []
  
  for [fold in range[0, k_folds]] |
    let train_data, val_data := split_fold[data, fold, k_folds]
    let model := model_factory[params]
    
    // Train model
    model := train[model, train_data, params]
    
    // Evaluate
    let score := evaluate[model, val_data]
    scores := append[scores, score]
  ^
  
  return[mean[scores], std[scores]]
^
```

### Model Metrics

```cortex
func evaluate_model[model, test_data] |
  let predictions := model[test_data["inputs"]]
  let targets := test_data["targets"]
  
  // Classification metrics
  let accuracy := accuracy_score[predictions, targets]
  let precision := precision_score[predictions, targets]
  let recall := recall_score[predictions, targets]
  let f1 := f1_score[predictions, targets]
  
  // Regression metrics
  let mse := mse_score[predictions, targets]
  let mae := mae_score[predictions, targets]
  let r2 := r2_score[predictions, targets]
  
  return[{
    "accuracy": accuracy,
    "precision": precision,
    "recall": recall,
    "f1": f1,
    "mse": mse,
    "mae": mae,
    "r2": r2
  }]
^
```

## Best Practices

1. **Use appropriate batch sizes**:

   ```cortex
   let batch_size := 32  // Good for most cases
   let batch_size := 1   // For online learning
   let batch_size := 256 // For large datasets
   ```

2. **Monitor training progress**:

   ```cortex
   let history := {
     "train_loss": [],
     "val_loss": [],
     "train_acc": [],
     "val_acc": []
   }
   ```

3. **Save checkpoints regularly**:

   ```cortex
   if [epoch % 10 == 0] |
     save_checkpoint[model, epoch, loss]
   ^
   ```

4. **Use validation sets**:

   ```cortex
   let train_data, val_data := split[data, ratio=0.8]
   ```

5. **Implement proper error handling**:

   ```cortex
   try |
     let loss := train_epoch[model, data]
   ^ catch [error] |
     print["Training error: " + str[error]]
     return[null]
   ^
   ```
