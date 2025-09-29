const vscode = require('vscode');
const { exec } = require('child_process');
const path = require('path');

function activate(context) {
    console.log('Cortex extension is now active!');

    // Register formatter
    const formatter = vscode.languages.registerDocumentFormattingEditProvider('cortex', {
        provideDocumentFormattingEdits(document, options, token) {
            return new Promise((resolve, reject) => {
                const config = vscode.workspace.getConfiguration('cortex.format');
                const indentSize = config.get('indentSize', 2);
                
                // Get the formatter script path
                const formatterPath = path.join(__dirname, '..', 'format_cortex.py');
                
                // Create a temporary file with the document content
                const tempFile = path.join(__dirname, 'temp_format.ctx');
                require('fs').writeFileSync(tempFile, document.getText());
                
                // Run the formatter
                const command = `python3 "${formatterPath}" "${tempFile}" --indent ${indentSize}`;
                
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
            const runScript = path.join(__dirname, '..', 'run_cortex.sh');
            
            const terminal = vscode.window.createTerminal('Cortex');
            terminal.sendText(`"${runScript}" "${filePath}"`);
            terminal.show();
        }
    });

    context.subscriptions.push(formatter, formatCommand, runCommand);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
