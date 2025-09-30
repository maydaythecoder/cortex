//! Cortex Language Server
//! 
//! Implements the Language Server Protocol (LSP) for Cortex programming language.
//! Provides features like syntax highlighting, error checking, code completion,
//! go-to-definition, and more.

use tower_lsp::{LspService, Server};
use anyhow::Result;

mod server;
mod capabilities;
mod diagnostics;
mod completion;
mod hover;
mod references;
mod symbols;

use server::CortexLanguageServer;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    
    let (service, socket) = LspService::new(|client| CortexLanguageServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
    
    Ok(())
}