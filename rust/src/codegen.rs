//! Cortex Interpreter
//! 
//! Interprets Cortex AST directly without LLVM compilation.

use crate::ast::*;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Dictionary(HashMap<String, Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Dictionary(dict) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in dict.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, val)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    current_return_value: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_return_value: None,
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
                    // If we hit a return statement, stop processing the block
                    if self.current_return_value.is_some() {
                        break;
                    }
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
                let result = self.interpret_return_statement(return_stmt)?;
                self.current_return_value = Some(result);
                return Ok(());  // Exit current function scope
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
        // Implement for loop with iterable support
        if let Some(iterable) = &for_loop.iterable {
            // Iterate over array or other iterable
            let iterable_value = self.interpret_expression(iterable)?;
            
            match iterable_value {
                Value::Array(arr) => {
                    for (index, element) in arr.iter().enumerate() {
                        // Set both index and element variables
                        self.variables.insert(format!("{}_index", for_loop.variable), Value::Number(index as f64));
                        self.variables.insert(for_loop.variable.clone(), element.clone());
                        
                        // Interpret the loop body
                        self.interpret_block(&for_loop.body)?;
                    }
                }
                Value::Number(n) => {
                    // Range loop: for [i] | ... ^ with implicit 0 to n range
                    for i in 0..(n as usize) {
                        self.variables.insert(for_loop.variable.clone(), Value::Number(i as f64));
                        self.interpret_block(&for_loop.body)?;
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Cannot iterate over non-array value"));
                }
            }
        } else {
            // Simple counter loop - iterate 10 times by default
            for i in 0..10 {
                self.variables.insert(for_loop.variable.clone(), Value::Number(i as f64));
                self.interpret_block(&for_loop.body)?;
            }
        }
        
        Ok(())
    }
    
    fn interpret_return_statement(&mut self, return_stmt: &ReturnStatement) -> Result<Value> {
        if let Some(value) = &return_stmt.value {
            let result = self.interpret_expression(value)?;
            return Ok(result);
        }
        Ok(Value::Null)
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
                    "@" => self.access_element(&left, &right),
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
                    // Built-in math functions
                    if func_name.name == "sqrt" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("sqrt() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Number(n) => Ok(Value::Number(n.sqrt())),
                            _ => Err(anyhow::anyhow!("sqrt() requires a number argument"))
                        }
                    } else if func_name.name == "exp" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("exp() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Number(n) => Ok(Value::Number(n.exp())),
                            _ => Err(anyhow::anyhow!("exp() requires a number argument"))
                        }
                    } else if func_name.name == "ln" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("ln() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Number(n) => {
                                if n <= 0.0 {
                                    return Err(anyhow::anyhow!("ln() requires a positive number"));
                                }
                                Ok(Value::Number(n.ln()))
                            },
                            _ => Err(anyhow::anyhow!("ln() requires a number argument"))
                        }
                    } else if func_name.name == "sin" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("sin() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Number(n) => Ok(Value::Number(n.sin())),
                            _ => Err(anyhow::anyhow!("sin() requires a number argument"))
                        }
                    } else if func_name.name == "cos" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("cos() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Number(n) => Ok(Value::Number(n.cos())),
                            _ => Err(anyhow::anyhow!("cos() requires a number argument"))
                        }
                    } else if func_name.name == "abs" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("abs() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Number(n) => Ok(Value::Number(n.abs())),
                            _ => Err(anyhow::anyhow!("abs() requires a number argument"))
                        }
                    } else if func_name.name == "min" {
                        if call.arguments.len() != 2 {
                            return Err(anyhow::anyhow!("min() takes exactly two arguments"));
                        }
                        let val1 = self.interpret_expression(&call.arguments[0])?;
                        let val2 = self.interpret_expression(&call.arguments[1])?;
                        match (&val1, &val2) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1.min(*n2))),
                            _ => Err(anyhow::anyhow!("min() requires two number arguments"))
                        }
                    } else if func_name.name == "max" {
                        if call.arguments.len() != 2 {
                            return Err(anyhow::anyhow!("max() takes exactly two arguments"));
                        }
                        let val1 = self.interpret_expression(&call.arguments[0])?;
                        let val2 = self.interpret_expression(&call.arguments[1])?;
                        match (&val1, &val2) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1.max(*n2))),
                            _ => Err(anyhow::anyhow!("max() requires two number arguments"))
                        }
                    } else if func_name.name == "len" {
                        if call.arguments.len() != 1 {
                            return Err(anyhow::anyhow!("len() takes exactly one argument"));
                        }
                        let value = self.interpret_expression(&call.arguments[0])?;
                        match value {
                            Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
                            Value::Dictionary(dict) => Ok(Value::Number(dict.len() as f64)),
                            Value::String(s) => Ok(Value::Number(s.len() as f64)),
                            _ => Err(anyhow::anyhow!("len() requires an array, dictionary, or string"))
                        }
                    } else if func_name.name == "print" {
                        // Handle print function
                        if !call.arguments.is_empty() {
                            let mut output = String::new();
                            for (i, arg) in call.arguments.iter().enumerate() {
                                if i > 0 {
                                    output.push(' ');
                                }
                                let value = self.interpret_expression(arg)?;
                                output.push_str(&format!("{}", value));
                            }
                            println!("{}", output);
                        }
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
            Expression::Array(array) => {
                // Implement array support
                let mut values = Vec::new();
                for element in &array.elements {
                    let value = self.interpret_expression(element)?;
                    values.push(value);
                }
                Ok(Value::Array(values))
            }
            Expression::Dictionary(dict) => {
                // Implement dictionary support
                let mut map = std::collections::HashMap::new();
                for (key, value) in &dict.pairs {
                    let key_value = self.interpret_expression(key)?;
                    let value_value = self.interpret_expression(value)?;
                    
                    // Convert key to string for HashMap
                    let key_str = match key_value {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        Value::Array(_) => "array".to_string(), // Simplified approach
                        Value::Dictionary(_) => "dict".to_string(), // Simplified approach
                    };
                    
                    map.insert(key_str, value_value);
                }
                Ok(Value::Dictionary(map))
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
                    } else {
                        result = Value::Null;
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
        
        // Return the result from the block
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
            Value::Array(arr) => !arr.is_empty(),
            Value::Dictionary(dict) => !dict.is_empty(),
        }
    }
    
    fn access_element(&self, container: &Value, index: &Value) -> Result<Value> {
        match container {
            Value::Array(arr) => {
                match index {
                    Value::Number(n) => {
                        let idx = *n as usize;
                        if idx < arr.len() {
                            Ok(arr[idx].clone())
                        } else {
                            Err(anyhow::anyhow!("Array index {} out of bounds (length: {})", idx, arr.len()))
                        }
                    }
                    _ => Err(anyhow::anyhow!("Array index must be a number, got: {:?}", index))
                }
            }
            Value::Dictionary(map) => {
                match index {
                    Value::String(s) => {
                        map.get(s).cloned().ok_or_else(|| anyhow::anyhow!("Key '{}' not found in dictionary", s))
                    }
                    Value::Number(n) => {
                        let key = n.to_string();
                        map.get(&key).cloned().ok_or_else(|| anyhow::anyhow!("Key '{}' not found in dictionary", key))
                    }
                    _ => Err(anyhow::anyhow!("Dictionary key must be a string or number, got: {:?}", index))
                }
            }
            _ => Err(anyhow::anyhow!("Cannot access elements of non-container type: {:?}", container))
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