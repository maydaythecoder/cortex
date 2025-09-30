//! Main language server implementation

use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use anyhow::Result;
use std::collections::HashMap;
use ropey::Rope;

use crate::capabilities::Capabilities;
use crate::diagnostics::Diagnostics;
use crate::completion::Completion;
use crate::hover::Hover;
use crate::references::References;
use crate::symbols::Symbols;

pub struct CortexLanguageServer {
    client: Client,
    capabilities: Capabilities,
    diagnostics: Diagnostics,
    completion: Completion,
    hover: Hover,
    references: References,
    symbols: Symbols,
    documents: HashMap<Url, Rope>,
}

impl CortexLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            capabilities: Capabilities::new(),
            diagnostics: Diagnostics::new(),
            completion: Completion::new(),
            hover: Hover::new(),
            references: References::new(),
            symbols: Symbols::new(),
            documents: HashMap::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for CortexLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: self.capabilities.get_capabilities(),
            server_info: Some(ServerInfo {
                name: "cortex-lsp".to_string(),
                version: Some("1.0.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Cortex Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn did_open(&mut self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        self.documents.insert(uri.clone(), Rope::from_str(&text));
        
        // Send diagnostics
        if let Ok(diagnostics) = self.diagnostics.analyze(&text) {
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_change(&mut self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        
        if let Some(document) = self.documents.get_mut(&uri) {
            for change in params.content_changes {
                match change {
                    TextDocumentContentChangeEvent {
                        range: Some(range),
                        range_length: _,
                        text,
                    } => {
                        let start = document.line_to_char(range.start.line as usize)
                            + range.start.character as usize;
                        let end = document.line_to_char(range.end.line as usize)
                            + range.end.character as usize;
                        
                        document.remove(start..end);
                        document.insert(start, &text);
                    }
                    TextDocumentContentChangeEvent {
                        range: None,
                        range_length: None,
                        text,
                    } => {
                        *document = Rope::from_str(&text);
                    }
                    _ => {}
                }
            }
            
            // Send updated diagnostics
            if let Ok(diagnostics) = self.diagnostics.analyze(&document.to_string()) {
                self.client
                    .publish_diagnostics(uri, diagnostics, None)
                    .await;
            }
        }
    }

    async fn did_close(&mut self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        if let Some(document) = self.documents.get(&uri) {
            let completions = self.completion.get_completions(document, position);
            Ok(Some(CompletionResponse::Array(completions)))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        if let Some(document) = self.documents.get(&uri) {
            self.hover.get_hover(document, position)
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        if let Some(document) = self.documents.get(&uri) {
            self.references.find_definition(document, position, &self.documents)
        } else {
            Ok(None)
        }
    }

    async fn references(&self, params: ReferenceParams) -> LspResult<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        if let Some(document) = self.documents.get(&uri) {
            self.references.find_references(document, position, &self.documents)
        } else {
            Ok(None)
        }
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> LspResult<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        
        if let Some(document) = self.documents.get(&uri) {
            let symbols = self.symbols.get_document_symbols(document);
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        } else {
            Ok(None)
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> LspResult<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        
        if let Some(document) = self.documents.get(&uri) {
            let formatted = self.format_document(document);
            Ok(Some(vec![TextEdit {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position {
                        line: document.len_lines() as u32 - 1,
                        character: document.line(document.len_lines() - 1).len_chars() as u32,
                    },
                },
                new_text: formatted,
            }]))
        } else {
            Ok(None)
        }
    }
}

impl CortexLanguageServer {
    fn format_document(&self, document: &Rope) -> String {
        // Simple formatting implementation
        // In a real implementation, you'd use the Cortex formatter
        let mut formatted = String::new();
        let mut indent_level = 0;
        
        for line in document.lines() {
            let line_str = line.to_string();
            let trimmed = line_str.trim();
            
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }
            
            // Handle block endings
            if trimmed == "^" {
                indent_level = indent_level.saturating_sub(1);
            }
            
            // Add indentation
            for _ in 0..indent_level {
                formatted.push_str("  ");
            }
            formatted.push_str(trimmed);
            formatted.push('\n');
            
            // Handle block beginnings
            if trimmed.ends_with(" |") {
                indent_level += 1;
            }
        }
        
        formatted
    }
}