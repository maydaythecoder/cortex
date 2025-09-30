//! Debugger for Cortex programs
//! 
//! Provides debugging capabilities including breakpoints, step execution,
//! variable inspection, and call stack tracing.

use crate::ast::*;
use crate::codegen::{Interpreter, Value};
use crate::error::{CompilerError, DetailedError, ErrorContext, Result};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub line: usize,
    pub condition: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct DebugState {
    pub current_line: usize,
    pub current_function: Option<String>,
    pub call_stack: Vec<String>,
    pub variables: HashMap<String, Value>,
    pub breakpoints: Vec<Breakpoint>,
    pub is_stepping: bool,
    pub is_paused: bool,
}

pub struct Debugger {
    interpreter: Interpreter,
    state: DebugState,
    source_lines: Vec<String>,
}

impl Debugger {
    pub fn new(source: &str) -> Self {
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        
        Self {
            interpreter: Interpreter::new(),
            state: DebugState {
                current_line: 0,
                current_function: None,
                call_stack: Vec::new(),
                variables: HashMap::new(),
                breakpoints: Vec::new(),
                is_stepping: false,
                is_paused: false,
            },
            source_lines,
        }
    }
    
    pub fn add_breakpoint(&mut self, line: usize, condition: Option<String>) {
        self.state.breakpoints.push(Breakpoint {
            line,
            condition,
            enabled: true,
        });
    }
    
    pub fn remove_breakpoint(&mut self, line: usize) {
        self.state.breakpoints.retain(|bp| bp.line != line);
    }
    
    pub fn enable_breakpoint(&mut self, line: usize) {
        if let Some(bp) = self.state.breakpoints.iter_mut().find(|bp| bp.line == line) {
            bp.enabled = true;
        }
    }
    
    pub fn disable_breakpoint(&mut self, line: usize) {
        if let Some(bp) = self.state.breakpoints.iter_mut().find(|bp| bp.line == line) {
            bp.enabled = false;
        }
    }
    
    pub fn list_breakpoints(&self) {
        println!("Breakpoints:");
        for (i, bp) in self.state.breakpoints.iter().enumerate() {
            let status = if bp.enabled { "enabled" } else { "disabled" };
            let condition = bp.condition.as_ref()
                .map(|c| format!(" (condition: {})", c))
                .unwrap_or_default();
            println!("  {}: line {} [{}]{}", i + 1, bp.line, status, condition);
        }
    }
    
    pub fn step_into(&mut self, program: &Program) -> Result<()> {
        self.state.is_stepping = true;
        self.state.is_paused = false;
        self.execute_with_debugging(program)
    }
    
    pub fn step_over(&mut self, program: &Program) -> Result<()> {
        // For now, same as step_into
        self.step_into(program)
    }
    
    pub fn continue_execution(&mut self, program: &Program) -> Result<()> {
        self.state.is_stepping = false;
        self.state.is_paused = false;
        self.execute_with_debugging(program)
    }
    
    pub fn pause_execution(&mut self) {
        self.state.is_paused = true;
    }
    
    pub fn inspect_variable(&self, name: &str) -> Option<&Value> {
        self.state.variables.get(name)
    }
    
    pub fn list_variables(&self) {
        println!("Variables:");
        for (name, value) in &self.state.variables {
            println!("  {} = {}", name, value);
        }
    }
    
    pub fn show_call_stack(&self) {
        println!("Call stack:");
        for (i, function) in self.state.call_stack.iter().enumerate() {
            let marker = if i == self.state.call_stack.len() - 1 { "->" } else { "  " };
            println!("  {} {}", marker, function);
        }
    }
    
    pub fn show_current_line(&self) {
        if self.state.current_line > 0 && self.state.current_line <= self.source_lines.len() {
            let line_num = self.state.current_line;
            let line_content = &self.source_lines[line_num - 1];
            println!("  {} | {}", line_num, line_content);
        }
    }
    
    fn execute_with_debugging(&mut self, program: &Program) -> Result<()> {
        // This is a simplified version - in a real implementation,
        // you'd need to modify the interpreter to support debugging
        self.interpreter.interpret(program)
            .map_err(|e| DetailedError {
                error: CompilerError::RuntimeError {
                    message: e.to_string(),
                    line: self.state.current_line,
                    column: 0,
                },
                context: ErrorContext {
                    file_path: None,
                    line: self.state.current_line,
                    column: 0,
                    source_line: self.source_lines.get(self.state.current_line.saturating_sub(1)).cloned(),
                    suggestion: None,
                },
            })
    }
    
    pub fn interactive_debug(&mut self, program: &Program) -> Result<()> {
        println!("Cortex Debugger - Interactive Mode");
        println!("Commands: step, continue, break <line>, list, vars, stack, quit");
        
        loop {
            print!("(cortex-debug) ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let command = input.trim();
            
            match command {
                "step" | "s" => {
                    self.step_into(program)?;
                    self.show_current_line();
                }
                "continue" | "c" => {
                    self.continue_execution(program)?;
                    break;
                }
                "list" | "l" => {
                    self.list_breakpoints();
                }
                "vars" | "v" => {
                    self.list_variables();
                }
                "stack" => {
                    self.show_call_stack();
                }
                "quit" | "q" => {
                    break;
                }
                cmd if cmd.starts_with("break ") => {
                    if let Ok(line) = cmd[6..].parse::<usize>() {
                        self.add_breakpoint(line, None);
                        println!("Breakpoint set at line {}", line);
                    } else {
                        println!("Invalid line number");
                    }
                }
                _ => {
                    println!("Unknown command: {}", command);
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    
    #[test]
    fn test_debugger_creation() {
        let source = "let x := 42\nprint[x]";
        let debugger = Debugger::new(source);
        assert_eq!(debugger.source_lines.len(), 2);
    }
    
    #[test]
    fn test_breakpoint_management() {
        let source = "let x := 42";
        let mut debugger = Debugger::new(source);
        
        debugger.add_breakpoint(1, None);
        assert_eq!(debugger.state.breakpoints.len(), 1);
        
        debugger.remove_breakpoint(1);
        assert_eq!(debugger.state.breakpoints.len(), 0);
    }
}