# Python Implementation Error Reference

This document provides specific error information for the Python-based Cortex compiler implementation using LLVM.

## Table of Contents

1. [Python-Specific Errors](#python-specific-errors)
2. [LLVM Integration Errors](#llvm-integration-errors)
3. [Code Generation Issues](#code-generation-issues)
4. [Python Environment Issues](#python-environment-issues)
5. [Troubleshooting Python Errors](#troubleshooting-python-errors)

## Python-Specific Errors

### Import and Module Errors

**Error:** `Import "llvmlite.ir" could not be resolved`

- **Cause:** `llvmlite` package not installed or not in Python path
- **Solution:**

  ```bash
  pip install llvmlite
  # or
  source venv/bin/activate
  pip install -r requirements.txt
  ```

**Error:** `ModuleNotFoundError: No module named 'compiler'`

- **Cause:** Python path not set correctly
- **Solution:** Run from the correct directory or set PYTHONPATH

### LLVM Integration Errors

**Error:** `'NoneType' object has no attribute 'gep'`

- **Cause:** LLVM builder not initialized when accessing global variables
- **Location:** `codegen.py` - `_get_string_constant` method
- **Solution:** Check if `self.builder` is None before using LLVM operations

**Error:** `Operands must be the same type, got ([6 x i8]*, i8*)`

- **Cause:** Type mismatch in LLVM IR generation for string operations
- **Location:** `codegen.py` - `_generate_binary_op` method
- **Solution:** Ensure type consistency in string concatenation

**Error:** `Undefined variable: variable_name`

- **Cause:** Variable not found in local or global scope
- **Location:** `codegen.py` - `_generate_identifier` method
- **Solution:** Check variable declaration and scope

### Code Generation Issues

**Error:** `Global variable 'variable_name' not initialized`

- **Cause:** Global variable declared but not assigned a value
- **Location:** `codegen.py` - `_generate_assignment` method
- **Solution:** Initialize global variables before use

**Error:** `Function 'function_name' not declared`

- **Cause:** Function called before declaration
- **Location:** `codegen.py` - `_generate_function_call` method
- **Solution:** Declare functions before calling them

**Error:** `Invalid LLVM type for expression`

- **Cause:** Unsupported type in LLVM IR generation
- **Location:** `codegen.py` - type mapping
- **Solution:** Add support for the missing type

### String and Type Handling

**Error:** `String concatenation type mismatch`

- **Cause:** Attempting to concatenate incompatible types
- **Location:** `codegen.py` - `_generate_binary_op` method
- **Solution:** Implement proper type conversion for string operations

**Error:** `Cannot convert number to string`

- **Cause:** Missing number-to-string conversion in LLVM IR
- **Location:** `codegen.py` - `_generate_str_call` method
- **Solution:** Implement proper number-to-string conversion

 LLVM Integration Errors

#### LLVM IR Generation

**Error:** `LLVM IR generation failed`

- **Cause:** Invalid LLVM IR structure or operations
- **Solution:** Check LLVM IR syntax and operation validity

**Error:** `LLVM module verification failed`

- **Cause:** Generated LLVM IR doesn't pass validation
- **Solution:** Use LLVM's verification tools to identify issues

**Error:** `LLVM optimization failed`

- **Cause:** LLVM optimization passes encounter errors
- **Solution:** Disable problematic optimizations or fix IR issues

### LLVM Execution

**Error:** `LLVM execution engine failed`

- **Cause:** Issues with LLVM execution engine setup
- **Solution:** Check LLVM installation and configuration

**Error:** `LLVM JIT compilation failed`

- **Cause:** Just-in-time compilation issues
- **Solution:** Verify LLVM JIT support and configuration

## Code Generation Patterns

### Function Handling

**Error:** `Function parameter count mismatch`

- **Cause:** Function called with wrong number of arguments
- **Location:** `codegen.py` - function call generation
- **Solution:** Check function signature and call arguments

**Error:** `Function return type mismatch`

- **Cause:** Function returns different type than expected
- **Location:** `codegen.py` - return statement handling
- **Solution:** Ensure consistent return types

### Variable Scope Management

**Error:** `Variable scope violation`

- **Cause:** Variable accessed outside its scope
- **Location:** `codegen.py` - variable lookup
- **Solution:** Check variable declaration scope

**Error:** `Global variable access error`

- **Cause:** Incorrect global variable access in functions
- **Location:** `codegen.py` - global variable handling
- **Solution:** Implement proper global variable access

### Control Flow

**Error:** `Invalid control flow structure`

- **Cause:** Malformed if/while/for statements
- **Location:** `codegen.py` - control flow generation
- **Solution:** Check control flow syntax and structure

**Error:** `Loop condition type error`

- **Cause:** Loop condition doesn't evaluate to boolean
- **Location:** `codegen.py` - loop generation
- **Solution:** Ensure loop conditions are boolean expressions

## Python Environment Issues

### Virtual Environment

**Error:** `Virtual environment not activated`

- **Cause:** Running without activating the virtual environment
- **Solution:**

  ```bash
  source venv/bin/activate
  ```

**Error:** `Package not found in virtual environment`

- **Cause:** Required packages not installed in venv
- **Solution:**

  ```bash
  pip install -r requirements.txt
  ```

### Python Version

**Error:** `Python version incompatible`

- **Cause:** Using unsupported Python version
- **Solution:** Use Python 3.8+ for LLVM compatibility

**Error:** `LLVM version mismatch`

- **Cause:** LLVM version incompatible with llvmlite
- **Solution:** Check LLVM and llvmlite version compatibility

## Troubleshooting Python Errors

### Debug Mode

Enable debug output:
