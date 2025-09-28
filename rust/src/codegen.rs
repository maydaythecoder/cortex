//! Cortex Interpreter
//! 
//! Interprets Cortex AST directly without LLVM compilation.

use crate::ast::*;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<()> {
        // Interpret the program
        self.interpret_program(program)?;
        Ok(())
    }
    
    fn interpret_program(&mut self, program: &Program) -> Result<()> {
        // Interpret each statement
        for statement in &program.statements {
            self.interpret_statement(statement)?;
        }
        Ok(())
    }
    
    fn interpret_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Block(block) => {
                for stmt in &block.statements {
                    self.interpret_statement(stmt)?;
                }
            }
            Statement::Function(function) => {
                self.interpret_function(function)?;
            }
            Statement::IfStatement(if_stmt) => {
                self.interpret_if_statement(if_stmt)?;
            }
            Statement::WhileLoop(while_loop) => {
                self.interpret_while_loop(while_loop)?;
            }
            Statement::ForLoop(for_loop) => {
                self.interpret_for_loop(for_loop)?;
            }
            Statement::ReturnStatement(return_stmt) => {
                self.interpret_return_statement(return_stmt)?;
            }
            Statement::Assignment(assignment) => {
                self.interpret_assignment(assignment)?;
            }
            Statement::ConstantAssignment(constant_assignment) => {
                self.interpret_constant_assignment(constant_assignment)?;
            }
            Statement::Expression(expr) => {
                self.interpret_expression(expr)?;
            }
        }
        Ok(())
    }
    
    fn interpret_function(&mut self, function: &Function) -> Result<()> {
        // Store function definition
        self.functions.insert(function.name.clone(), function.clone());
        Ok(())
    }
    
    fn interpret_if_statement(&mut self, if_stmt: &IfStatement) -> Result<()> {
        let condition = self.interpret_expression(&if_stmt.condition)?;
        
        if self.is_truthy(&condition) {
            self.interpret_block(&if_stmt.then_block)?;
        } else if let Some(else_block) = &if_stmt.else_block {
            self.interpret_block(else_block)?;
        }
        
        Ok(())
    }
    
    fn interpret_while_loop(&mut self, while_loop: &WhileLoop) -> Result<()> {
        loop {
            let condition = self.interpret_expression(&while_loop.condition)?;
            if !self.is_truthy(&condition) {
                break;
            }
            self.interpret_block(&while_loop.body)?;
        }
        Ok(())
    }
    
    fn interpret_for_loop(&mut self, for_loop: &ForLoop) -> Result<()> {
        // For now, implement as a simple infinite loop with break
        // TODO: Implement proper for loop semantics
        loop {
            self.interpret_block(&for_loop.body)?;
            break; // Simple implementation - just run once
        }
        Ok(())
    }
    
    fn interpret_return_statement(&mut self, return_stmt: &ReturnStatement) -> Result<()> {
        if let Some(value) = &return_stmt.value {
            let _result = self.interpret_expression(value)?;
            // TODO: Handle return value properly
        }
        Ok(())
    }
    
    fn interpret_assignment(&mut self, assignment: &Assignment) -> Result<()> {
        let value = self.interpret_expression(&assignment.value)?;
        self.variables.insert(assignment.variable.clone(), value);
        Ok(())
    }
    
    fn interpret_constant_assignment(&mut self, constant_assignment: &ConstantAssignment) -> Result<()> {
        let value = self.interpret_expression(&constant_assignment.value)?;
        self.variables.insert(constant_assignment.variable.clone(), value);
        Ok(())
    }
    
    fn interpret_expression(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Literal(literal) => {
                match &literal.value {
                    LiteralValue::Number(n) => Ok(Value::Number(*n)),
                    LiteralValue::String(s) => Ok(Value::String(s.clone())),
                    LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
                    LiteralValue::Null => Ok(Value::Null),
                }
            }
            Expression::Identifier(identifier) => {
                if let Some(value) = self.variables.get(&identifier.name) {
                    Ok(value.clone())
                } else {
                    Err(anyhow::anyhow!("Undefined variable: {}", identifier.name))
                }
            }
            Expression::BinaryOp(binary_op) => {
                let left = self.interpret_expression(&binary_op.left)?;
                let right = self.interpret_expression(&binary_op.right)?;
                
                match binary_op.operator.as_str() {
                    "+" => self.add_values(&left, &right),
                    "-" => self.subtract_values(&left, &right),
                    "*" => self.multiply_values(&left, &right),
                    "/" => self.divide_values(&left, &right),
                    "%" => self.modulo_values(&left, &right),
                    "**" => self.power_values(&left, &right),
                    "==" => Ok(Value::Boolean(self.values_equal(&left, &right))),
                    "!=" => Ok(Value::Boolean(!self.values_equal(&left, &right))),
                    "<" => self.compare_values(&left, &right, |a, b| a < b),
                    "<=" => self.compare_values(&left, &right, |a, b| a <= b),
                    ">" => self.compare_values(&left, &right, |a, b| a > b),
                    ">=" => self.compare_values(&left, &right, |a, b| a >= b),
                    "&&" => Ok(Value::Boolean(self.is_truthy(&left) && self.is_truthy(&right))),
                    "||" => Ok(Value::Boolean(self.is_truthy(&left) || self.is_truthy(&right))),
                    _ => Err(anyhow::anyhow!("Unsupported binary operator: {}", binary_op.operator))
                }
            }
            Expression::UnaryOp(unary_op) => {
                let operand = self.interpret_expression(&unary_op.operand)?;
                
                match unary_op.operator.as_str() {
                    "!" => Ok(Value::Boolean(!self.is_truthy(&operand))),
                    "-" => {
                        if let Value::Number(n) = operand {
                            Ok(Value::Number(-n))
                        } else {
                            Err(anyhow::anyhow!("Cannot negate non-number value"))
                        }
                    }
                    _ => Err(anyhow::anyhow!("Unsupported unary operator: {}", unary_op.operator))
                }
            }
            Expression::Call(call) => {
                if let Expression::Identifier(func_name) = &*call.function {
                    if func_name.name == "print" {
                        // Handle print function
                        self.interpret_print_call(&call.arguments)?;
                        Ok(Value::Null)
                    } else if func_name.name == "str" {
                        // Handle str function
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("str() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        Ok(Value::String(value.to_string()))
                    } else if let Some(function) = self.functions.get(&func_name.name) {
                        // Implement function calls with parameters
                        let function = function.clone();
                        let result = self.interpret_function_call(&function, &call.arguments)?;
                        Ok(result)
                    } else {
                        Err(anyhow::anyhow!("Undefined function: {}", func_name.name))
                    }
                } else {
                    Err(anyhow::anyhow!("Invalid function call"))
                }
            }
            Expression::Array(_) => {
                // TODO: Implement array support
                Err(anyhow::anyhow!("Array literals not yet implemented"))
            }
            Expression::Dictionary(_) => {
                // TODO: Implement dictionary support
                Err(anyhow::anyhow!("Dictionary literals not yet implemented"))
            }
        }
    }
    
    fn interpret_block(&mut self, block: &Block) -> Result<Value> {
        let mut result = Value::Null;
        for statement in &block.statements {
            match statement {
                Statement::ReturnStatement(return_stmt) => {
                    if let Some(expr) = &return_stmt.value {
                        result = self.interpret_expression(expr)?;
                    }
                    break; // Return immediately
                }
                _ => {
                    self.interpret_statement(statement)?;
                }
            }
        }
        Ok(result)
    }
    
    fn interpret_function_call(&mut self, function: &Function, arguments: &[Expression]) -> Result<Value> {
        // Check argument count
        if arguments.len() != function.parameters.len() {
            return Err(anyhow::anyhow!(
                "Function {} expects {} arguments, got {}",
                function.name,
                function.parameters.len(),
                arguments.len()
            ));
        }
        
        // Save current variables
        let old_variables = self.variables.clone();
        
        // Bind parameters to arguments
        for (param, arg) in function.parameters.iter().zip(arguments.iter()) {
            let value = self.interpret_expression(arg)?;
            self.variables.insert(param.name.clone(), value);
        }
        
        // Execute function body
        let result = self.interpret_block(&function.body)?;
        
        // Restore variables
        self.variables = old_variables;
        
        Ok(result)
    }

    fn interpret_print_call(&mut self, args: &[Expression]) -> Result<()> {
        for (i, arg) in args.iter().enumerate() {
            let value = self.interpret_expression(arg)?;
            if i > 0 {
                print!(" ");
            }
            print!("{}", value);
        }
        println!();
        Ok(())
    }
    
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Number(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Number(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(anyhow::anyhow!("Cannot add {:?} and {:?}", left, right))
        }
    }
    
    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(anyhow::anyhow!("Cannot subtract {:?} and {:?}", left, right))
        }
    }
    
    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(anyhow::anyhow!("Cannot multiply {:?} and {:?}", left, right))
        }
    }
    
    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err(anyhow::anyhow!("Division by zero"))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(anyhow::anyhow!("Cannot divide {:?} and {:?}", left, right))
        }
    }
    
    fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err(anyhow::anyhow!("Modulo by zero"))
                } else {
                    Ok(Value::Number(a % b))
                }
            }
            _ => Err(anyhow::anyhow!("Cannot modulo {:?} and {:?}", left, right))
        }
    }
    
    fn power_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
            _ => Err(anyhow::anyhow!("Cannot raise {:?} to power {:?}", left, right))
        }
    }
    
    fn compare_values<F>(&self, left: &Value, right: &Value, op: F) -> Result<Value>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(op(*a, *b))),
            _ => Err(anyhow::anyhow!("Cannot compare {:?} and {:?}", left, right))
        }
    }
    
    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Boolean(b) => *b,
            Value::Null => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    
    #[test]
    fn test_simple_interpretation() {
        let source = "let x := 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&ast);
        
        assert!(result.is_ok());
    }
}