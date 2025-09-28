# Cortex Development TODO

This document contains actionable tasks for Cortex development, organized by priority and timeline.

## Short-term Tasks (Immediate - Next 2 weeks)

### Core Interpreter Improvements

- [ ] **Fix parser error handling**
  - Add line/column information to all error messages
  - Implement error recovery for common syntax errors
  - Add helpful suggestions for typos and common mistakes
  - **Files**: `compiler/parser/parser.py`, `compiler/parser/lexer.py`

- [ ] **Enhance REPL functionality**
  - Add multi-line input support
  - Implement command history (up/down arrows)
  - Add syntax highlighting
  - Support variable inspection (`?variable`)
  - **Files**: `cortexc.py` (repl_command function)

- [ ] **Add comprehensive test suite**
  - Create `tests/` directory structure
  - Add unit tests for lexer, parser, and interpreter
  - Add integration tests for example programs
  - Set up continuous integration (GitHub Actions)
  - **Files**: Create `tests/` directory, add `test_*.py` files

- [ ] **Improve error messages**
  - Add context to runtime errors (variable names, function calls)
  - Implement stack traces for function calls
  - Add type mismatch error messages
  - **Files**: `compiler/interpreter.py`

### Basic ML Primitives

- [ ] **Implement tensor data type**
  - Add `Tensor` class to AST nodes
  - Implement basic tensor operations (+, -, *, /)
  - Add tensor literals: `[1, 2, 3]` → `Tensor([1, 2, 3])`
  - **Files**: `compiler/ast/nodes.py`, `compiler/interpreter.py`

- [ ] **Add basic mathematical functions**
  - Implement `sin`, `cos`, `exp`, `log`, `sqrt`
  - Add `sum`, `mean`, `std` for tensors
  - Implement `dot` product for vectors
  - **Files**: `compiler/interpreter.py` (builtin functions)

- [ ] **Create simple linear regression example**
  - Implement gradient descent manually
  - Show tensor operations in action
  - **Files**: `examples/linear_regression.ctx`

### Documentation and Examples

- [ ] **Update README with current status**
  - Fix installation instructions
  - Update feature list with what's actually implemented
  - Add troubleshooting section
  - **Files**: `README.MD`

- [ ] **Create getting started tutorial**
  - Step-by-step guide for first Cortex program
  - Explain syntax and basic concepts
  - **Files**: `docs/getting_started.md`

- [ ] **Add more example programs**
  - Matrix multiplication
  - Simple neural network (without autodiff)
  - Data processing pipeline
  - **Files**: `examples/` directory

## Medium-term Tasks (Next 1-2 months)

### Type System Foundation

- [ ] **Design type system**
  - Define type hierarchy (Number, String, Tensor, Function)
  - Plan type inference algorithm
  - Design type annotations syntax
  - **Files**: `compiler/types/` (new directory)

- [ ] **Implement basic type checker**
  - Add type checking pass to compiler
  - Implement type inference for simple expressions
  - Add type error reporting
  - **Files**: `compiler/type_checker.py`

- [ ] **Add type annotations to AST**
  - Extend existing type annotation support
  - Add type information to function definitions
  - **Files**: `compiler/ast/nodes.py`

### Advanced Language Features

- [ ] **Implement proper for loops**
  - Support `for [i in range[10]]` syntax
  - Add iterator protocol
  - **Files**: `compiler/parser/parser.py`, `compiler/interpreter.py`

- [ ] **Add array indexing**
  - Support `arr[0]`, `arr[1:3]` syntax
  - Implement bounds checking
  - **Files**: `compiler/ast/nodes.py`, `compiler/interpreter.py`

- [ ] **Implement function overloading**
  - Support multiple function definitions with same name
  - Dispatch based on argument types
  - **Files**: `compiler/interpreter.py`

### Performance and Optimization

- [ ] **Add performance benchmarks**
  - Create benchmark suite for common operations
  - Compare with Python/NumPy performance
  - **Files**: `benchmarks/` directory

- [ ] **Optimize interpreter performance**
  - Profile hot paths in interpreter
  - Implement caching for function calls
  - **Files**: `compiler/interpreter.py`

- [ ] **Add memory management**
  - Implement reference counting
  - Add garbage collection for circular references
  - **Files**: `compiler/runtime/` (new directory)

## Long-term Milestones (3-6 months)

### Phase 1: Foundation Complete

- [ ] **Comprehensive test coverage (>90%)**
  - Unit tests for all components
  - Integration tests for example programs
  - Performance regression tests
  - **Files**: `tests/` directory

- [ ] **Basic tensor operations**
  - Element-wise operations
  - Matrix multiplication
  - Broadcasting
  - **Files**: `compiler/tensor/` (new directory)

- [ ] **Standard library foundation**
  - Math functions
  - String operations
  - I/O utilities
  - **Files**: `stdlib/` directory

### Phase 2: ML Primitives

- [ ] **Automatic differentiation system**
  - Design autodiff architecture
  - Implement forward-mode autodiff
  - Add reverse-mode autodiff (backpropagation)
  - **Files**: `compiler/autodiff/` (new directory)

- [ ] **Neural network primitives**
  - Layer definitions (Dense, Conv2D, etc.)
  - Activation functions
  - Loss functions
  - **Files**: `stdlib/nn/` directory

- [ ] **Training utilities**
  - Optimizers (SGD, Adam, etc.)
  - Learning rate schedulers
  - Callbacks and monitoring
  - **Files**: `stdlib/optim/` directory

- [ ] **Data processing pipeline**
  - Dataset loading and preprocessing
  - Data augmentation
  - Batch processing
  - **Files**: `stdlib/data/` directory

### Phase 3: Compilation Backend

- [ ] **LLVM IR generation**
  - Design IR generation architecture
  - Implement basic IR generation for simple programs
  - **Files**: `compiler/llvm/` (new directory)

- [ ] **Type system implementation**
  - Complete type checker
  - Type inference algorithm
  - Type safety guarantees
  - **Files**: `compiler/types/` directory

- [ ] **Optimization passes**
  - Dead code elimination
  - Constant folding
  - Loop optimization
  - **Files**: `compiler/optimization/` (new directory)

- [ ] **GPU compilation support**
  - CUDA kernel generation
  - OpenCL support
  - Memory management for GPU
  - **Files**: `compiler/gpu/` (new directory)

### Phase 4: Ecosystem and Tooling

- [ ] **Package manager**
  - Design package format
  - Implement dependency resolution
  - **Files**: `tools/cortex-pm/` (new directory)

- [ ] **IDE support**
  - Language Server Protocol implementation
  - Syntax highlighting
  - Code completion
  - **Files**: `tools/lsp/` (new directory)

- [ ] **Debugging tools**
  - Breakpoint support
  - Variable inspection
  - Call stack visualization
  - **Files**: `tools/debugger/` (new directory)

- [ ] **Python interoperability**
  - C API for Python integration
  - NumPy/PyTorch interop
  - **Files**: `compiler/interop/` (new directory)

## Development Workflow

### Daily Tasks

1. **Morning**: Review and update TODO list
2. **Development**: Work on highest priority short-term task
3. **Evening**: Write tests for new features
4. **Before commit**: Run full test suite

### Weekly Tasks

1. **Monday**: Plan week's priorities
2. **Wednesday**: Mid-week progress review
3. **Friday**: Update documentation and examples
4. **Weekend**: Work on medium-term tasks

### Monthly Tasks

1. **Month start**: Review roadmap and adjust priorities
2. **Mid-month**: Performance benchmarking
3. **Month end**: Release planning and documentation updates

## Quality Gates

### Before merging any PR

- [ ] All tests pass
- [ ] Code coverage >90%
- [ ] Performance benchmarks show no regression
- [ ] Documentation updated
- [ ] Examples updated if needed

### Before each release

- [ ] Full test suite passes
- [ ] Performance benchmarks meet targets
- [ ] Documentation is complete and accurate
- [ ] Examples are working and demonstrate features
- [ ] Changelog is updated

## Resources and References

### Compiler Design

- [Crafting Interpreters](https://craftinginterpreters.com/) - Excellent resource for language implementation
- [LLVM Tutorial](https://llvm.org/docs/tutorial/) - Official LLVM documentation
- [Modern Compiler Implementation in ML](https://www.cs.princeton.edu/~appel/modern/ml/) - Advanced compiler techniques

### ML Systems

- [PyTorch Internals](https://pytorch.org/tutorials/advanced/cpp_frontend.html) - Understanding ML framework design
- [Automatic Differentiation](https://en.wikipedia.org/wiki/Automatic_differentiation) - AD theory and implementation
- [GPU Computing](https://developer.nvidia.com/cuda-zone) - CUDA programming guide

### Language Design

- [Rust Language Design](https://doc.rust-lang.org/book/) - Modern language features
- [Julia Language](https://julialang.org/) - High-performance scientific computing
- [Swift Language](https://swift.org/) - Modern language with ML focus

## Getting Started

1. **Pick a short-term task** from the list above
2. **Create a feature branch**: `git checkout -b feature/task-name`
3. **Implement the feature** with tests
4. **Run the test suite**: `python -m pytest tests/`
5. **Update documentation** if needed
6. **Submit a pull request**

## Contributing Guidelines

- **Code Style**: Follow PEP 8 for Python code
- **Testing**: Write tests for all new features
- **Documentation**: Update docs for user-facing changes
- **Performance**: Benchmark performance-critical code
- **Security**: Follow security best practices for language implementation

---

**Last Updated**: [9/27/25]
**Next Review**: [10/11/25]
