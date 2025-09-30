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

    context.subscriptions.push(
        formatter, 
        formatCommand, 
        runCommand, 
        debugCommand, 
        buildCompiler, 
        checkCommand, 
        buildCommand,
        codeLensProvider
    );
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
