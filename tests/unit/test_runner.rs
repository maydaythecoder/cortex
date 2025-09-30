//! Unit test runner for Cortex

use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct TestRunner {
    test_dir: String,
    verbose: bool,
}

impl TestRunner {
    pub fn new(test_dir: String, verbose: bool) -> Self {
        Self { test_dir, verbose }
    }
    
    pub fn run_all_tests(&self) -> Result<TestResults> {
        let mut results = TestResults::new();
        
        // Find all test files
        let test_files = self.find_test_files()?;
        
        for test_file in test_files {
            if self.verbose {
                println!("Running test: {}", test_file.display());
            }
            
            let result = self.run_test_file(&test_file)?;
            results.add_result(result);
        }
        
        Ok(results)
    }
    
    fn find_test_files(&self) -> Result<Vec<std::path::PathBuf>> {
        let mut test_files = Vec::new();
        
        if Path::new(&self.test_dir).exists() {
            for entry in fs::read_dir(&self.test_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "ctx") {
                    if path.file_name().unwrap().to_string_lossy().starts_with("test_") {
                        test_files.push(path);
                    }
                }
            }
        }
        
        Ok(test_files)
    }
    
    fn run_test_file(&self, test_file: &Path) -> Result<TestResult> {
        let test_name = test_file.file_stem().unwrap().to_string_lossy().to_string();
        let start_time = std::time::Instant::now();
        
        // Read test file
        let test_content = fs::read_to_string(test_file)?;
        
        // Parse test content
        let test_cases = self.parse_test_cases(&test_content)?;
        
        let mut test_result = TestResult {
            name: test_name,
            file: test_file.to_path_buf(),
            cases: Vec::new(),
            duration: start_time.elapsed(),
            passed: 0,
            failed: 0,
        };
        
        // Run each test case
        for test_case in test_cases {
            let case_result = self.run_test_case(&test_case)?;
            if case_result.passed {
                test_result.passed += 1;
            } else {
                test_result.failed += 1;
            }
            test_result.cases.push(case_result);
        }
        
        Ok(test_result)
    }
    
    fn parse_test_cases(&self, content: &str) -> Result<Vec<TestCase>> {
        let mut test_cases = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.starts_with("// TEST:") {
                let test_name = line[8..].trim().to_string();
                let mut test_code = String::new();
                let mut expected_output = String::new();
                let mut in_expected = false;
                
                i += 1;
                while i < lines.len() {
                    let current_line = lines[i];
                    
                    if current_line.trim() == "// EXPECTED:" {
                        in_expected = true;
                    } else if current_line.trim().starts_with("// TEST:") {
                        i -= 1; // Back up to process this test
                        break;
                    } else if in_expected {
                        expected_output.push_str(current_line);
                        expected_output.push('\n');
                    } else {
                        test_code.push_str(current_line);
                        test_code.push('\n');
                    }
                    
                    i += 1;
                }
                
                test_cases.push(TestCase {
                    name: test_name,
                    code: test_code.trim().to_string(),
                    expected_output: expected_output.trim().to_string(),
                });
            }
            
            i += 1;
        }
        
        Ok(test_cases)
    }
    
    fn run_test_case(&self, test_case: &TestCase) -> Result<TestCaseResult> {
        // This would integrate with the Cortex compiler/interpreter
        // For now, we'll simulate the test execution
        
        let start_time = std::time::Instant::now();
        
        // Simulate running the test code
        let actual_output = self.execute_test_code(&test_case.code)?;
        
        let passed = actual_output.trim() == test_case.expected_output.trim();
        
        Ok(TestCaseResult {
            name: test_case.name.clone(),
            passed,
            expected_output: test_case.expected_output.clone(),
            actual_output,
            duration: start_time.elapsed(),
            error: if passed { None } else { Some("Output mismatch".to_string()) },
        })
    }
    
    fn execute_test_code(&self, code: &str) -> Result<String> {
        // This would use the actual Cortex interpreter
        // For now, we'll return a placeholder
        Ok("Test output placeholder".to_string())
    }
}

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub code: String,
    pub expected_output: String,
}

#[derive(Debug)]
pub struct TestCaseResult {
    pub name: String,
    pub passed: bool,
    pub expected_output: String,
    pub actual_output: String,
    pub duration: std::time::Duration,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub file: std::path::PathBuf,
    pub cases: Vec<TestCaseResult>,
    pub duration: std::time::Duration,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug)]
pub struct TestResults {
    pub results: Vec<TestResult>,
    pub total_passed: usize,
    pub total_failed: usize,
    pub total_duration: std::time::Duration,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            total_passed: 0,
            total_failed: 0,
            total_duration: std::time::Duration::new(0, 0),
        }
    }
    
    pub fn add_result(&mut self, result: TestResult) {
        self.total_passed += result.passed;
        self.total_failed += result.failed;
        self.total_duration += result.duration;
        self.results.push(result);
    }
    
    pub fn print_summary(&self) {
        println!("\n=== Test Results ===");
        println!("Total tests: {}", self.total_passed + self.total_failed);
        println!("Passed: {}", self.total_passed);
        println!("Failed: {}", self.total_failed);
        println!("Duration: {:?}", self.total_duration);
        
        if self.total_failed > 0 {
            println!("\n=== Failed Tests ===");
            for result in &self.results {
                if result.failed > 0 {
                    println!("{} ({} failures)", result.name, result.failed);
                    for case in &result.cases {
                        if !case.passed {
                            println!("  - {}: {}", case.name, case.error.as_deref().unwrap_or("Unknown error"));
                        }
                    }
                }
            }
        }
    }
}