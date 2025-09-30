# Cortex VSCode Extension Development Guide

Complete guide to developing, testing, and deploying the Cortex language extension for Visual Studio Code.

## Table of Contents

1. [Extension Features](#extension-features)
2. [Architecture Overview](#architecture-overview)
3. [Adding IntelliSense](#adding-intellisense)
4. [Creating Snippets](#creating-snippets)
5. [Testing the Extension](#testing-the-extension)
6. [Deployment](#deployment)
7. [Troubleshooting](#troubleshooting)

---

## Extension Features

### Current Features

✅ **Syntax Highlighting** - Full `.ctx` file syntax support  
✅ **Code Formatting** - Auto-format with bundled compiler  
✅ **Run Button** - Play button in editor toolbar  
✅ **Code Lens** - Inline Run/Debug buttons  
✅ **Bundled Compiler** - No external dependencies  
✅ **IntelliSense** - Auto-completion for keywords, functions, types  
✅ **Hover Documentation** - Inline docs on hover  
✅ **Snippets** - 20+ code snippets for rapid development  
✅ **Context Menu** - Right-click Run/Debug/Check/Build  
✅ **Keyboard Shortcuts** - `Ctrl+Shift+R` to run, `F5` to debug  

---

## Architecture Overview

### File Structure

``` txt
packages/vscode-extension/
├── extension.js              # Main extension logic
├── package.json              # Extension manifest
├── cortexc                   # Bundled compiler binary
├── syntaxes/
│   └── cortex.tmLanguage.json  # Syntax grammar
├── snippets/
│   └── cortex.json           # Code snippets
├── icons/
│   ├── cortex-dark.svg       # Dark theme icon
│   └── cortex-light.svg      # Light theme icon
├── language-configuration.json  # Brackets, comments config
└── README.md                 # User-facing docs
```

### Key Components

1. **extension.js** - Registers all providers and commands
2. **package.json** - Defines contributions (commands, menus, snippets)
3. **cortexc** - Rust compiler binary for run/format/check
4. **Snippets** - Pre-defined code templates
5. **IntelliSense Providers** - Completion and hover providers

---

## Adding IntelliSense

### 1. Completion Provider (Auto-complete)

The completion provider suggests items as you type. Located in `extension.js`:

```javascript
const completionProvider = vscode.languages.registerCompletionItemProvider('cortex', {
    provideCompletionItems(document, position, token, context) {
        const completions = [];
        
        // Add completion item
        const item = new vscode.CompletionItem('myfunction', vscode.CompletionItemKind.Function);
        item.detail = 'Description of function';
        item.insertText = new vscode.SnippetString('myfunction[${1:param}]');
        completions.push(item);
        
        return completions;
    }
}, '[', ' ', ':');  // Trigger characters
```

### 2. Hover Provider (Documentation)

Shows documentation when hovering over keywords:

```javascript
const hoverProvider = vscode.languages.registerHoverProvider('cortex', {
    provideHover(document, position, token) {
        const range = document.getWordRangeAtPosition(position);
        const word = document.getText(range);
        
        const docs = {
            'keyword': '**Description**\n\n```cortex\ncode example\n```\nExplanation'
        };
        
        if (docs[word]) {
            return new vscode.Hover(new vscode.MarkdownString(docs[word]));
        }
    }
});
```

### 3. Register Providers

Always register providers in `context.subscriptions`:

```javascript
context.subscriptions.push(completionProvider, hoverProvider);
```

### Adding New Keywords

1. **Open `extension.js`**
2. **Find `provideCompletionItems` function**
3. **Add to the `keywords` array**:

```javascript
const keywords = ['func', 'let', 'if', 'else', 'your_new_keyword'];
```

### Adding New Built-in Functions

1. **Find the `builtins` array in `extension.js`**
2. **Add new function**:

```javascript
const builtins = [
    // ... existing functions
    { 
        name: 'mynewfunc', 
        detail: 'What it does', 
        snippet: 'mynewfunc[${1:param1}, ${2:param2}]' 
    }
];
```

### Adding Hover Documentation

1. **Find the `docs` object in `hoverProvider`**
2. **Add new entry**:

```javascript
const docs = {
    'mynewfunc': '**My New Function**\n\n```cortex\nmynewfunc[param]\n```\nDetailed description'
};
```

---

## Creating Snippets

### Snippet File Location

`snippets/cortex.json`

### Snippet Syntax

```json
{
  "Snippet Name": {
    "prefix": "shortcut",
    "body": [
      "line 1 with ${1:placeholder}",
      "line 2 with ${2:default_value}",
      "line 3"
    ],
    "description": "What this snippet does"
  }
}
```

### Placeholder Syntax

- `${1:name}` - First tab stop with placeholder text "name"
- `${2}` - Second tab stop (no placeholder)
- `${1|opt1,opt2,opt3|}` - Dropdown selection
- `$0` - Final cursor position

### Example: Adding a Loop Snippet

1. **Open `snippets/cortex.json`**
2. **Add new snippet**:

```json
{
  "Custom Loop": {
    "prefix": "myloop",
    "body": [
      "for [${1:i} in range[${2:0}, ${3:10}]] |",
      "  ${4:// loop body}",
      "^"
    ],
    "description": "Custom loop with range"
  }
}
```

Save file
Reload extension (see Testing section)

### Best Practices for Snippets

✅ Use descriptive prefixes (2-5 characters)  
✅ Include helpful placeholder text  
✅ Add descriptions for clarity  
✅ Follow Cortex syntax conventions  
✅ Test snippets before publishing  

---

## Testing the Extension

### Method 1: Local Development

1. **Open extension directory in VSCode**:

   ```bash
   code /Users/muhyadinmohamed/Documents/Development/cortex/packages/vscode-extension
   ```

2. **Press `F5`** to launch Extension Development Host

3. **Test in new window**:
   - Create a `.ctx` file
   - Type snippet prefix + Tab
   - Trigger IntelliSense with `Ctrl+Space`
   - Hover over keywords

### Method 2: Install Packaged Extension

1. **Package the extension**:

   ```bash
   cd /Users/muhyadinmohamed/Documents/Development/cortex/packages/vscode-extension
   vsce package --no-dependencies --allow-star-activation
   ```

2. **Install**:

   ```bash
   code --install-extension cortex-language-1.0.0.vsix
   ```

3. **Test**:
   - Open any `.ctx` file
   - Verify all features work

### What to Test

- [ ] Snippets trigger correctly
- [ ] IntelliSense appears on typing
- [ ] Hover docs display
- [ ] Run button works
- [ ] Code lens buttons appear
- [ ] Syntax highlighting correct
- [ ] Formatting works

---

## Deployment

### Prerequisites

1. **Install vsce** (VSCode Extension Manager):

   ```bash
   npm install -g vsce
   ```

2. **Get Personal Access Token (PAT)**:
   - Go to <https://dev.azure.com>
   - Create organization
   - User Settings → Personal Access Tokens
   - Create token with `Marketplace (Manage)` scope
   - Save token securely

### Publishing Steps

#### 1. Update Version

Edit `package.json`:

```json
{
  "version": "1.1.0"  // Bump version
}
```

#### 2. Package Extension

```bash
cd /Users/muhyadinmohamed/Documents/Development/cortex/packages/vscode-extension
vsce package --no-dependencies --allow-star-activation
```

This creates `cortex-language-1.1.0.vsix`

#### 3. Test Locally

```bash
code --install-extension cortex-language-1.1.0.vsix
```

Verify all features work!

#### 4. Publish to Marketplace

```bash
vsce publish --no-dependencies --allow-star-activation --pat YOUR_PAT_HERE
```

Replace `YOUR_PAT_HERE` with your actual PAT.

#### 5. Verify Publication

Visit: <https://marketplace.visualstudio.com/items?itemName=muhyadinmohamed.cortex-language>

### Quick Publish Command

```bash
cd /Users/muhyadinmohamed/Documents/Development/cortex/packages/vscode-extension && \
vsce publish --no-dependencies --allow-star-activation --pat YOUR_PAT_HERE
```

### Version Bump Guidelines

- **Patch** (1.0.0 → 1.0.1): Bug fixes, typos
- **Minor** (1.0.0 → 1.1.0): New features, snippets, IntelliSense updates
- **Major** (1.0.0 → 2.0.0): Breaking changes, major rewrites

---

## Troubleshooting

### Snippets Not Working

**Problem**: Snippet doesn't trigger  
**Solution**:

1. Check `snippets/cortex.json` syntax (must be valid JSON)
2. Verify `package.json` includes snippets contribution:

   ```json
   "contributes": {
     "snippets": [
       {
         "language": "cortex",
         "path": "./snippets/cortex.json"
       }
     ]
   }
   ```

3. Reload VSCode window (`Ctrl+Shift+P` → "Reload Window")

### IntelliSense Not Appearing

**Problem**: No autocomplete suggestions  
**Solution**:

1. Check file is recognized as Cortex (bottom-right of VSCode)
2. Verify completion provider is registered in `extension.js`
3. Check console for errors (`Help` → `Toggle Developer Tools`)
4. Try triggering manually with `Ctrl+Space`

### Run Button Missing

**Problem**: No play button in toolbar  
**Solution**:

1. Verify `package.json` has menu contribution:

   ```json
   "menus": {
     "editor/title/run": [
       {
         "command": "cortex.run",
         "when": "resourceLangId == cortex"
       }
     ]
   }
   ```

2. Ensure file is `.ctx` extension
3. Check command is registered in `extension.js`

### Compiler Not Found

**Problem**: "cortexc not found" error  
**Solution**:

1. Verify `cortexc` binary exists in extension directory
2. Check it's executable: `chmod +x cortexc`
3. Ensure it's included in package (not in `.vscodeignore`)

### Publishing Fails

**Problem**: `vsce publish` errors  
**Solution**:

Error: PAT invalid

- Regenerate PAT with correct scopes
- Ensure `Marketplace (Manage)` is selected

Error: Invalid path

- Add to `.vscodeignore`:

  ``` txt
  ../../.git/**
  ../../.github/**
  ```

Error: Compilation failed

- Check `package.json` scripts
- We use JavaScript, not TypeScript, so:

  ```json
  "scripts": {
    "vscode:prepublish": "echo 'Using JavaScript - no compilation needed'"
  }
  ```

---

## Advanced Features

### Adding Signature Help

Show parameter hints while typing function calls:

```javascript
const signatureProvider = vscode.languages.registerSignatureHelpProvider('cortex', {
    provideSignatureHelp(document, position) {
        const signature = new vscode.SignatureInformation('func[param1, param2]');
        signature.parameters = [
            new vscode.ParameterInformation('param1', 'First parameter'),
            new vscode.ParameterInformation('param2', 'Second parameter')
        ];
        const help = new vscode.SignatureHelp();
        help.signatures = [signature];
        return help;
    }
}, '[', ',');
```

### Adding Definition Provider

Jump to function definitions:

```javascript
const definitionProvider = vscode.languages.registerDefinitionProvider('cortex', {
    provideDefinition(document, position) {
        // Parse document, find definition
        // Return location of definition
        return new vscode.Location(
            vscode.Uri.file('/path/to/file.ctx'),
            new vscode.Position(line, char)
        );
    }
});
```

### Adding Diagnostic Provider

Show errors/warnings inline:

```javascript
const diagnosticCollection = vscode.languages.createDiagnosticCollection('cortex');

function updateDiagnostics(document) {
    const diagnostics = [];
    // Parse document, find errors
    const diagnostic = new vscode.Diagnostic(
        range,
        'Error message',
        vscode.DiagnosticSeverity.Error
    );
    diagnostics.push(diagnostic);
    diagnosticCollection.set(document.uri, diagnostics);
}
```

---

## Resources

- **VSCode Extension API**: <https://code.visualstudio.com/api>
- **Extension Samples**: <https://github.com/microsoft/vscode-extension-samples>
- **TextMate Grammars**: <https://macromates.com/manual/en/language_grammars>
- **Publishing Extensions**: <https://code.visualstudio.com/api/working-with-extensions/publishing-extension>
- **Marketplace**: <https://marketplace.visualstudio.com/vscode>

---

## Quick Reference

### File Edits Checklist

When adding new features, update:

- [ ] `extension.js` - Add providers/commands
- [ ] `package.json` - Add contributions
- [ ] `snippets/cortex.json` - Add snippets
- [ ] `README.md` - Update user docs
- [ ] Bump version in `package.json`
- [ ] Test locally
- [ ] Publish

### Common Commands

```bash
# Package extension
vsce package --no-dependencies --allow-star-activation

# Install locally
code --install-extension cortex-language-VERSION.vsix

# Publish
vsce publish --no-dependencies --allow-star-activation --pat YOUR_PAT

# Uninstall
code --uninstall-extension muhyadinmohamed.cortex-language
```

---

**Need Help?**  
Open an issue: <https://github.com/muhyadinmohamed/cortex/issues>
