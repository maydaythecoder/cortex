//! Cortex CLI Tool
//! 
//! Comprehensive command-line interface for the Cortex programming language.
//! Provides project management, package management, and development tools.

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

mod commands;
mod config;
mod project;
mod package;

use commands::*;

#[derive(Parser)]
#[command(name = "cortex")]
#[command(about = "Cortex CLI - Command-line interface for Cortex programming language")]
#[command(version)]
#[command(long_about = r#"
Cortex CLI provides comprehensive tools for developing with the Cortex programming language.

Features:
- Project initialization and management
- Package management and dependencies
- Code compilation and execution
- Testing and debugging
- Code formatting and linting
- Documentation generation

Examples:
  cortex new my-project          # Create a new Cortex project
  cortex run main.ctx            # Run a Cortex program
  cortex build --release         # Build optimized executable
  cortex test                    # Run project tests
  cortex fmt                     # Format code
  cortex add tensor              # Add tensor package
"#)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Cortex project
    New {
        /// Project name
        name: String,
        
        /// Project template to use
        #[arg(short, long, default_value = "basic")]
        template: String,
        
        /// Initialize in current directory
        #[arg(short, long)]
        current_dir: bool,
    },
    
    /// Run a Cortex program
    Run {
        /// Input file or project
        input: Option<PathBuf>,
        
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
        
        /// Additional arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// Build a Cortex project
    Build {
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Build in release mode
        #[arg(long)]
        release: bool,
        
        /// Optimization level (0-3)
        #[arg(short, long, default_value = "2")]
        opt_level: u8,
        
        /// Enable debug information
        #[arg(long)]
        debug_info: bool,
    },
    
    /// Test a Cortex project
    Test {
        /// Test file pattern
        pattern: Option<String>,
        
        /// Run tests in verbose mode
        #[arg(short, long)]
        verbose: bool,
        
        /// Run only tests matching this pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    
    /// Format Cortex code
    Format {
        /// Files or directories to format
        files: Vec<PathBuf>,
        
        /// Check formatting without making changes
        #[arg(long)]
        check: bool,
        
        /// Number of spaces for indentation
        #[arg(short, long, default_value = "2")]
        indent: usize,
    },
    
    /// Lint Cortex code
    Lint {
        /// Files or directories to lint
        files: Vec<PathBuf>,
        
        /// Fix auto-fixable issues
        #[arg(long)]
        fix: bool,
    },
    
    /// Debug a Cortex program
    Debug {
        /// Program to debug
        program: PathBuf,
        
        /// Breakpoint at line number
        #[arg(short, long)]
        breakpoint: Vec<usize>,
        
        /// Start in interactive mode
        #[arg(short, long)]
        interactive: bool,
    },
    
    /// Package management
    #[command(subcommand)]
    Package(PackageCommands),
    
    /// Project management
    #[command(subcommand)]
    Project(ProjectCommands),
    
    /// Documentation generation
    Doc {
        /// Output directory
        #[arg(short, long, default_value = "docs")]
        output: PathBuf,
        
        /// Open documentation in browser
        #[arg(long)]
        open: bool,
    },
    
    /// Clean build artifacts
    Clean {
        /// Clean all artifacts including dependencies
        #[arg(long)]
        all: bool,
    },
    
    /// Show version information
    Version {
        /// Show detailed version information
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum PackageCommands {
    /// Add a package dependency
    Add {
        /// Package name
        name: String,
        
        /// Package version
        #[arg(short, long)]
        version: Option<String>,
        
        /// Add as development dependency
        #[arg(long)]
        dev: bool,
    },
    
    /// Remove a package dependency
    Remove {
        /// Package name
        name: String,
    },
    
    /// List installed packages
    List {
        /// Show outdated packages
        #[arg(long)]
        outdated: bool,
    },
    
    /// Update packages
    Update {
        /// Package name (if not specified, update all)
        name: Option<String>,
    },
    
    /// Install packages from lockfile
    Install,
    
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Show project information
    Info,
    
    /// Initialize project configuration
    Init {
        /// Force reinitialization
        #[arg(long)]
        force: bool,
    },
    
    /// Build project dependencies
    Build,
    
    /// Clean project artifacts
    Clean {
        /// Clean all artifacts including dependencies
        #[arg(long)]
        all: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Set up logging level
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    
    match cli.command {
        Commands::New { name, template, current_dir } => {
            commands::new::create_project(name, template, current_dir).await
        }
        Commands::Run { input, debug, args } => {
            commands::run::run_program(input, debug, args).await
        }
        Commands::Build { output, release, opt_level, debug_info } => {
            commands::build::build_project(output, release, opt_level, debug_info).await
        }
        Commands::Test { pattern, verbose, filter } => {
            commands::test::run_tests(pattern, verbose, filter).await
        }
        Commands::Format { files, check, indent } => {
            commands::format::format_code(files, check, indent).await
        }
        Commands::Lint { files, fix } => {
            commands::lint::lint_code(files, fix).await
        }
        Commands::Debug { program, breakpoint, interactive } => {
            commands::debug::debug_program(program, breakpoint, interactive).await
        }
        Commands::Package(cmd) => {
            commands::package::handle_package_command(cmd).await
        }
        Commands::Project(cmd) => {
            commands::project::handle_project_command(cmd).await
        }
        Commands::Doc { output, open } => {
            commands::doc::generate_docs(output, open).await
        }
        Commands::Clean { all } => {
            commands::clean::clean_artifacts(all).await
        }
        Commands::Version { verbose } => {
            commands::version::show_version(verbose).await
        }
    }
}