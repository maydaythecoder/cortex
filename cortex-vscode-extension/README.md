# Cortex Language Extension

![Cortex Language Logo](./cortexlogo.png)

_Syntax highlighting, formatting, and language tooling for Cortex._

Syntax highlighting, formatting, and language support for the Cortex programming language in VS Code.

## Features

- **Syntax Highlighting**: Beautiful syntax highlighting for Cortex `.ctx` files
- **Code Formatting**: Automatic code formatting with configurable indentation
- **File Icons**: Custom Cortex brain icon for `.ctx` files
- **Language Support**: Full language configuration for Cortex
- **Commands**: Format and run Cortex files with keyboard shortcuts

## Installation

### From VS Code Marketplace

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Cortex Language"
4. Click Install

### From VSIX

1. Download the `.vsix` file
2. Open VS Code
3. Go to Extensions (Ctrl+Shift+X)
4. Click "..." menu → "Install from VSIX..."
5. Select the downloaded `.vsix` file

## Usage

### Syntax Highlighting

Open any `.ctx` file and enjoy beautiful syntax highlighting with the custom Cortex color palette.

### Code Formatting

- **Format on Save**: Automatically enabled for `.ctx` files
- **Manual Formatting**: Press `Ctrl+Shift+F` (or `Cmd+Shift+F` on macOS)
- **Command Palette**: "Format Document" or "Cortex: Format Cortex File"

### Running Cortex Files

- **Keyboard Shortcut**: Press `Ctrl+Shift+R` (or `Cmd+Shift+R` on macOS)
- **Command Palette**: "Cortex: Run Cortex File"

## Configuration

The extension provides the following settings:

- `cortex.format.indentSize`: Number of spaces for indentation (default: 2)
- `cortex.format.formatOnSave`: Format files on save (default: true)

## Color Palette

The extension uses a custom color palette:

- **Keywords**: `#9A48D0` (Purple)
- **Strings**: `#6BBAEC` (Blue)
- **Numbers**: `#F0544F` (Red)
- **Functions**: `#F6AE2D` (Orange)
- **Comments**: `#065143` (Dark Green)
- **Variables**: `#69995D` (Green)

## Requirements

- VS Code 1.74.0 or higher

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- **Cortex Language**: [GitHub Repository](https://github.com/muhyadinmohamed/cortex)
- **Documentation**: [Cortex Docs](https://github.com/muhyadinmohamed/cortex/tree/main/docs)
- **Issues**: [Report Issues](https://github.com/muhyadinmohamed/cortex/issues)
