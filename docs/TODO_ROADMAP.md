# Cortex Language TODO Roadmap

Complete implementation roadmap organized by priority and complexity.

## **🔴 CRITICAL - Core Functionality (Blocks Examples)**

### **Code Generation TODOs**

**Files**: `rust/src/codegen.rs`
**Priority**: **BLOCKER** - Prevents examples from running

#### ✅ TODO: Array Support (Line 228-235)

```rust
Expression::Array(_) => {
    // TODO: Implement array support
    Err(anyhow::anyhow!("Array literals not yet implemented"))
}
```

**Status**: Parsed but not code generated
**Implementation**: Create array initialization in LLVM

#### ✅ TODO: Dictionary Support (Line 228-235)

```rust
Expression::Dictionary(_) => {
    // TODO: Implement dictionary support  
    Err(anyhow::anyhow!("Dictionary literals not yet implemented"))
}
```

**Status**: Parsed but not code generated
**Implementation**: Create hash map/struct in LLVM

### **Interpreter TODOs**

**Files**: `rust/src/codegen.rs`

#### ✅ TODO: For Loop Semantics (Line 119-127)

```rust
fn interpret_for_loop(&mut self, for_loop: &ForLoop) -> Result<()> {
    // For now, implement as a simple infinite loop with break
    // TODO: Implement proper for loop semantics
    loop {
        self.interpret_block(&for_loop.body)?;
        break; // Simple implementation - just run once
    }
    Ok(())
}
```

**Status**: Runs loop body once only
**Implementation**: Parse iteration expression, implement proper iteration

#### ✅ TODO: Return Value Handling (Line 129-134)

```rust
fn interpret_return_statement(&mut self, return_stmt: &ReturnStatement) -> Result<()> {
    if let Some(value) = &return_stmt.value {
        let _result = self.interpret_expression(value)?;
        // TODO: Handle return value properly
    }
    Ok(())
}
```

**Status**: Parses return but doesn't store/return value
**Implementation**: Store return values in function frame

---

## **🟠 HIGH PRIORITY - Essential Features**

### **Parser TODOs**

#### ✅ TODO: AI/ML Operators (@, ∇, ∂)

**Status**: Tokenized (lines 104-111, lexer.rs) but parser implementation incomplete
**Implementation**: Add parsing rules for:

```cortex
// Matrix multiplication
let result := matrix_a @ matrix_b

// Gradient operations  
let gradient := ∇[loss_function]

// Partial derivatives
let partial := ∂[f, x]
```

#### ✅ TODO: Array Indexing Syntax

**Status**: Not implemented
**Implementation**: Add parsing for `array[index]` syntax
**Parser**: Need to handle `Expression::Identifier` followed by `[Expression]`

#### ✅ TODO: Dictionary Access Syntax  

**Status**: Not implemented
**Implementation**: Add parsing for `dict["key"]` or `dict.key` syntax

### **Runtime TODOs**

#### ✅ TODO: Expression Statements

**Status**: Basic implementation present
**Issue**: Ensure standalone expressions work as statements
**Example**: `5 + 3` should execute and produce output

#### ✅ TODO: Variable Scoping

**Status**: Basic scoping, need shadowing and nested scope support
**Implementation**: Ensure proper variable shadowing behavior

---

## **🟡 MEDIUM PRIORITY - Core Libraries**

### **Built-in Functions**

#### ✅ TODO: Standard Library Implementation

**Functions Needed**:

```cortex
// Array functions
len[array]           // Length of array
append[array, item]  // Append to array
index[array, idx]    // Access array element

// Math functions  
sqrt[value]
sin[value], cos[value], tan[value]
exp[value], log[value]
min[a, b], max[a, b]

// Type conversion
str[value]           // Convert to string
int[value]           // Convert to integer
float[value]         // Convert to float

// Utility functions
print[value]         // Already implemented?
sqrt[value]
time[]               // Current timestamp
```

#### ✅ TODO: Type Conversion Functions

**Status**: No built-in conversion functions
**Implementation**: Add runtime type coercion helpers

### **Error Handling**

#### ✅ TODO: Enhanced Error Messages

**Current**: Basic parse errors
**Needed**: Runtime error messages, type mismatch errors, better debugging

---

## **🟢 LOW PRIORITY - Advanced Features**

### **AI/ML Runtime**

#### ✅ TODO: Tensor Operations

**Status**: Tokenized operators, no runtime
**Needs**: Tensor data structure and operations

#### ✅ TODO: Automatic Differentiation  

**Status**: ∇ operator tokenized
**Implementation**: Backpropagation system
**Complexity**: Very high

### **Performance**

#### ✅ TODO: Optimizations

- Dead code elimination
- Constant folding  
- Loop optimizations
- Memory management

#### ✅ TODO: Memory Management

- Array/dictionary garbage collection
- Reference counting for complex structures

---

## **🧪 PRIORITY - Testing & Quality**

### **Testing Suite**

#### ✅ TODO: Comprehensive Test Suite

**Current**: Basic parser tests only
**Needed**:

- Full integration tests for examples
- Type checking tests
- Error handling tests
- Performance tests

#### ✅ TODO: Example Validation

**Status**: Examples created but may not run
**Action**: Test all example files against current implementation

---

## **📝 IMPLEMENTATION ORDER**

### **Phase 1: Core Blockers (Week 1)**

1. ✅ Array code generation
2. ✅ Dictionary code generation  
3. ✅ Return value handling
4. ✅ For loop semantics

### **Phase 2: Essential Features (Week 2)**

1. ✅ Expression statements verification
2. ✅ Basic array indexing
3. ✅ Basic dictionary access
4. ✅ AI/ML operator parsing

### **Phase 3: Standard Library (Week 3)**

1. ✅ Built-in function library
2. ✅ Type conversion functions
3. ✅ Error handling improvements

### **Phase 4: Advanced Features (Week 4+)**

1. ✅ Tensor operations runtime
2. ✅ Performance optimizations
3. ✅ Comprehensive testing

---

## **🎯 SUCCESS CRITERIA**

### **Minimum Viable Language**

- [ ] All examples run successfully
- [ ] Basic arithmetic operations work
- [ ] Function calls work
- [ ] Control flow works
- [ ] Arrays and dictionaries compile and run

### **Production Ready**

- [ ] Type checking works correctly
- [ ] All built-in functions implemented
- [ ] Comprehensive test coverage
- [ ] Performance optimized
- [ ] Error messages helpful

### **AI/ML Ready**  

- [ ] Tensor operations work
- [ ] Gradient calculations implemented
- [ ] Matrix operations efficient
- [ ] Neural network examples run

---

## **🔧 TECHNICAL DEBT**

### **Code Quality Issues**

- TODO comments in production code
- Incomplete error handling
- Missing documentation for complex functions
- No performance benchmarking

### **Architecture Improvements**

- Separate interpreter from compiler
- Modular code generation
- Plugin system for built-in functions
- Better AST visitor pattern usage

---

This roadmap prioritizes completing the core language features that prevent the comprehensive examples from running, followed by adding the sophisticated AI/ML capabilities that make Cortex unique.
