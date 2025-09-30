//! Project creation commands

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::commands::{create_progress_bar};

pub async fn create_project(name: String, template: String, current_dir: bool) -> Result<()> {
    let pb = create_progress_bar("Creating Cortex project...");
    
    let project_path = if current_dir {
        std::env::current_dir()?
    } else {
        std::env::current_dir()?.join(&name)
    };
    
    // Create project directory if not using current directory
    if !current_dir {
        fs::create_dir_all(&project_path)?;
    }
    
    // Create project structure
    create_project_structure(&project_path, &template)?;
    
    // Create cortex.toml configuration
    create_project_config(&project_path, &name)?;
    
    // Create initial source files
    create_initial_files(&project_path, &template)?;
    
    pb.finish_with_message("✅ Project created successfully!");
    
    println!("\n🎉 Cortex project '{}' created successfully!", name);
    println!("\nNext steps:");
    println!("  cd {}", project_path.display());
    println!("  cortex run main.ctx");
    println!("  cortex test");
    
    Ok(())
}

fn create_project_structure(project_path: &PathBuf, template: &str) -> Result<()> {
    let dirs = match template {
        "basic" => vec!["src", "tests", "docs"],
        "ml" => vec!["src", "tests", "docs", "data", "models"],
        "web" => vec!["src", "tests", "docs", "static", "templates"],
        "library" => vec!["src", "tests", "docs", "examples"],
        _ => vec!["src", "tests", "docs"],
    };
    
    for dir in dirs {
        fs::create_dir_all(project_path.join(dir))?;
    }
    
    Ok(())
}

fn create_project_config(project_path: &PathBuf, name: &str) -> Result<()> {
    let config_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
description = "A Cortex project"
authors = ["Cortex Developer"]
license = "MIT"

[dependencies]
# Add your dependencies here

[dev-dependencies]
# Add your development dependencies here

[build]
target = "native"
optimization = "balanced"

[format]
indent_size = 2
max_line_length = 100

[lint]
strict = false
warnings_as_errors = false
"#, name);
    
    fs::write(project_path.join("cortex.toml"), config_content)?;
    Ok(())
}

fn create_initial_files(project_path: &PathBuf, template: &str) -> Result<()> {
    let main_content = match template {
        "basic" => r#"// Main entry point for Cortex program

func main[] |
  print["Hello, Cortex!"]
  let message := "Welcome to Cortex programming"
  print[message]
^

main[]"#,
        "ml" => r#"// Machine Learning example with Cortex

func main[] |
  print["Cortex ML Example"]
  
  // Simple linear regression example
  let x := [1, 2, 3, 4, 5]
  let y := [2, 4, 6, 8, 10]
  
  let slope := calculate_slope[x, y]
  let intercept := calculate_intercept[x, y, slope]
  
  print["Linear regression results:"]
  print["Slope: " + str[slope]]
  print["Intercept: " + str[intercept]]
^

func calculate_slope[x, y] |
  let n := len[x]
  let sum_x := sum[x]
  let sum_y := sum[y]
  let sum_xy := sum_products[x, y]
  let sum_x2 := sum_squares[x]
  
  let numerator := n * sum_xy - sum_x * sum_y
  let denominator := n * sum_x2 - sum_x * sum_x
  
  return[numerator / denominator]
^

func calculate_intercept[x, y, slope] |
  let n := len[x]
  let sum_x := sum[x]
  let sum_y := sum[y]
  
  return[(sum_y - slope * sum_x) / n]
^

func sum[arr] |
  let total := 0
  for [i] |
    let total := total + arr@i
  ^
  return[total]
^

func sum_products[x, y] |
  let total := 0
  for [i] |
    let total := total + x@i * y@i
  ^
  return[total]
^

func sum_squares[arr] |
  let total := 0
  for [i] |
    let total := total + arr@i * arr@i
  ^
  return[total]
^

main[]"#,
        "web" => r#"// Web application example with Cortex

func main[] |
  print["Cortex Web Example"]
  
  // Simple web server simulation
  let server := create_server[]
  let routes := create_routes[]
  
  print["Server started on http://localhost:3000"]
  print["Available routes:"]
  for [route in routes] |
    print["  " + route]
  ^
^

func create_server[] |
  return["Cortex Web Server"]
^

func create_routes[] |
  return["/", "/api", "/docs", "/health"]
^

main[]"#,
        "library" => r#"// Library example with Cortex

func main[] |
  print["Cortex Library Example"]
  
  // Demonstrate library functions
  let numbers := [1, 2, 3, 4, 5]
  let doubled := map[numbers, double]
  let sum := reduce[doubled, add]
  
  print["Original: " + str[numbers]]
  print["Doubled: " + str[doubled]]
  print["Sum: " + str[sum]]
^

func double[x] |
  return[x * 2]
^

func add[a, b] |
  return[a + b]
^

func map[arr, func] |
  let result := []
  for [i] |
    let result := result + [func[arr@i]]
  ^
  return[result]
^

func reduce[arr, func] |
  if len[arr] == 0 |
    return[0]
  ^
  
  let result := arr@0
  for [i in range[1, len[arr]]] |
    let result := func[result, arr@i]
  ^
  return[result]
^

func range[start, end] |
  let result := []
  let i := start
  while [i < end] |
    let result := result + [i]
    let i := i + 1
  ^
  return[result]
^

main[]"#,
        _ => r#"// Cortex program

func main[] |
  print["Hello, Cortex!"]
^

main[]"#,
    };
    
    fs::write(project_path.join("src").join("main.ctx"), main_content)?;
    
    // Create README
    let readme_content = format!(r#"# {}

A Cortex programming language project.

## Getting Started

1. Run the program:
   ```bash
   cortex run src/main.ctx
   ```

2. Run tests:
   ```bash
   cortex test
   ```

3. Build the project:
   ```bash
   cortex build
   ```

## Project Structure

- `src/` - Source code
- `tests/` - Test files
- `docs/` - Documentation
- `cortex.toml` - Project configuration

## Development

- Format code: `cortex fmt`
- Lint code: `cortex lint`
- Debug: `cortex debug src/main.ctx`
"#, name);
    
    fs::write(project_path.join("README.md"), readme_content)?;
    
    // Create .gitignore
    let gitignore_content = r#"# Cortex build artifacts
target/
*.exe
*.dll
*.so
*.dylib

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Logs
*.log

# Dependencies
node_modules/
"#;
    
    fs::write(project_path.join(".gitignore"), gitignore_content)?;
    
    Ok(())
}