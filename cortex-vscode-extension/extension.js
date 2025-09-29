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
                
                // Get the Rust compiler path for formatting
                const rustDir = path.join(__dirname, '..', 'rust');
                const rustCompiler = path.join(rustDir, 'target', 'debug', 'cortexc');
                
                // Create a temporary file with the document content
                const tempFile = path.join(__dirname, 'temp_format.ctx');
                require('fs').writeFileSync(tempFile, document.getText());
                
                // Run the Rust formatter
                const command = `cd "${rustDir}" && cargo run -- format "${tempFile}" --indent ${indentSize}`;
                
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
            const rustDir = path.join(__dirname, '..', 'rust');
            const config = vscode.workspace.getConfiguration('cortex.compiler');
            const preferBinary = config.get('preferBinary', true);
            const configuredBinary = config.get('binaryPath', '').trim();

            const fallbackCargo = `cd "${rustDir}" && cargo run -- run "${filePath}"`;

            let command = fallbackCargo;
            if (configuredBinary) {
                command = `"${configuredBinary}" run "${filePath}"`;
            } else if (preferBinary) {
                // Try built binary in rust/target/debug or release
                const debugBin = path.join(rustDir, 'target', 'debug', 'cortexc');
                const releaseBin = path.join(rustDir, 'target', 'release', 'cortexc');
                if (fs.existsSync(debugBin)) {
                    command = `"${debugBin}" run "${filePath}"`;
                } else if (fs.existsSync(releaseBin)) {
                    command = `"${releaseBin}" run "${filePath}"`;
                }
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
        const rustDir = path.join(__dirname, '..', 'rust');
        const config = vscode.workspace.getConfiguration('cortex.compiler');
        const preferBinary = config.get('preferBinary', true);
        const configuredBinary = config.get('binaryPath', '').trim();

        const fallbackCargo = `cd "${rustDir}" && cargo run -- run "${filePath}"`;
        let cmd = fallbackCargo;
        if (configuredBinary) {
            cmd = `"${configuredBinary}" run "${filePath}"`;
        } else if (preferBinary) {
            const debugBin = path.join(rustDir, 'target', 'debug', 'cortexc');
            const releaseBin = path.join(rustDir, 'target', 'release', 'cortexc');
            if (fs.existsSync(debugBin)) {
                cmd = `"${debugBin}" run "${filePath}"`;
            } else if (fs.existsSync(releaseBin)) {
                cmd = `"${releaseBin}" run "${filePath}"`;
            }
        }

        const terminal = vscode.window.createTerminal({ name: 'Cortex Debug', env: { CORTEX_DEBUG: '1' } });
        terminal.sendText(cmd);
        terminal.show();
    });

    // Build compiler command
    const buildCompiler = vscode.commands.registerCommand('cortex.buildCompiler', async () => {
        const rustDir = path.join(__dirname, '..', 'rust');
        const terminal = vscode.window.createTerminal('Cortex Build');
        terminal.sendText(`cd "${rustDir}" && cargo build`);
        terminal.show();
    });

    context.subscriptions.push(formatter, formatCommand, runCommand, debugCommand, buildCompiler);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
