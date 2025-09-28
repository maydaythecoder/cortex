# Cortex Development Guide

This guide covers development practices, architecture decisions, and contribution guidelines for the Cortex programming language project.

## Project Structure

``` txt
cortex/
├── python/              # Python implementation (LLVM backend)
│   ├── compiler/        # Core compiler modules
│   ├── cortexc.py       # CLI interface
│   ├── test_compiler.py # Test suite
│   └── venv/            # Virtual environment
├── rust/                # Rust implementation (interpreter)
│   ├── src/             # Source code
│   ├── Cargo.toml       # Dependencies
│   └── target/          # Build artifacts
├── docs/                # Documentation
│   ├── shared/          # Shared documentation
│   ├── python/          # Python-specific docs
│   └── rust/            # Rust-specific docs
└── .gitignore           # Git ignore rules
```

## Architecture Principles

### 1. Dual Implementation Strategy

- **Python**: LLVM backend for production performance
- **Rust**: Interpreter for development and safety
- **Shared**: Common language specification and examples

### 2. Language Design Goals

- **AI/ML Focus**: Built for machine learning workflows
- **Readable Syntax**: Bracket-based, clear and concise
- **Performance**: Compiled performance with interpreted development
- **Safety**: Memory safety and type safety

### 3. Implementation Philosophy

- **Test-Driven**: Comprehensive test coverage
- **Documentation-First**: Clear documentation for all features
- **Performance-Conscious**: Optimize for both development and runtime
- **Cross-Platform**: Support multiple operating systems

## Development Workflow

### 1. Setting Up Development Environment

#### Python Implementation

```bash
cd python
python3 -m venv venv
source venv/bin/activate
pip install -r ../requirements.txt
```

#### Rust Implementation

```bash
cd rust
cargo build
```

### 2. Running Tests

#### Python Tests

```bash
cd python
python test_compiler.py
```

#### Rust Tests

```bash
cd rust
cargo test
```

### 3. Adding New Features

#### Step 1: Language Specification

1. Update `docs/shared/syntax.md` with new syntax
2. Add examples to `docs/shared/examples/`
3. Update `docs/shared/COMPARISON.md` if needed

#### Step 2: Python Implementation

1. **Lexer**: Add tokens in `compiler/parser/lexer.py`
2. **Parser**: Add parsing rules in `compiler/parser/parser.py`
3. **AST**: Define nodes in `compiler/ast/nodes.py`
4. **Codegen**: Implement LLVM IR in `compiler/codegen.py`
5. **Tests**: Add test cases in `test_compiler.py`

#### Step 3: Rust Implementation

1. **Lexer**: Add tokens in `src/lexer.rs`
2. **Parser**: Add parsing methods in `src/parser.rs`
3. **AST**: Define types in `src/ast.rs`
4. **Interpreter**: Implement logic in `src/codegen.rs`
5. **Tests**: Add test cases with `cargo test`

#### Step 4: Documentation

1. Update implementation-specific READMEs
2. Add examples and usage guides
3. Update comparison documentation

### 4. Code Quality Standards

#### Python

- Follow PEP 8 style guidelines
- Use type hints where possible
- Add docstrings for all public functions
- Maintain test coverage above 80%

#### Rust

- Follow Rust style guidelines (`cargo fmt`)
- Use `cargo clippy` for linting
- Add documentation comments for public items
- Use `Result` types for error handling

## Testing Strategy

### 1. Unit Tests

- **Lexer**: Test tokenization of various inputs
- **Parser**: Test AST generation for language constructs
- **Codegen**: Test LLVM IR generation (Python) or interpretation (Rust)
- **Integration**: Test complete compilation/execution pipeline

### 2. Example-Based Testing

- All examples in `docs/shared/examples/` must pass
- Add new examples for new features
- Test both implementations with same examples

### 3. Performance Testing

- Benchmark critical operations
- Monitor memory usage
- Track compilation time

### 4. Cross-Platform Testing

- Test on macOS, Linux, and Windows
- Verify ARM64 and x86_64 support
- Test with different LLVM versions

## Error Handling

### 1. Error Types

- **Lexical Errors**: Invalid tokens, unterminated strings
- **Syntax Errors**: Malformed expressions, missing delimiters
- **Semantic Errors**: Undefined variables, type mismatches
- **Runtime Errors**: Division by zero, out of bounds access

### 2. Error Reporting

- **Location**: Line and column information
- **Context**: Surrounding code for clarity
- **Suggestions**: Helpful hints for fixing errors
- **Recovery**: Continue parsing when possible

### 3. Implementation-Specific

- **Python**: Exception-based with LLVM error integration
- **Rust**: Result types with detailed error context

## Performance Considerations

### 1. Compilation Performance

- **Lexer**: Efficient tokenization algorithms
- **Parser**: Recursive descent with lookahead
- **Codegen**: Optimized LLVM IR generation
- **Optimization**: LLVM optimization passes

### 2. Runtime Performance

- **Python**: LLVM optimizations, efficient memory management
- **Rust**: Zero-cost abstractions, ownership system
- **Benchmarking**: Regular performance testing
- **Profiling**: Identify and fix bottlenecks

### 3. Memory Management

- **Python**: Python GC with LLVM memory management
- **Rust**: Ownership system, no GC overhead
- **Leak Detection**: Regular memory leak testing
- **Optimization**: Minimize allocations

## Documentation Standards

### 1. Code Documentation

- **Python**: Google-style docstrings
- **Rust**: Rust doc comments with examples
- **API**: Complete API documentation
- **Examples**: Working code examples

### 2. User Documentation

- **Getting Started**: Quick start guides
- **Language Reference**: Complete syntax reference
- **Examples**: Comprehensive example collection
- **Tutorials**: Step-by-step learning guides

### 3. Developer Documentation

- **Architecture**: System design and decisions
- **Contributing**: How to contribute guidelines
- **Testing**: Testing strategies and practices
- **Performance**: Performance considerations

## Release Process

### 1. Version Management

- **Semantic Versioning**: Major.Minor.Patch
- **Changelog**: Detailed change documentation
- **Compatibility**: Backward compatibility considerations
- **Migration**: Upgrade guides for breaking changes

### 2. Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Examples verified
- [ ] Performance benchmarks
- [ ] Cross-platform testing
- [ ] Security review

### 3. Distribution

- **Python**: PyPI package (future)
- **Rust**: Crates.io package (future)
- **Binaries**: GitHub releases
- **Docker**: Container images (future)

## Contributing Guidelines

### 1. Code Contributions

- Fork the repository
- Create a feature branch
- Follow coding standards
- Add comprehensive tests
- Update documentation
- Submit a pull request

### 2. Documentation Contributions

- Fix typos and improve clarity
- Add missing examples
- Improve existing documentation
- Translate documentation (future)

### 3. Issue Reporting

- Use GitHub issues
- Provide reproduction steps
- Include system information
- Label issues appropriately

### 4. Feature Requests

- Discuss in GitHub discussions
- Provide use cases and examples
- Consider implementation complexity
- Get community feedback

## Security Considerations

### 1. Code Security

- **Input Validation**: Sanitize all inputs
- **Memory Safety**: Prevent buffer overflows
- **Dependency Management**: Keep dependencies updated
- **Code Review**: Security-focused code reviews

### 2. Runtime Security

- **Sandboxing**: Isolate execution environment
- **Resource Limits**: Prevent resource exhaustion
- **Access Control**: Limit system access
- **Audit Logging**: Track security events

## Future Development

### 1. Short Term (3-6 months)

- Complete function parameter handling
- Improve string operations
- Add array data structures
- Enhanced error messages

### 2. Medium Term (6-12 months)

- Advanced type system
- Standard library
- Package system
- Performance optimizations

### 3. Long Term (1-2 years)

- GPU acceleration
- WebAssembly support
- IDE integration
- Community ecosystem

## Getting Help

### 1. Documentation

- Check existing documentation first
- Look for examples and tutorials
- Review implementation-specific guides

### 2. Community

- GitHub discussions for questions
- GitHub issues for bugs
- Pull requests for contributions

### 3. Development

- Join the development team
- Contribute to the project
- Help others learn and use Cortex

## Conclusion

The Cortex project follows modern software development practices with a focus on performance, safety, and usability. By maintaining dual implementations and comprehensive documentation, we ensure both development velocity and production readiness.

Contributors are encouraged to follow these guidelines and help improve the project for the entire community.

## Related Documentation

- [Function Status](FUNCTION_STATUS.md)
- [Python Implementation Guide](../python/README.md)
- [Rust Implementation Guide](../rust/README.md)
- [Language Comparison](COMPARISON.md)
