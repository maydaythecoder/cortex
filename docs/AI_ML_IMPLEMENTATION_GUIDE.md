# AI/ML Primitives Implementation Guide

This guide provides a comprehensive roadmap for implementing Cortex's AI/ML primitives, from tensor operations to neural network architectures, with detailed technical specifications and implementation phases.

## Implementation Roadmap

### Phase 1: Core Tensor Primitives (Months 1-3)

#### 1.1 Enhanced Value System

**Goal**: Extend the runtime value system to support tensor primitives

**Implementation**:

```rust
// packages/compiler/src/codegen.rs
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    data: Vec<f64>,           // Contiguous memory storage
    shape: Vec<usize>,        // Dimensions [batch, height, width, channels]
    strides: Vec<usize>,      // Memory layout optimization
    dtype: DataType,          // float32, float64, int32, int64
    device: Device,           // CPU, GPU, TPU
    requires_grad: bool,      // Automatic differentiation flag
    grad_fn: Option<Box<dyn GradientFunction>>, // Gradient computation graph
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Float32,
    Float64,
    Int32,
    Int64,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Device {
    Cpu,
    Cuda { device_id: usize },
    OpenCL { device_id: usize },
    Tpu { device_id: usize },
}
```

**Tasks**:

- [ ] Implement `Tensor` struct with memory management
- [ ] Add `SparseTensor` for memory-efficient operations
- [ ] Implement `LazyTensor` for deferred computation
- [ ] Add device abstraction layer
- [ ] Implement tensor display formatting

#### 1.2 Tensor AST Nodes

**Goal**: Extend the AST to support tensor expressions

**Implementation**:

```rust
// packages/compiler/src/ast.rs
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct TensorLiteral {
    elements: Vec<Expression>,        // Nested tensor elements
    shape: Option<Vec<usize>>,        // Explicit shape specification
    dtype: Option<DataType>,          // Data type specification
    device: Option<Device>,           // Device specification
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorOperation {
    operation: TensorOpType,          // @, ∇, ∂, +, -, *, /
    left: Box<Expression>,            // Left operand
    right: Box<Expression>,           // Right operand (optional for unary ops)
    in_place: bool,                   // In-place operation flag
}

#[derive(Debug, Clone, PartialEq)]
pub enum TensorOpType {
    // Linear algebra
    MatrixMultiply,                   // @ operator
    DotProduct,                       // Vector dot product
    OuterProduct,                     // ⊗ operator
    
    // Automatic differentiation
    Gradient,                         // ∇ operator
    PartialDerivative,                // ∂ operator
    Jacobian,                         // Jacobian matrix
    Hessian,                          // Hessian matrix
    
    // Element-wise operations
    Add, Subtract, Multiply, Divide,  // +, -, *, /
    Power,                            // ** operator
    
    // Reduction operations
    Sum, Mean, Max, Min,              // Aggregation functions
    Norm,                             // Vector/matrix norms
    
    // Tensor operations
    Reshape, Transpose,               // Shape manipulation
    Concat, Split,                    // Tensor composition
    Broadcast,                        // Broadcasting
}
```

**Tasks**:

- [ ] Implement `TensorLiteral` parsing and interpretation
- [ ] Add `TensorOperation` AST nodes
- [ ] Implement tensor slicing and broadcasting
- [ ] Add parser support for tensor operations

#### 1.3 Basic Tensor Operations

**Goal**: Implement core tensor operations

**Implementation**:

```rust
// packages/compiler/src/tensor_ops.rs
pub mod tensor_ops {
    use super::*;
    
    pub fn tensor_matmul(left: &Tensor, right: &Tensor) -> Result<Tensor> {
        // Matrix multiplication implementation
        // Support for broadcasting and different data types
    }
    
    pub fn tensor_add(left: &Tensor, right: &Tensor) -> Result<Tensor> {
        // Element-wise addition with broadcasting
    }
    
    pub fn tensor_multiply(left: &Tensor, right: &Tensor) -> Result<Tensor> {
        // Element-wise multiplication
    }
    
    pub fn tensor_reshape(tensor: &Tensor, new_shape: &[usize]) -> Result<Tensor> {
        // Reshape tensor with validation
    }
    
    pub fn tensor_transpose(tensor: &Tensor, axes: Option<&[usize]>) -> Result<Tensor> {
        // Transpose tensor
    }
    
    // Broadcasting utilities
    pub fn broadcast_shapes(shape_a: &[usize], shape_b: &[usize]) -> Result<Vec<usize>> {
        // Compute broadcasted shape
    }
    
    pub fn broadcast_tensors(tensors: &[&Tensor]) -> Result<Vec<Tensor>> {
        // Broadcast multiple tensors to compatible shapes
    }
}
```

**Tasks**:

- [ ] Implement basic arithmetic operations
- [ ] Add matrix multiplication with BLAS integration
- [ ] Implement broadcasting logic
- [ ] Add tensor reshaping and transposition
- [ ] Implement reduction operations (sum, mean, max, min)

### Phase 2: Neural Network Primitives (Months 4-6)

#### 2.1 Neuron Primitive

**Goal**: Implement modular neuron primitive

**Implementation**:

```rust
// packages/compiler/src/neural_network.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Neuron {
    weights: Tensor,                  // Weight tensor
    bias: Tensor,                     // Bias tensor
    activation: ActivationFunction,   // Activation type
    custom_params: HashMap<String, Value>, // Domain-specific parameters
    domain: String,                   // Application domain
    version: String,                  // Version tracking
    author: String,                   // Creator identification
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivationFunction {
    ReLU,
    Sigmoid,
    Tanh,
    GELU,
    Swish,
    Softmax,
    Custom(String),                   // Custom activation function
}

impl Neuron {
    pub fn new(
        input_size: usize,
        output_size: usize,
        activation: ActivationFunction,
        domain: String,
    ) -> Self {
        Self {
            weights: Tensor::randn(vec![input_size, output_size]),
            bias: Tensor::zeros(vec![output_size]),
            activation,
            custom_params: HashMap::new(),
            domain,
            version: "v1.0.0".to_string(),
            author: "system".to_string(),
        }
    }
    
    pub fn forward(&self, inputs: &Tensor) -> Result<Tensor> {
        // Forward pass: inputs @ weights + bias
        let linear_output = tensor_matmul(inputs, &self.weights)?;
        let biased_output = tensor_add(&linear_output, &self.bias)?;
        
        // Apply activation function
        self.apply_activation(&biased_output)
    }
    
    fn apply_activation(&self, input: &Tensor) -> Result<Tensor> {
        match &self.activation {
            ActivationFunction::ReLU => relu(input),
            ActivationFunction::Sigmoid => sigmoid(input),
            ActivationFunction::Tanh => tanh(input),
            ActivationFunction::GELU => gelu(input),
            ActivationFunction::Swish => swish(input),
            ActivationFunction::Softmax => softmax(input),
            ActivationFunction::Custom(name) => custom_activation(input, name),
        }
    }
}
```

**Tasks**:

- [ ] Implement `Neuron` struct with forward pass
- [ ] Add activation function implementations
- [ ] Implement domain-specific parameter handling
- [ ] Add neuron serialization/deserialization
- [ ] Create neuron creation utilities

#### 2.2 Layer Primitive

**Goal**: Implement composable layer primitive

**Implementation**:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    neurons: Vec<Neuron>,            // Collection of neurons
    learning_algorithm: LearningAlgorithm, // Layer-specific optimization
    name: String,                    // Human-readable identifier
    domain: String,                  // Application domain
    capabilities: Vec<String>,       // Layer capabilities
    version: String,                 // Semantic versioning
    dependencies: Vec<String>,       // Required prerequisite layers
    api_version: String,            // Cortex API compatibility
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearningAlgorithm {
    optimizer: OptimizerType,        // SGD, Adam, RMSprop, etc.
    learning_rate: f64,             // Base learning rate
    scheduler: Option<SchedulerType>, // Learning rate scheduling
    regularization: RegularizationConfig, // L1, L2, dropout
    early_stopping: EarlyStoppingConfig, // Early stopping config
    domain_config: HashMap<String, Value>, // Domain-specific params
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizerType {
    SGD { momentum: f64 },           // Stochastic Gradient Descent
    Adam { beta1: f64, beta2: f64 }, // Adam optimizer
    RMSprop { alpha: f64 },          // RMSprop optimizer
    AdaGrad,                         // AdaGrad optimizer
    Custom(String),                  // Custom optimizer
}

impl Layer {
    pub fn new(
        neurons: Vec<Neuron>,
        learning_algorithm: LearningAlgorithm,
        name: String,
        domain: String,
    ) -> Self {
        Self {
            neurons,
            learning_algorithm,
            name,
            domain,
            capabilities: vec![],
            version: "v1.0.0".to_string(),
            dependencies: vec![],
            api_version: "cortex.v1.0".to_string(),
        }
    }
    
    pub fn forward(&self, inputs: &Tensor) -> Result<Tensor> {
        // Forward pass through all neurons
        let mut outputs = vec![];
        for neuron in &self.neurons {
            let neuron_output = neuron.forward(inputs)?;
            outputs.push(neuron_output);
        }
        
        // Combine neuron outputs (concatenation, averaging, etc.)
        self.combine_outputs(&outputs)
    }
    
    fn combine_outputs(&self, outputs: &[Tensor]) -> Result<Tensor> {
        // Default: concatenate outputs
        tensor_concat(outputs, axis: 0)
    }
}
```

**Tasks**:

- [ ] Implement `Layer` struct with neuron management
- [ ] Add learning algorithm implementations
- [ ] Implement layer forward pass
- [ ] Add layer serialization/deserialization
- [ ] Create layer composition utilities

#### 2.3 Neural Network Primitive

**Goal**: Implement neural network assembly and management

**Implementation**:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,              // Ordered sequence of layers
    global_optimizer: OptimizerType, // Network-wide coordination
    meta_learning: MetaLearningConfig, // Cross-domain learning
    architecture_type: ArchitectureType, // Sequential, attention, etc.
    input_specs: HashMap<String, Value>, // Domain-specific input formats
    validation_required: bool,       // Pre-deployment validation
    compliance_level: String,        // Regulatory compliance
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArchitectureType {
    Sequential,                      // Simple feedforward
    Attention,                       // Attention mechanisms
    Residual,                        // Residual connections
    Graph,                          // Graph neural networks
    Transformer,                    // Transformer architecture
    Custom(String),                 // Custom architecture
}

impl NeuralNetwork {
    pub fn new(
        layers: Vec<Layer>,
        global_optimizer: OptimizerType,
        architecture_type: ArchitectureType,
    ) -> Self {
        Self {
            layers,
            global_optimizer,
            meta_learning: MetaLearningConfig::default(),
            architecture_type,
            input_specs: HashMap::new(),
            validation_required: false,
            compliance_level: "general".to_string(),
        }
    }
    
    pub fn forward(&self, inputs: &Tensor) -> Result<Tensor> {
        let mut current_input = inputs.clone();
        
        for layer in &self.layers {
            current_input = layer.forward(&current_input)?;
        }
        
        Ok(current_input)
    }
    
    pub fn backward(&self, loss: &Tensor) -> Result<Vec<Tensor>> {
        // Backward pass through all layers
        let mut gradients = vec![];
        let mut current_grad = loss.clone();
        
        for layer in self.layers.iter().rev() {
            let layer_grads = layer.backward(&current_grad)?;
            gradients.extend(layer_grads);
            current_grad = layer_grads[0].clone(); // Propagate gradient
        }
        
        gradients.reverse(); // Restore forward order
        Ok(gradients)
    }
}
```

**Tasks**:

- [ ] Implement `NeuralNetwork` struct
- [ ] Add forward and backward pass implementations
- [ ] Implement different architecture types
- [ ] Add network serialization/deserialization
- [ ] Create network composition utilities

### Phase 3: Automatic Differentiation (Months 7-9)

#### 3.1 Computational Graph

**Goal**: Implement computational graph for automatic differentiation

**Implementation**:

```rust
// packages/compiler/src/autodiff.rs
#[derive(Debug, Clone, PartialEq)]
pub struct ComputationalGraph {
    nodes: Vec<GraphNode>,           // Computation nodes
    edges: Vec<GraphEdge>,           // Data flow edges
    gradients: HashMap<NodeId, Tensor>, // Computed gradients
    requires_grad: HashSet<NodeId>,  // Nodes requiring gradients
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    id: NodeId,                      // Unique identifier
    operation: OperationType,        // Operation performed
    inputs: Vec<NodeId>,            // Input node IDs
    output: Tensor,                 // Output tensor
    grad_fn: Option<Box<dyn GradientFunction>>, // Gradient function
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    // Arithmetic operations
    Add, Subtract, Multiply, Divide,
    Power, Negate,
    
    // Tensor operations
    MatrixMultiply, Reshape, Transpose,
    Sum, Mean, Max, Min,
    
    // Activation functions
    ReLU, Sigmoid, Tanh, Softmax,
    
    // Loss functions
    MSE, CrossEntropy, BinaryCrossEntropy,
    
    // Custom operations
    Custom(String),
}

impl ComputationalGraph {
    pub fn forward(&mut self, inputs: &HashMap<NodeId, Tensor>) -> Result<HashMap<NodeId, Tensor>> {
        let mut outputs = HashMap::new();
        
        // Topological sort of nodes
        let sorted_nodes = self.topological_sort()?;
        
        for node_id in sorted_nodes {
            let node = &self.nodes[node_id.0];
            let node_inputs: Vec<&Tensor> = node.inputs.iter()
                .map(|input_id| outputs.get(input_id).unwrap())
                .collect();
            
            let output = self.execute_operation(&node.operation, &node_inputs)?;
            outputs.insert(node_id, output);
        }
        
        Ok(outputs)
    }
    
    pub fn backward(&mut self, final_grad: &Tensor) -> Result<HashMap<NodeId, Tensor>> {
        let mut gradients = HashMap::new();
        gradients.insert(self.nodes.len() - 1, final_grad.clone());
        
        // Reverse topological sort for backward pass
        let sorted_nodes = self.topological_sort()?;
        sorted_nodes.reverse();
        
        for node_id in sorted_nodes {
            if let Some(node) = self.nodes.get(node_id.0) {
                if let Some(grad_fn) = &node.grad_fn {
                    let input_grads = grad_fn.backward(
                        &gradients[&node_id],
                        &self.get_input_tensors(node_id)?
                    )?;
                    
                    for (i, input_id) in node.inputs.iter().enumerate() {
                        gradients.insert(*input_id, input_grads[i].clone());
                    }
                }
            }
        }
        
        Ok(gradients)
    }
}
```

**Tasks**:

- [ ] Implement computational graph data structure
- [ ] Add forward pass execution
- [ ] Implement backward pass with gradient computation
- [ ] Add gradient function implementations
- [ ] Create graph optimization utilities

#### 3.2 Gradient Functions

**Goal**: Implement gradient functions for all operations

**Implementation**:

```rust
pub trait GradientFunction {
    fn forward(&self, inputs: &[Tensor]) -> Result<Tensor>;
    fn backward(&self, grad_output: &Tensor, inputs: &[Tensor]) -> Result<Vec<Tensor>>;
}

// Matrix multiplication gradient
pub struct MatMulGradient {
    input_a: Tensor,
    input_b: Tensor,
}

impl GradientFunction for MatMulGradient {
    fn forward(&self, inputs: &[Tensor]) -> Result<Tensor> {
        tensor_matmul(&inputs[0], &inputs[1])
    }
    
    fn backward(&self, grad_output: &Tensor, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        // ∂(A @ B)/∂A = grad_output @ B.T
        // ∂(A @ B)/∂B = A.T @ grad_output
        let grad_a = tensor_matmul(grad_output, &tensor_transpose(&inputs[1], None)?)?;
        let grad_b = tensor_matmul(&tensor_transpose(&inputs[0], None)?, grad_output)?;
        Ok(vec![grad_a, grad_b])
    }
}

// ReLU gradient
pub struct ReLUGradient {
    input: Tensor,
}

impl GradientFunction for ReLUGradient {
    fn forward(&self, inputs: &[Tensor]) -> Result<Tensor> {
        relu(&inputs[0])
    }
    
    fn backward(&self, grad_output: &Tensor, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        // ∂ReLU(x)/∂x = 1 if x > 0, else 0
        let mask = tensor_gt(&inputs[0], &Tensor::zeros(inputs[0].shape.clone()))?;
        let grad_input = tensor_multiply(grad_output, &mask)?;
        Ok(vec![grad_input])
    }
}

// Mean Squared Error gradient
pub struct MSEGradient {
    predictions: Tensor,
    targets: Tensor,
}

impl GradientFunction for MSEGradient {
    fn forward(&self, inputs: &[Tensor]) -> Result<Tensor> {
        let diff = tensor_subtract(&inputs[0], &inputs[1])?;
        let squared_diff = tensor_multiply(&diff, &diff)?;
        tensor_mean(&squared_diff)
    }
    
    fn backward(&self, grad_output: &Tensor, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        let batch_size = inputs[0].shape[0] as f64;
        let diff = tensor_subtract(&inputs[0], &inputs[1])?;
        let grad_predictions = tensor_multiply(&diff, &Tensor::scalar(2.0 / batch_size))?;
        let grad_targets = tensor_multiply(&diff, &Tensor::scalar(-2.0 / batch_size))?;
        Ok(vec![grad_predictions, grad_targets])
    }
}
```

**Tasks**:

- [ ] Implement gradient functions for all operations
- [ ] Add activation function gradients
- [ ] Implement loss function gradients
- [ ] Add custom gradient function support
- [ ] Create gradient testing utilities

#### 3.3 Gradient Operators

**Goal**: Implement ∇ and ∂ operators in the interpreter

**Implementation**:

```rust
// packages/compiler/src/codegen.rs
impl Interpreter {
    fn interpret_tensor_operation(&mut self, op: &TensorOperation) -> Result<Value> {
        match op.operation {
            TensorOpType::Gradient => {
                // ∇[loss, params] - compute gradient of loss w.r.t. parameters
                let loss = self.interpret_expression(&op.left)?;
                let params = self.interpret_expression(&op.right)?;
                
                if let (Value::Tensor(loss_tensor), Value::Tensor(params_tensor)) = (&loss, &params) {
                    let gradient = self.compute_gradient(loss_tensor, params_tensor)?;
                    Ok(Value::Tensor(gradient))
                } else {
                    Err(anyhow::anyhow!("Gradient operator requires tensor arguments"))
                }
            }
            TensorOpType::PartialDerivative => {
                // ∂[f, x] - compute partial derivative of f w.r.t. x
                let function = self.interpret_expression(&op.left)?;
                let variable = self.interpret_expression(&op.right)?;
                
                if let (Value::Tensor(func_tensor), Value::Tensor(var_tensor)) = (&function, &variable) {
                    let partial = self.compute_partial_derivative(func_tensor, var_tensor)?;
                    Ok(Value::Tensor(partial))
                } else {
                    Err(anyhow::anyhow!("Partial derivative operator requires tensor arguments"))
                }
            }
            TensorOpType::MatrixMultiply => {
                // A @ B - matrix multiplication
                let left = self.interpret_expression(&op.left)?;
                let right = self.interpret_expression(&op.right)?;
                
                if let (Value::Tensor(left_tensor), Value::Tensor(right_tensor)) = (&left, &right) {
                    let result = tensor_matmul(left_tensor, right_tensor)?;
                    Ok(Value::Tensor(result))
                } else {
                    Err(anyhow::anyhow!("Matrix multiplication requires tensor arguments"))
                }
            }
            // ... other operations
        }
    }
    
    fn compute_gradient(&self, loss: &Tensor, params: &Tensor) -> Result<Tensor> {
        // Create computational graph
        let mut graph = ComputationalGraph::new();
        
        // Add nodes for loss and parameters
        let loss_node = graph.add_node(OperationType::Custom("loss".to_string()), vec![], loss.clone())?;
        let params_node = graph.add_node(OperationType::Custom("params".to_string()), vec![], params.clone())?;
        
        // Build gradient computation graph
        let gradient_node = graph.add_gradient_node(loss_node, params_node)?;
        
        // Execute backward pass
        let gradients = graph.backward(&Tensor::ones(loss.shape.clone()))?;
        
        Ok(gradients[&params_node].clone())
    }
}
```

**Tasks**:

- [ ] Implement ∇ operator in interpreter
- [ ] Add ∂ operator for partial derivatives
- [ ] Implement matrix multiplication (@) operator
- [ ] Add gradient computation utilities
- [ ] Create automatic differentiation examples

### Phase 4: Optimization Framework (Months 10-12)

#### 4.1 Optimizers

**Goal**: Implement SGD, Adam, RMSprop optimizers

**Implementation**:

```rust
// packages/compiler/src/optimizers.rs
pub trait Optimizer {
    fn step(&mut self, gradients: &[Tensor], parameters: &mut [Tensor]) -> Result<()>;
    fn get_state(&self) -> HashMap<String, Value>;
    fn set_state(&mut self, state: HashMap<String, Value>) -> Result<()>;
}

pub struct SGD {
    learning_rate: f64,
    momentum: f64,
    velocity: HashMap<String, Tensor>,
}

impl Optimizer for SGD {
    fn step(&mut self, gradients: &[Tensor], parameters: &mut [Tensor]) -> Result<()> {
        for (i, (grad, param)) in gradients.iter().zip(parameters.iter_mut()).enumerate() {
            let param_key = format!("param_{}", i);
            
            // Update velocity with momentum
            let velocity = self.velocity.get(&param_key).cloned()
                .unwrap_or_else(|| Tensor::zeros(param.shape.clone()));
            let new_velocity = tensor_add(
                &tensor_multiply(&velocity, &Tensor::scalar(self.momentum))?,
                &tensor_multiply(grad, &Tensor::scalar(self.learning_rate))?
            )?;
            
            // Update parameter
            *param = tensor_subtract(param, &new_velocity)?;
            self.velocity.insert(param_key, new_velocity);
        }
        Ok(())
    }
}

pub struct Adam {
    learning_rate: f64,
    beta1: f64,
    beta2: f64,
    epsilon: f64,
    m: HashMap<String, Tensor>,      // First moment estimates
    v: HashMap<String, Tensor>,      // Second moment estimates
    t: usize,                        // Time step
}

impl Optimizer for Adam {
    fn step(&mut self, gradients: &[Tensor], parameters: &mut [Tensor]) -> Result<()> {
        self.t += 1;
        
        for (i, (grad, param)) in gradients.iter().zip(parameters.iter_mut()).enumerate() {
            let param_key = format!("param_{}", i);
            
            // Initialize moments if not present
            if !self.m.contains_key(&param_key) {
                self.m.insert(param_key.clone(), Tensor::zeros(param.shape.clone()));
                self.v.insert(param_key.clone(), Tensor::zeros(param.shape.clone()));
            }
            
            // Update biased first moment estimate
            let m = self.m.get(&param_key).unwrap();
            let new_m = tensor_add(
                &tensor_multiply(m, &Tensor::scalar(self.beta1))?,
                &tensor_multiply(grad, &Tensor::scalar(1.0 - self.beta1))?
            )?;
            
            // Update biased second raw moment estimate
            let v = self.v.get(&param_key).unwrap();
            let grad_squared = tensor_multiply(grad, grad)?;
            let new_v = tensor_add(
                &tensor_multiply(v, &Tensor::scalar(self.beta2))?,
                &tensor_multiply(&grad_squared, &Tensor::scalar(1.0 - self.beta2))?
            )?;
            
            // Compute bias-corrected first moment estimate
            let m_hat = tensor_divide(&new_m, &Tensor::scalar(1.0 - self.beta1.powi(self.t as i32)))?;
            
            // Compute bias-corrected second raw moment estimate
            let v_hat = tensor_divide(&new_v, &Tensor::scalar(1.0 - self.beta2.powi(self.t as i32)))?;
            
            // Update parameters
            let update = tensor_divide(
                &tensor_multiply(&m_hat, &Tensor::scalar(self.learning_rate))?,
                &tensor_add(&tensor_sqrt(&v_hat)?, &Tensor::scalar(self.epsilon))?
            )?;
            
            *param = tensor_subtract(param, &update)?;
            
            // Store updated moments
            self.m.insert(param_key.clone(), new_m);
            self.v.insert(param_key, new_v);
        }
        Ok(())
    }
}
```

**Tasks**:

- [ ] Implement SGD optimizer with momentum
- [ ] Add Adam optimizer with bias correction
- [ ] Implement RMSprop optimizer
- [ ] Add AdaGrad optimizer
- [ ] Create optimizer state management

#### 4.2 Learning Rate Schedulers

**Goal**: Implement learning rate scheduling

**Implementation**:

```rust
pub trait Scheduler {
    fn get_lr(&self, epoch: usize) -> f64;
    fn step(&mut self, epoch: usize, loss: Option<f64>) -> Result<()>;
}

pub struct ConstantScheduler {
    learning_rate: f64,
}

impl Scheduler for ConstantScheduler {
    fn get_lr(&self, _epoch: usize) -> f64 {
        self.learning_rate
    }
    
    fn step(&mut self, _epoch: usize, _loss: Option<f64>) -> Result<()> {
        Ok(())
    }
}

pub struct StepScheduler {
    initial_lr: f64,
    step_size: usize,
    gamma: f64,
}

impl Scheduler for StepScheduler {
    fn get_lr(&self, epoch: usize) -> f64 {
        self.initial_lr * self.gamma.powi((epoch / self.step_size) as i32)
    }
    
    fn step(&mut self, _epoch: usize, _loss: Option<f64>) -> Result<()> {
        Ok(())
    }
}

pub struct CosineAnnealingScheduler {
    initial_lr: f64,
    min_lr: f64,
    max_epochs: usize,
}

impl Scheduler for CosineAnnealingScheduler {
    fn get_lr(&self, epoch: usize) -> f64 {
        if epoch >= self.max_epochs {
            self.min_lr
        } else {
            self.min_lr + (self.initial_lr - self.min_lr) * 
                (1.0 + (std::f64::consts::PI * epoch as f64 / self.max_epochs as f64).cos()) / 2.0
        }
    }
    
    fn step(&mut self, _epoch: usize, _loss: Option<f64>) -> Result<()> {
        Ok(())
    }
}
```

**Tasks**:

- [ ] Implement constant learning rate scheduler
- [ ] Add step decay scheduler
- [ ] Implement cosine annealing scheduler
- [ ] Add linear warmup scheduler
- [ ] Create scheduler testing utilities

### Phase 5: Layer Registry (Months 13-15)

#### 5.1 Layer Registry System

**Goal**: Implement community-driven layer sharing

**Implementation**:

```rust
// packages/compiler/src/registry.rs
pub struct LayerRegistry {
    layers: HashMap<String, Layer>,
    metadata: HashMap<String, LayerMetadata>,
    dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct LayerMetadata {
    name: String,
    version: String,
    author: String,
    description: String,
    domain: String,
    capabilities: Vec<String>,
    dependencies: Vec<String>,
    license: String,
    documentation: String,
    validation_status: ValidationStatus,
}

#[derive(Debug, Clone)]
pub enum ValidationStatus {
    Pending,
    Validated,
    Failed { errors: Vec<String> },
}

impl LayerRegistry {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            metadata: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    pub fn register_layer(&mut self, layer: Layer, metadata: LayerMetadata) -> Result<()> {
        let layer_id = format!("{}:{}", metadata.domain, metadata.name);
        let version_id = format!("{}:{}", layer_id, metadata.version);
        
        // Validate layer
        let validation_result = self.validate_layer(&layer, &metadata)?;
        if !validation_result.passed {
            return Err(anyhow::anyhow!("Layer validation failed: {:?}", validation_result.errors));
        }
        
        // Check dependencies
        self.check_dependencies(&metadata.dependencies)?;
        
        // Register layer
        self.layers.insert(version_id.clone(), layer);
        self.metadata.insert(version_id, metadata);
        
        Ok(())
    }
    
    pub fn import_layer(&self, layer_id: &str, version: Option<&str>) -> Result<Layer> {
        let version_id = if let Some(ver) = version {
            format!("{}:{}", layer_id, ver)
        } else {
            // Find latest version
            self.find_latest_version(layer_id)?
        };
        
        if let Some(layer) = self.layers.get(&version_id) {
            Ok(layer.clone())
        } else {
            Err(anyhow::anyhow!("Layer not found: {}", version_id))
        }
    }
    
    fn validate_layer(&self, layer: &Layer, metadata: &LayerMetadata) -> Result<ValidationResult> {
        let mut errors = vec![];
        
        // Check layer structure
        if layer.neurons.is_empty() {
            errors.push("Layer must contain at least one neuron".to_string());
        }
        
        // Check domain compatibility
        if layer.domain != metadata.domain {
            errors.push("Layer domain mismatch".to_string());
        }
        
        // Check version format
        if !self.is_valid_version(&metadata.version) {
            errors.push("Invalid version format".to_string());
        }
        
        // Check capabilities
        for capability in &metadata.capabilities {
            if !self.is_valid_capability(capability) {
                errors.push(format!("Invalid capability: {}", capability));
            }
        }
        
        Ok(ValidationResult {
            passed: errors.is_empty(),
            errors,
        })
    }
}

#[derive(Debug)]
pub struct ValidationResult {
    passed: bool,
    errors: Vec<String>,
}
```

**Tasks**:

- [ ] Implement layer registry data structure
- [ ] Add layer validation system
- [ ] Implement dependency management
- [ ] Add layer import/export functionality
- [ ] Create registry API

#### 5.2 Layer Blending System

**Goal**: Implement automatic layer blending

**Implementation**:

```rust
pub struct LayerBlender {
    registry: LayerRegistry,
    blending_strategies: HashMap<String, Box<dyn BlendingStrategy>>,
}

pub trait BlendingStrategy {
    fn blend(&self, layer_a: &Layer, layer_b: &Layer, config: &BlendConfig) -> Result<Layer>;
    fn analyze_compatibility(&self, layer_a: &Layer, layer_b: &Layer) -> CompatibilityAnalysis;
}

pub struct GradualTransferStrategy;

impl BlendingStrategy for GradualTransferStrategy {
    fn blend(&self, layer_a: &Layer, layer_b: &Layer, config: &BlendConfig) -> Result<Layer> {
        let mut intermediate_layers = vec![];
        
        for (i, weight) in config.interpolation_weights.iter().enumerate() {
            let blended_neurons = self.interpolate_neurons(
                &layer_a.neurons,
                &layer_b.neurons,
                *weight
            )?;
            
            let intermediate_layer = Layer {
                neurons: blended_neurons,
                learning_algorithm: self.interpolate_learning_algorithms(
                    &layer_a.learning_algorithm,
                    &layer_b.learning_algorithm,
                    *weight
                )?,
                name: format!("Bridge_{}_{}", layer_a.name, i),
                domain: format!("{}_to_{}", layer_a.domain, layer_b.domain),
                capabilities: self.merge_capabilities(&layer_a.capabilities, &layer_b.capabilities),
                version: "v1.0.0".to_string(),
                dependencies: vec![layer_a.name.clone(), layer_b.name.clone()],
                api_version: "cortex.v1.0".to_string(),
            };
            
            intermediate_layers.push(intermediate_layer);
        }
        
        // Combine intermediate layers
        self.combine_layers(intermediate_layers)
    }
    
    fn analyze_compatibility(&self, layer_a: &Layer, layer_b: &Layer) -> CompatibilityAnalysis {
        CompatibilityAnalysis {
            domain_similarity: self.compute_domain_similarity(&layer_a.domain, &layer_b.domain),
            feature_alignment: self.compute_feature_alignment(&layer_a.neurons, &layer_b.neurons),
            capability_overlap: self.compute_capability_overlap(&layer_a.capabilities, &layer_b.capabilities),
            recommended_strategy: "gradual_transfer".to_string(),
        }
    }
}

impl LayerBlender {
    pub fn blend_layers(&self, layer_a: &Layer, layer_b: &Layer, config: Option<BlendConfig>) -> Result<Layer> {
        let config = config.unwrap_or_else(|| self.auto_detect_config(layer_a, layer_b));
        let strategy = self.select_strategy(&config.blend_type)?;
        
        strategy.blend(layer_a, layer_b, &config)
    }
    
    fn auto_detect_config(&self, layer_a: &Layer, layer_b: &Layer) -> BlendConfig {
        let analysis = self.analyze_compatibility(layer_a, layer_b);
        
        BlendConfig {
            blend_type: analysis.recommended_strategy,
            bridge_neurons: self.compute_optimal_bridge_size(layer_a, layer_b),
            transfer_weight: analysis.domain_similarity,
            adaptation_epochs: self.estimate_convergence_time(layer_a, layer_b),
            preserve_source_accuracy: true,
            enhance_target_effectiveness: 2.0,
        }
    }
}
```

**Tasks**:

- [ ] Implement layer blending strategies
- [ ] Add compatibility analysis
- [ ] Implement automatic configuration detection
- [ ] Add blending validation
- [ ] Create blending examples

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::new(vec![2, 3], DataType::Float32, Device::Cpu);
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.dtype, DataType::Float32);
    }
    
    #[test]
    fn test_matrix_multiplication() {
        let a = Tensor::randn(vec![2, 3]);
        let b = Tensor::randn(vec![3, 4]);
        let c = tensor_matmul(&a, &b).unwrap();
        assert_eq!(c.shape, vec![2, 4]);
    }
    
    #[test]
    fn test_gradient_computation() {
        let x = Tensor::new(vec![1], DataType::Float32, Device::Cpu).requires_grad(true);
        let y = tensor_multiply(&x, &x).unwrap(); // y = x²
        let grad = compute_gradient(&y, &x).unwrap();
        assert_eq!(grad.data[0], 2.0 * x.data[0]); // dy/dx = 2x
    }
}
```

### Integration Tests

```rust
#[test]
fn test_neural_network_training() {
    // Create simple neural network
    let mut network = NeuralNetwork::new(
        vec![
            Layer::dense(784, 128, ActivationFunction::ReLU),
            Layer::dense(128, 10, ActivationFunction::Softmax),
        ],
        OptimizerType::Adam { beta1: 0.9, beta2: 0.999 },
        ArchitectureType::Sequential,
    );
    
    // Generate synthetic data
    let inputs = Tensor::randn(vec![100, 784]);
    let targets = Tensor::randint(vec![100], 0, 10);
    
    // Training loop
    for epoch in 0..10 {
        let predictions = network.forward(&inputs).unwrap();
        let loss = cross_entropy_loss(&predictions, &targets).unwrap();
        
        let gradients = network.backward(&loss).unwrap();
        network.update_parameters(&gradients, 0.001).unwrap();
        
        println!("Epoch {}: Loss = {}", epoch, loss.data[0]);
    }
}
```

## Performance Benchmarks

### Tensor Operations

```rust
#[bench]
fn bench_matrix_multiplication(b: &mut Bencher) {
    let a = Tensor::randn(vec![1000, 1000]);
    let b = Tensor::randn(vec![1000, 1000]);
    
    b.iter(|| {
        tensor_matmul(&a, &b).unwrap()
    });
}

#[bench]
fn bench_gradient_computation(b: &mut Bencher) {
    let x = Tensor::randn(vec![1000]).requires_grad(true);
    let y = tensor_multiply(&x, &x).unwrap();
    
    b.iter(|| {
        compute_gradient(&y, &x).unwrap()
    });
}
```

## Documentation and Examples

### Tensor Operations Examples

```cortex
// Basic tensor operations
let a := tensor[[1, 2], [3, 4]]
let b := tensor[[5, 6], [7, 8]]
let c := a @ b  // Matrix multiplication

// Automatic differentiation
let x := tensor[2.0, requires_grad: true]
let y := x ** 2 + 3 * x + 1
let grad := ∇[y, x]  // dy/dx = 2x + 3
```

### Neural Network Examples

```cortex
// Create neural network
let network := compose_network[
  layers: [
    create_layer[domain: "medical", neurons: [medical_neuron]],
    create_layer[domain: "education", neurons: [educational_neuron]]
  ],
  global_optimizer: "adam"
]

// Training loop
for [epoch in range[0, 100]] {
  let predictions := network.forward[train_inputs]
  let loss := cross_entropy[predictions, train_targets]
  let gradients := ∇[loss, network.parameters]
  network := update_parameters[network, gradients, 0.001]
}
```

## Conclusion

This implementation guide provides a comprehensive roadmap for implementing Cortex's AI/ML primitives. The phased approach ensures systematic development while maintaining code quality and performance. Each phase builds upon the previous one, creating a solid foundation for a production-ready AI/ML programming language.

**Key Success Factors**:

- **Modular Design**: Each component is independently testable and maintainable
- **Performance Focus**: Optimized implementations for production use
- **Community Integration**: Registry system enables community-driven development
- **Documentation**: Comprehensive examples and documentation for adoption
- **Testing**: Rigorous testing strategy ensures reliability and correctness

**Expected Outcomes**:

- **10x+ Performance**: Tensor operations significantly faster than interpreter
- **Production Ready**: Robust, tested, and documented primitives
- **Community Adoption**: Active layer sharing and contributions
- **Ecosystem Growth**: Rich library of specialized domain layers
