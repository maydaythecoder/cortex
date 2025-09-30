//! Command implementations for Cortex CLI

pub mod new;
pub mod run;
pub mod build;
pub mod test;
pub mod format;
pub mod lint;
pub mod debug;
pub mod package;
pub mod project;
pub mod doc;
pub mod clean;
pub mod version;

use anyhow::Result;
use std::path::PathBuf;
use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    pb
}

pub fn find_cortex_file(path: Option<PathBuf>) -> Result<PathBuf> {
    match path {
        Some(p) => {
            if p.is_file() {
                Ok(p)
            } else if p.is_dir() {
                // Look for main.ctx or index.ctx
                let main_ctx = p.join("main.ctx");
                let index_ctx = p.join("index.ctx");
                
                if main_ctx.exists() {
                    Ok(main_ctx)
                } else if index_ctx.exists() {
                    Ok(index_ctx)
                } else {
                    Err(anyhow::anyhow!("No main.ctx or index.ctx found in directory"))
                }
            } else {
                Err(anyhow::anyhow!("Path does not exist: {}", p.display()))
            }
        }
        None => {
            // Look in current directory
            let current_dir = std::env::current_dir()?;
            find_cortex_file(Some(current_dir))
        }
    }
}