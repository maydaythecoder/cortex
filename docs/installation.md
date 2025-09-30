# Installation Guide

This guide will help you install Cortex and its development tools on your system.

## System Requirements

### Minimum Requirements
- **Operating System**: macOS 10.15+, Ubuntu 18.04+, Windows 10+
- **Memory**: 4GB RAM
- **Storage**: 2GB free space
- **Network**: Internet connection for downloading dependencies

### Recommended Requirements
- **Operating System**: macOS 12+, Ubuntu 20.04+, Windows 11
- **Memory**: 8GB RAM
- **Storage**: 5GB free space
- **CPU**: Multi-core processor

## Prerequisites

### 1. Install Rust

Cortex is built in Rust, so you'll need Rust installed:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or run:
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

**Minimum Rust version**: 1.70.0

### 2. Install Node.js (for VSCode extension)

```bash
# Using Node Version Manager (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install 18
nvm use 18

# Or download from nodejs.org
# Visit https://nodejs.org and download LTS version

# Verify installation
node --version
npm --version
```

**Minimum Node.js version**: 16.0.0

### 3. Install Git

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install git

# macOS (with Homebrew)
brew install git

# Windows
# Download from https://git-scm.com/download/win

# Verify installation
git --version
```

## Installation Methods

### Method 1: From Source (Recommended)

This method gives you the latest development version with all features:

```bash
# Clone the repository
git clone https://github.com/muhyadinmohamed/cortex.git
cd cortex

# Install dependencies
npm install

# Build all components
npm run build

# Verify installation
cortex --version
```

### Method 2: Pre-built Binaries

Download pre-built binaries from the [Releases page](https://github.com/muhyadinmohamed/cortex/releases):

```bash
# Download the appropriate binary for your system
# Extract and add to PATH

# Example for Linux x64:
wget https://github.com/muhyadinmohamed/cortex/releases/latest/download/cortex-linux-x64.tar.gz
tar -xzf cortex-linux-x64.tar.gz
sudo mv cortex /usr/local/bin/

# Verify installation
cortex --version
```

### Method 3: Package Managers

#### macOS (Homebrew)
```bash
# Add the tap (when available)
brew tap cortex-team/cortex

# Install Cortex
brew install cortex
```

#### Ubuntu/Debian (APT)
```bash
# Add the repository (when available)
curl -fsSL https://packages.cortex-lang.org/apt/key.gpg | sudo gpg --dearmor -o /usr/share/keyrings/cortex-archive-keyring.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/cortex-archive-keyring.gpg] https://packages.cortex-lang.org/apt stable main" | sudo tee /etc/apt/sources.list.d/cortex.list

# Install Cortex
sudo apt update
sudo apt install cortex
```

#### Windows (Chocolatey)
```powershell
# Install Cortex (when available)
choco install cortex
```

## VSCode Extension Installation

### Method 1: From Marketplace (Recommended)

1. Open VSCode
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Cortex Language"
4. Click Install

### Method 2: From VSIX File

```bash
# Download the extension
wget https://github.com/muhyadinmohamed/cortex/releases/latest/download/cortex-language-1.0.0.vsix

# Install the extension
code --install-extension cortex-language-1.0.0.vsix
```

### Method 3: From Source

```bash
# Build the extension
cd packages/vscode-extension
npm install
npm run compile
npm run package

# Install the built extension
code --install-extension cortex-language-1.0.0.vsix
```

## Verification

After installation, verify everything is working:

```bash
# Check Cortex CLI
cortex --version
cortex --help

# Check compiler
cortexc --version

# Check language server
cortex-lsp --version

# Test with a simple program
echo 'print["Hello, Cortex!"]' > test.ctx
cortex run test.ctx
```

Expected output:
```
Cortex CLI 1.0.0
Cortex Compiler 1.0.0
Cortex Language Server 1.0.0
Hello, Cortex!
```

## Troubleshooting

### Common Issues

#### 1. Rust Installation Issues

**Problem**: `rustc: command not found`

**Solution**:
```bash
# Reinstall Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Or add to PATH manually
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### 2. Node.js Version Issues

**Problem**: `npm: command not found` or version too old

**Solution**:
```bash
# Update Node.js using nvm
nvm install 18
nvm use 18
nvm alias default 18
```

#### 3. Build Failures

**Problem**: Compilation errors during build

**Solution**:
```bash
# Clean and rebuild
npm run clean
npm run build

# Or build individual components
cd packages/compiler && cargo build
cd packages/cli && cargo build
cd packages/language-server && cargo build
```

#### 4. Permission Issues

**Problem**: Permission denied when installing

**Solution**:
```bash
# Fix npm permissions
sudo chown -R $(whoami) ~/.npm
sudo chown -R $(whoami) /usr/local/lib/node_modules

# Or use a Node version manager
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
```

#### 5. VSCode Extension Issues

**Problem**: Extension not working or not recognized

**Solution**:
```bash
# Reload VSCode window
# Or restart VSCode completely

# Check extension is installed
code --list-extensions | grep cortex

# Reinstall extension
code --uninstall-extension cortex-team.cortex-language
code --install-extension cortex-team.cortex-language
```

### Getting Help

If you encounter issues not covered here:

1. **Check the logs**: Look for error messages in the terminal output
2. **Search issues**: Check [GitHub Issues](https://github.com/muhyadinmohamed/cortex/issues)
3. **Ask for help**: Create a new issue with:
   - Your operating system and version
   - Installation method used
   - Complete error messages
   - Steps to reproduce the issue

## Next Steps

After successful installation:

1. **Follow the [Quick Start Guide](quick-start.md)**
2. **Read the [First Program Tutorial](first-program.md)**
3. **Explore [Examples](examples/)**
4. **Join the [Community](https://github.com/muhyadinmohamed/cortex/discussions)**

---

**Welcome to Cortex!** 🎉