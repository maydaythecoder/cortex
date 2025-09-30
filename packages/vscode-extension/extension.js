const vscode = require('vscode');
const { exec } = require('child_process');
const path = require('path');
const fs = require('fs');

function activate(context) {
    console.log('Cortex extension is now active!');

    // Register formatter
    const formatter = vscode.languages.registerDocumentFormattingEditProvider('cortex', {
        provideDocumentFormattingEdits(document, options, token) {
            return new Promise((resolve, reject) => {
                const config = vscode.workspace.getConfiguration('cortex.format');
                const indentSize = config.get('indentSize', 2);
                
                // Get the bundled compiler path for formatting
                const bundledCompiler = path.join(__dirname, 'cortexc');
                
                // Create a temporary file with the document content
                const tempFile = path.join(__dirname, 'temp_format.ctx');
                require('fs').writeFileSync(tempFile, document.getText());
                
                // Run the bundled compiler formatter
                const command = `"${bundledCompiler}" format "${tempFile}" --indent ${indentSize}`;
                
                exec(command, (error, stdout, stderr) => {
                    if (error) {
                        console.error('Formatter error:', error);
                        reject(error);
                        return;
                    }
                    
                    try {
                        // Read the formatted content
                        const formattedContent = require('fs').readFileSync(tempFile, 'utf8');
                        
                        // Clean up temp file
                        require('fs').unlinkSync(tempFile);
                        
                        // Create the edit
                        const fullRange = new vscode.Range(
                            document.positionAt(0),
                            document.positionAt(document.getText().length)
                        );
                        
                        const edit = vscode.TextEdit.replace(fullRange, formattedContent);
                        resolve([edit]);
                    } catch (err) {
                        console.error('Error processing formatted content:', err);
                        reject(err);
                    }
                });
            });
        }
    });

    // Register command for manual formatting
    const formatCommand = vscode.commands.registerCommand('cortex.format', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'cortex') {
            vscode.commands.executeCommand('editor.action.formatDocument');
        }
    });

    // Register command for running Cortex files
    const runCommand = vscode.commands.registerCommand('cortex.run', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'cortex') {
            const filePath = editor.document.fileName;
            const config = vscode.workspace.getConfiguration('cortex.compiler');
            const configuredBinary = config.get('binaryPath', '').trim();

            // Use bundled compiler by default
            const bundledCompiler = path.join(__dirname, 'cortexc');
            let command = `"${bundledCompiler}" run "${filePath}"`;
            
            if (configuredBinary) {
                command = `"${configuredBinary}" run "${filePath}"`;
            }

            const terminal = vscode.window.createTerminal('Cortex');
            terminal.sendText(command);
            terminal.show();
        }
    });

    // Register command for debugging Cortex files (launch with extra logging)
    const debugCommand = vscode.commands.registerCommand('cortex.debug', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cortex') {
            vscode.window.showWarningMessage('Open a Cortex (.ctx) file to debug.');
            return;
        }
        const filePath = editor.document.fileName;
        const config = vscode.workspace.getConfiguration('cortex.compiler');
        const configuredBinary = config.get('binaryPath', '').trim();

        // Use bundled compiler by default
        const bundledCompiler = path.join(__dirname, 'cortexc');
        let cmd = `"${bundledCompiler}" run "${filePath}"`;
        
        if (configuredBinary) {
            cmd = `"${configuredBinary}" run "${filePath}"`;
        }

        const terminal = vscode.window.createTerminal({ name: 'Cortex Debug', env: { CORTEX_DEBUG: '1' } });
        terminal.sendText(cmd);
        terminal.show();
    });

    // Build compiler command
    const buildCompiler = vscode.commands.registerCommand('cortex.buildCompiler', async () => {
        const rustDir = path.join(__dirname, '..', 'rust');
        const terminal = vscode.window.createTerminal('Cortex Build');
        terminal.sendText(`cd "${rustDir}" && cargo build && cp target/debug/cortexc "${__dirname}/cortexc"`);
        terminal.show();
    });

    // Register command for checking Cortex file syntax
    const checkCommand = vscode.commands.registerCommand('cortex.check', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'cortex') {
            const filePath = editor.document.fileName;
            const config = vscode.workspace.getConfiguration('cortex.compiler');
            const configuredBinary = config.get('binaryPath', '').trim();

            // Use bundled compiler by default
            const bundledCompiler = path.join(__dirname, 'cortexc');
            let command = `"${bundledCompiler}" check "${filePath}"`;
            
            if (configuredBinary) {
                command = `"${configuredBinary}" check "${filePath}"`;
            }

            const terminal = vscode.window.createTerminal('Cortex Check');
            terminal.sendText(command);
            terminal.show();
        }
    });

    // Register command for building Cortex files to executable
    const buildCommand = vscode.commands.registerCommand('cortex.build', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'cortex') {
            const filePath = editor.document.fileName;
            const config = vscode.workspace.getConfiguration('cortex.compiler');
            const configuredBinary = config.get('binaryPath', '').trim();

            // Use bundled compiler by default
            const bundledCompiler = path.join(__dirname, 'cortexc');
            const outputPath = filePath.replace('.ctx', '');
            let command = `"${bundledCompiler}" build "${filePath}" --output "${outputPath}"`;
            
            if (configuredBinary) {
                command = `"${configuredBinary}" build "${filePath}" --output "${outputPath}"`;
            }

            const terminal = vscode.window.createTerminal('Cortex Build');
            terminal.sendText(command);
            terminal.show();
        }
    });

    // Register code lens provider for run/debug buttons in editor
    const codeLensProvider = vscode.languages.registerCodeLensProvider('cortex', {
        provideCodeLenses(document) {
            const codeLenses = [];
            const topOfDocument = new vscode.Range(0, 0, 0, 0);
            
            // Add "▶ Run" button at top of file
            const runLens = new vscode.CodeLens(topOfDocument, {
                title: '▶ Run',
                command: 'cortex.run',
                tooltip: 'Run this Cortex file'
            });
            
            // Add "🐛 Debug" button at top of file
            const debugLens = new vscode.CodeLens(topOfDocument, {
                title: '🐛 Debug',
                command: 'cortex.debug',
                tooltip: 'Debug this Cortex file'
            });
            
            codeLenses.push(runLens, debugLens);
            return codeLenses;
        }
    });

    // Register completion provider for IntelliSense
    const completionProvider = vscode.languages.registerCompletionItemProvider('cortex', {
        provideCompletionItems(document, position, token, context) {
            const completions = [];

            // Keywords
            const keywords = ['func', 'let', 'if', 'else', 'while', 'for', 'in', 'return', 'true', 'false', 'null', 'try', 'catch'];
            keywords.forEach(keyword => {
                const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
                item.detail = `Cortex keyword`;
                completions.push(item);
            });

            // Built-in functions
            const builtins = [
                { name: 'print', detail: 'Print to console', snippet: 'print[${1:value}]' },
                { name: 'range', detail: 'Generate range', snippet: 'range[${1:start}, ${2:end}]' },
                { name: 'len', detail: 'Get length', snippet: 'len[${1:array}]' },
                { name: 'push', detail: 'Add to array', snippet: 'push[${1:array}, ${2:value}]' },
                { name: 'pop', detail: 'Remove from array', snippet: 'pop[${1:array}]' },
                { name: 'sum', detail: 'Sum of array', snippet: 'sum[${1:array}]' },
                { name: 'mean', detail: 'Mean of array', snippet: 'mean[${1:array}]' },
                { name: 'max', detail: 'Maximum value', snippet: 'max[${1:array}]' },
                { name: 'min', detail: 'Minimum value', snippet: 'min[${1:array}]' }
            ];
            builtins.forEach(fn => {
                const item = new vscode.CompletionItem(fn.name, vscode.CompletionItemKind.Function);
                item.detail = fn.detail;
                item.insertText = new vscode.SnippetString(fn.snippet);
                completions.push(item);
            });

            // ML/AI functions
            const mlFunctions = [
                { name: 'tensor', detail: 'Create tensor', snippet: 'tensor[shape: [${1:dims}], data: [${2:values}]]' },
                { name: 'layer', detail: 'Neural network layer', snippet: 'layer[type: "${1|dense,conv,pool|}", units: ${2:128}]' },
                { name: 'model', detail: 'Create model', snippet: 'model[layers: [${1:layers}]]' },
                { name: 'train', detail: 'Train model', snippet: 'train[model: ${1:model}, data: ${2:X}, labels: ${3:y}]' },
                { name: 'matmul', detail: 'Matrix multiplication', snippet: 'matmul[${1:A}, ${2:B}]' },
                { name: 'dot', detail: 'Dot product', snippet: 'dot[${1:a}, ${2:b}]' },
                { name: 'relu', detail: 'ReLU activation', snippet: 'relu[${1:x}]' },
                { name: 'sigmoid', detail: 'Sigmoid activation', snippet: 'sigmoid[${1:x}]' },
                { name: 'tanh', detail: 'Tanh activation', snippet: 'tanh[${1:x}]' },
                { name: 'softmax', detail: 'Softmax activation', snippet: 'softmax[${1:x}]' },
                { name: 'gradient_descent', detail: 'Gradient descent', snippet: 'gradient_descent[loss_fn: ${1:fn}, initial: ${2:w0}]' }
            ];
            mlFunctions.forEach(fn => {
                const item = new vscode.CompletionItem(fn.name, vscode.CompletionItemKind.Function);
                item.detail = fn.detail + ' (ML/AI)';
                item.insertText = new vscode.SnippetString(fn.snippet);
                completions.push(item);
            });

            // Type annotations
            const types = ['number', 'string', 'boolean', 'array', 'tensor', 'model'];
            types.forEach(type => {
                const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.TypeParameter);
                item.detail = `Cortex type`;
                completions.push(item);
            });

            return completions;
        }
    }, '[', ' ', ':');  // Trigger characters

    // Register hover provider for documentation
    const hoverProvider = vscode.languages.registerHoverProvider('cortex', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);

            const docs = {
                'func': '**Function Definition**\n\n```cortex\nfunc name[params] | body ^\n```\nDefines a function with parameters',
                'let': '**Variable Declaration**\n\n`let name := value` (mutable)\n\n`let name :: value` (constant)',
                'print': '**Print Function**\n\n```cortex\nprint[value]\n```\nPrints value to console',
                'tensor': '**Tensor Creation**\n\n```cortex\ntensor[shape: [dims], data: [values]]\n```\nCreates a multi-dimensional tensor',
                'train': '**Model Training**\n\n```cortex\ntrain[model: m, data: X, labels: y, epochs: 100]\n```\nTrains a machine learning model',
                'matmul': '**Matrix Multiplication**\n\n```cortex\nmatmul[A, B]\n```\nPerforms matrix multiplication',
                'relu': '**ReLU Activation**\n\n```cortex\nrelu[x]\n```\nRectified Linear Unit: max(0, x)',
                'sigmoid': '**Sigmoid Activation**\n\n```cortex\nsigmoid[x]\n```\nSigmoid function: 1 / (1 + e^(-x))'
            };

            if (docs[word]) {
                return new vscode.Hover(new vscode.MarkdownString(docs[word]));
            }
        }
    });

    context.subscriptions.push(
        formatter, 
        formatCommand, 
        runCommand, 
        debugCommand, 
        buildCompiler, 
        checkCommand, 
        buildCommand,
        codeLensProvider,
        completionProvider,
        hoverProvider
    );
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
