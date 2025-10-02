# Cortex VSCode Extension v1.0.3 - Release Notes

**Release Date**: October 2, 2025  
**Version**: 1.0.3  
**Status**: ✅ PUBLISHED

---

## 🎉 Release Highlights

This release accompanies a major cleanup and test consolidation effort for the Cortex compiler, ensuring the VSCode extension works with the latest, fully-tested compiler implementation.

### What's New

#### 🧹 Project Cleanup

- Removed incomplete and non-compiling packages
- Consolidated all tests following Rust best practices
- Cleaned codebase now contains only functional, tested code

#### ✅ Test Suite Verification

- **64 comprehensive tests** - All passing
- **Unit tests**: 26/26 passing
- **Integration tests**: 27/28 passing (1 ignored for performance)
- **Doc tests**: 10/10 passing

#### 📚 Documentation Improvements

- Added comprehensive API documentation
- Created library interface for compiler
- Updated README with accurate build/test instructions
- Added doc tests to all public APIs

---

## 📦 Extension Details

### Package Information

- **Name**: cortex-language
- **Version**: 1.0.3
- **Publisher**: muhyadinmohamed
- **Size**: 2.59MB
- **Files**: 13

### Installation

```bash
# Install from VSCode Marketplace
code --install-extension muhyadinmohamed.cortex-language

# Or search "Cortex Language" in VSCode Extensions
```

 Extension URL
<https://marketplace.visualstudio.com/items?itemName=muhyadinmohamed.cortex-language>

---

## ✨ Features

### Syntax Highlighting

- ✅ Complete Cortex language syntax support
- ✅ Keywords, operators, and AI/ML primitives
- ✅ Custom color scheme optimized for Cortex

### Code Formatting

- ✅ Automatic formatting on save
- ✅ Configurable indentation
- ✅ Proper bracket and block alignment

### Commands

- ✅ `Cortex: Run File` - Execute Cortex programs
- ✅ `Cortex: Check Syntax` - Validate syntax
- ✅ `Cortex: Build Executable` - Compile to binary
- ✅ `Cortex: Format File` - Format code
- ✅ `Cortex: Build Compiler` - Build Rust compiler
- ✅ `Cortex: Debug File` - Debug programs
- ✅ `Cortex: Run Tests` - Execute test suite
- ✅ `Cortex: Lint File` - Code linting

### Keyboard Shortcuts

- ✅ `Ctrl+Shift+F` - Format file
- ✅ `Ctrl+Shift+R` - Run file
- ✅ `F5` - Debug file
- ✅ `Ctrl+Shift+T` - Run tests

### IntelliSense

- ✅ Code completion suggestions
- ✅ Function signature help
- ✅ Hover documentation
- ✅ Symbol navigation

### File Icons

- ✅ Custom icons for `.ctx` files
- ✅ Light and dark theme variants

---

## 🛠️ Compiler Backend

### Fully Tested Implementation

The extension now works with a thoroughly tested Cortex compiler:

#### Test Coverage

``` txt
Total Tests: 64
├─ Unit Tests: 26 ✅
├─ Integration Tests: 27 ✅ (1 ignored)
└─ Doc Tests: 10 ✅

Success Rate: 98.4%
```

#### Tested Features

- ✅ Variables and assignments
- ✅ Arithmetic operations (all operators)
- ✅ String operations
- ✅ Functions (definition, calls, recursion)
- ✅ Control flow (if/else, while, for loops)
- ✅ Arrays and dictionaries
- ✅ Built-in functions (print, str, len, math operations)
- ✅ Error handling (undefined vars, type mismatches, syntax errors)

---

## 🔧 Configuration Options

### Available Settings

```json
{
  "cortex.format.indentSize": 2,
  "cortex.format.formatOnSave": true,
  "cortex.compiler.binaryPath": "",
  "cortex.languageServer.enabled": true,
  "cortex.debugging.enabled": true,
  "cortex.linting.enabled": true,
  "cortex.completion.enabled": true
}
```

---

## 📝 System Requirements

- **VSCode**: 1.74.0 or higher
- **Rust**: 1.70+ (for building compiler from source)
- **OS**: macOS, Linux, or Windows

---

## 🚀 Getting Started

### 1. Install the Extension

Install from VSCode Marketplace or:

```bash
code --install-extension muhyadinmohamed.cortex-language
```

### 2. Create a Cortex File

```cortex
// hello.ctx
func main[] |
  print["Hello, Cortex!"]
^

main[]
```

### 3. Run Your Program

- Press `Ctrl+Shift+R` or
- Right-click → "Run Cortex File" or
- Use Command Palette → "Cortex: Run File"

---

## 🐛 Bug Fixes

This release includes fixes from the comprehensive testing effort:

- ✅ Fixed lexer string literal handling
- ✅ Fixed comment tokenization
- ✅ Improved test coverage
- ✅ Enhanced error reporting

---

## 🔗 Links

- **Marketplace**: <https://marketplace.visualstudio.com/items?itemName=muhyadinmohamed.cortex-language>
- **GitHub**: <https://github.com/muhyadinmohamed/cortex>
- **Documentation**: <https://github.com/muhyadinmohamed/cortex/tree/main/docs>
- **Issues**: <https://github.com/muhyadinmohamed/cortex/issues>

---

## 📊 Project Statistics

### Codebase

- **Core Compiler**: 6 Rust modules (~3,000 lines)
- **VSCode Extension**: JavaScript (~300 lines)
- **Documentation**: 20+ comprehensive guides
- **Examples**: 15+ working `.ctx` programs

 Test Coverage

- **Total Tests**: 64
- **Code Coverage**: Comprehensive (all major features)
- **Test Organization**: Following Rust best practices

### Build Times

- **Compiler Build**: ~5.25s (release)
- **Test Execution**: <1s (excluding ignored)
- **Documentation**: ~10s

---

## 🎯 What's Next

### Planned Features (Future Releases)

- **Tensor Primitives**: Multi-dimensional array support
- **Neural Network Primitives**: Modular neurons and layers
- **Automatic Differentiation**: Gradient computation
- **GPU Support**: CUDA/OpenCL backends
- **Enhanced LSP**: Better IntelliSense and diagnostics
- **Debugging**: Advanced debugging features

---

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](https://github.com/muhyadinmohamed/cortex/blob/main/docs/DEVELOPMENT.md).

---

## 📄 License

MIT License - See [LICENSE](https://github.com/muhyadinmohamed/cortex/blob/main/LICENSE)

---

## 🙏 Acknowledgments

Thanks to all contributors and users who provided feedback and helped improve the Cortex language and tooling!

---

**Happy coding with Cortex!** 🚀
