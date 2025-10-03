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

/// The Cortex interpreter that executes AST nodes directly.
/// 
/// Provides a tree-walking interpreter with variable scoping, function calls,
/// and built-in functions.
/// 
/// # Examples
/// 
/// ```
/// use cortex_rust::{lexer::Lexer, parser::Parser as CortexParser, codegen::Interpreter};
/// 
/// let source = r#"
///     func add[a, b] |
///         return[a + b]
///     ^
///     
///     let result := add[10, 20]
///     print[result]
/// "#;
/// 
/// let mut lexer = Lexer::new(source);
/// let tokens = lexer.tokenize().unwrap();
/// let mut parser = CortexParser::new(tokens);
/// let ast = parser.parse().unwrap();
/// 
/// let mut interpreter = Interpreter::new();
/// interpreter.interpret(&ast).unwrap();
/// ```
pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    current_return_value: Option<Value>,
}

impl Interpreter {
    /// Creates a new interpreter with empty variable and function environments.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cortex_rust::codegen::Interpreter;
    /// 
    /// let interpreter = Interpreter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_return_value: None,
        }
    }
    
    /// Interprets a Cortex program AST.
    /// 
    /// Executes all statements in the program sequentially, maintaining state
    /// across function and variable definitions.
    /// 
    /// # Arguments
    /// 
    /// * `program` - The root AST node containing all statements
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If execution completes successfully
    /// * `Err(anyhow::Error)` - If a runtime error occurs
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cortex_rust::{lexer::Lexer, parser::Parser as CortexParser, codegen::Interpreter, ast::Program};
    /// 
    /// let source = "let x := 42";
    /// let mut lexer = Lexer::new(source);
    /// let tokens = lexer.tokenize().unwrap();
    /// let mut parser = CortexParser::new(tokens);
    /// let ast = parser.parse().unwrap();
    /// 
    /// let mut interpreter = Interpreter::new();
    /// interpreter.interpret(&ast).unwrap();
    /// ```
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
            Statement::IndexAssignment(index_assignment) => {
                self.interpret_index_assignment(index_assignment)?;
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
                Value::Dictionary(dict) => {
                    // Dictionary iteration: iterate over keys
                    for (key, value) in dict.iter() {
                        // Set both key and value variables
                        self.variables.insert(for_loop.variable.clone(), Value::String(key.clone()));
                        self.variables.insert(format!("{}_value", for_loop.variable), value.clone());
                        
                        // Interpret the loop body
                        self.interpret_block(&for_loop.body)?;
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Cannot iterate over non-array, number, or dictionary value"));
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
    
    fn interpret_index_assignment(&mut self, index_assignment: &IndexAssignment) -> Result<()> {
        let container_value = self.interpret_expression(&index_assignment.container)?;
        let index_value = self.interpret_expression(&index_assignment.index)?;
        let new_value = self.interpret_expression(&index_assignment.value)?;
        
        match (container_value, index_value) {
            (Value::Array(mut arr), Value::Number(index)) => {
                let idx = index as usize;
                if idx >= arr.len() || index < 0.0 {
                    return Err(anyhow::anyhow!("Array index {} out of bounds (length: {}) - cannot assign to out-of-bounds index", idx, arr.len()));
                }
                arr[idx] = new_value;
                
                // Update the variable if it's an identifier
                if let Expression::Identifier(identifier) = &*index_assignment.container {
                    self.variables.insert(identifier.name.clone(), Value::Array(arr));
                }
            }
            (Value::Dictionary(mut dict), Value::String(key)) => {
                dict.insert(key, new_value);
                
                // Update the variable if it's an identifier
                if let Expression::Identifier(identifier) = &*index_assignment.container {
                    self.variables.insert(identifier.name.clone(), Value::Dictionary(dict));
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Cannot assign to non-array or non-dictionary value"));
            }
        }
        
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
                    "+" => {
                        if let Value::Number(n) = operand {
                            Ok(Value::Number(n))
                        } else {
                            Err(anyhow::anyhow!("Cannot apply unary plus to non-number value"))
                        }
                    }
                    _ => Err(anyhow::anyhow!("Unsupported unary operator: {}", unary_op.operator))
                }
            }
            /* Expression::Call(call) => {
                // if let Expression::Identifier(func_name) = &*call.function {
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
            } */
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
            Expression::Range(range_expr) => {
                let start = self.interpret_expression(&range_expr.start)?;
                let end = self.interpret_expression(&range_expr.end)?;

                // Convert to numbers and create range
                if let (Value::Number(start_num), Value::Number(end_num)) = (start, end) {
                    let mut range_array = Vec::new();
                    let mut current = start_num;
                    while current < end_num {
                        range_array.push(Value::Number(current));
                        current += 1.0;
                    }
                    Ok(Value::Array(range_array))
                } else {
                    Err(anyhow::anyhow!("Range expressions require numbers"))
                }
            }
            Expression::Index(index_expr) => {
                // Check if this is a function call
                if let Expression::Identifier(identifier) = &*index_expr.container {
                    // Check if this is a built-in function
                    if identifier.name == "print" {
                        // Handle print function
                        let value = self.interpret_expression(&index_expr.index)?;
                        println!("{}", value);
                        Ok(Value::Null)
                    } else if identifier.name == "str" {
                        // Handle str function
                        let value = self.interpret_expression(&index_expr.index)?;
                        Ok(Value::String(value.to_string()))
                    } else if identifier.name == "len" {
                        // Handle len function
                        let value = self.interpret_expression(&index_expr.index)?;
                        let length = match value {
                            Value::Array(arr) => arr.len(),
                            Value::Dictionary(dict) => dict.len(),
                            Value::String(s) => s.len(),
                            Value::Null => return Err(anyhow::anyhow!("len() requires an argument - provide an array, dictionary, or string")),
                            _ => return Err(anyhow::anyhow!("len() expects array, dictionary, or string, got {:?}", value)),
                        };
                        Ok(Value::Number(length as f64))
                    } else if identifier.name == "push" {
                        // Handle push function (array, value)
                        let index_value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Array(args) = index_value {
                            if args.len() != 2 {
                                return Err(anyhow::anyhow!("push() expects exactly 2 arguments: array and value"));
                            }
                            let array = args[0].clone();
                            let value = args[1].clone();
                            if let Value::Array(mut arr) = array {
                                arr.push(value);
                                Ok(Value::Array(arr))
                            } else {
                                Err(anyhow::anyhow!("push() first argument must be an array"))
                            }
                        } else {
                            Err(anyhow::anyhow!("push() expects array arguments"))
                        }
                    } else if identifier.name == "pop" {
                        // Handle pop function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Array(mut arr) = value {
                            if arr.is_empty() {
                                Ok(Value::Null)
                            } else {
                                Ok(arr.pop().unwrap())
                            }
                        } else {
                            Err(anyhow::anyhow!("pop() expects an array"))
                        }
                    } else if identifier.name == "keys" {
                        // Handle keys function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Dictionary(dict) = value {
                            let keys: Vec<Value> = dict.keys().map(|k| Value::String(k.clone())).collect();
                            Ok(Value::Array(keys))
                        } else {
                            Err(anyhow::anyhow!("keys() expects a dictionary"))
                        }
                    } else if identifier.name == "values" {
                        // Handle values function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Dictionary(dict) = value {
                            let values: Vec<Value> = dict.values().cloned().collect();
                            Ok(Value::Array(values))
                        } else {
                            Err(anyhow::anyhow!("values() expects a dictionary"))
                        }
                    } else if identifier.name == "abs" {
                        // Handle abs function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Number(n) = value {
                            Ok(Value::Number(n.abs()))
                        } else {
                            Err(anyhow::anyhow!("abs() expects a number"))
                        }
                    } else if identifier.name == "min" {
                        // Handle min function
                        let index_value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Array(args) = index_value {
                            if args.is_empty() {
                                return Err(anyhow::anyhow!("min() expects at least one argument"));
                            }
                            let mut min_val = None;
                            for arg in args {
                                if let Value::Number(n) = arg {
                                    match min_val {
                                        None => min_val = Some(n),
                                        Some(current_min) => {
                                            if n < current_min {
                                                min_val = Some(n);
                                            }
                                        }
                                    }
                                } else {
                                    return Err(anyhow::anyhow!("min() expects all arguments to be numbers"));
                                }
                            }
                            Ok(Value::Number(min_val.unwrap()))
                        } else {
                            Err(anyhow::anyhow!("min() expects array of numbers"))
                        }
                    } else if identifier.name == "max" {
                        // Handle max function
                        let index_value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Array(args) = index_value {
                            if args.is_empty() {
                                return Err(anyhow::anyhow!("max() expects at least one argument"));
                            }
                            let mut max_val = None;
                            for arg in args {
                                if let Value::Number(n) = arg {
                                    match max_val {
                                        None => max_val = Some(n),
                                        Some(current_max) => {
                                            if n > current_max {
                                                max_val = Some(n);
                                            }
                                        }
                                    }
                                } else {
                                    return Err(anyhow::anyhow!("max() expects all arguments to be numbers"));
                                }
                            }
                            Ok(Value::Number(max_val.unwrap()))
                        } else {
                            Err(anyhow::anyhow!("max() expects array of numbers"))
                        }
                    } else if identifier.name == "substring" {
                        // Handle substring function (string, start, length)
                        let index_value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Array(args) = index_value {
                            if args.len() != 3 {
                                return Err(anyhow::anyhow!("substring() expects exactly 3 arguments: string, start, length"));
                            }
                            if let (Value::String(s), Value::Number(start), Value::Number(length)) = (&args[0], &args[1], &args[2]) {
                                let start_idx = *start as usize;
                                let len = *length as usize;
                                if start_idx >= s.len() {
                                    Ok(Value::String(String::new()))
                                } else {
                                    let end_idx = (start_idx + len).min(s.len());
                                    Ok(Value::String(s[start_idx..end_idx].to_string()))
                                }
                            } else {
                                Err(anyhow::anyhow!("substring() expects string, number, number arguments"))
                            }
                        } else {
                            Err(anyhow::anyhow!("substring() expects array of arguments"))
                        }
                    } else if identifier.name == "upper" {
                        // Handle upper function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::String(s) = value {
                            Ok(Value::String(s.to_uppercase()))
                        } else {
                            Err(anyhow::anyhow!("upper() expects a string"))
                        }
                    } else if identifier.name == "lower" {
                        // Handle lower function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::String(s) = value {
                            Ok(Value::String(s.to_lowercase()))
                        } else {
                            Err(anyhow::anyhow!("lower() expects a string"))
                        }
                    } else if identifier.name == "split" {
                        // Handle split function (string, delimiter)
                        let index_value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Array(args) = index_value {
                            if args.len() != 2 {
                                return Err(anyhow::anyhow!("split() expects exactly 2 arguments: string, delimiter"));
                            }
                            if let (Value::String(s), Value::String(delimiter)) = (&args[0], &args[1]) {
                                let parts: Vec<Value> = s.split(delimiter)
                                    .map(|part| Value::String(part.to_string()))
                                    .collect();
                                Ok(Value::Array(parts))
                            } else {
                                Err(anyhow::anyhow!("split() expects string, string arguments"))
                            }
                        } else {
                            Err(anyhow::anyhow!("split() expects array of arguments"))
                        }
                    } else if identifier.name == "sqrt" {
                        // Handle sqrt function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Number(n) = value {
                            if n < 0.0 {
                                // Graceful handling: return null for sqrt of negative numbers
                                Ok(Value::Null)
                            } else {
                                Ok(Value::Number(n.sqrt()))
                            }
                        } else {
                            Err(anyhow::anyhow!("sqrt() expects a number"))
                        }
                    } else if identifier.name == "floor" {
                        // Handle floor function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Number(n) = value {
                            Ok(Value::Number(n.floor()))
                        } else {
                            Err(anyhow::anyhow!("floor() expects a number"))
                        }
                    } else if identifier.name == "ceil" {
                        // Handle ceil function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Number(n) = value {
                            Ok(Value::Number(n.ceil()))
                        } else {
                            Err(anyhow::anyhow!("ceil() expects a number"))
                        }
                    } else if identifier.name == "round" {
                        // Handle round function
                        let value = self.interpret_expression(&index_expr.index)?;
                        if let Value::Number(n) = value {
                            Ok(Value::Number(n.round()))
                        } else {
                            Err(anyhow::anyhow!("round() expects a number"))
                        }
                    } else if self.variables.contains_key(&identifier.name) {
                        // This is array/dictionary indexing - the identifier refers to a variable
                        let container = self.interpret_expression(&index_expr.container)?;
                        let index = self.interpret_expression(&index_expr.index)?;
                        self.access_element(&container, &index)
                    } else if self.functions.contains_key(&identifier.name) {
                        // This is a user-defined function call
                        let index_value = self.interpret_expression(&index_expr.index)?;
                        
                        // Check if this is a function call with no arguments (index is null)
                        if matches!(index_value, Value::Null) {
                            // Function call with no arguments
                            self.call_function(&identifier.name, vec![])
                        } else if let Value::Array(ref arguments) = index_value {
                            // Check if this is a multi-argument call or a single array argument
                            // The key insight: if the index_expr.index is an Array expression,
                            // it was parsed from comma-separated arguments (multi-arg call).
                            // If the index_expr.index is an Identifier, it's a single array argument.
                            
                            match &*index_expr.index {
                                Expression::Array(_) => {
                                    // This was parsed from comma-separated arguments like add[10, 20]
                                    // Treat as multi-argument call
                                    self.call_function(&identifier.name, arguments.clone())
                                }
                                Expression::Identifier(_) => {
                                    // This was parsed from a single identifier like bubble_sort[unsorted]
                                    // Treat as single-argument call with array argument
                                    self.call_function(&identifier.name, vec![index_value])
                                }
                                _ => {
                                    // For other cases, treat as single-argument call
                                    self.call_function(&identifier.name, vec![index_value])
                                }
                            }
                        } else {
                            // Single argument function call
                            self.call_function(&identifier.name, vec![index_value])
                        }
                    } else {
                        // This is array/dictionary indexing - the identifier could refer to a variable that hasn't been declared yet
                        // or it might be a typo. Try to access as variable first, let it fail naturally if undefined
                        let container = self.interpret_expression(&index_expr.container)?;
                        let index = self.interpret_expression(&index_expr.index)?;
                        self.access_element(&container, &index)
                    }
                } else {
                    // This is array/dictionary indexing
                    let container = self.interpret_expression(&index_expr.container)?;
                    let index = self.interpret_expression(&index_expr.index)?;
                    self.access_element(&container, &index)
                }
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
                    // Set the return value and break
                    self.current_return_value = Some(result.clone());
                    break; // Return immediately
                }
                _ => {
                    self.interpret_statement(statement)?;
                    // Check if we hit a return statement
                    if self.current_return_value.is_some() {
                        break;
                    }
                }
            }
        }
        Ok(result)
    }
    
    fn call_function(&mut self, function_name: &str, arguments: Vec<Value>) -> Result<Value> {
        if let Some(function) = self.functions.get(function_name).cloned() {
            // Check argument count
            if arguments.len() != function.parameters.len() {
                return Err(anyhow::anyhow!(
                    "Function {} expects {} arguments, got {}",
                    function.name,
                    function.parameters.len(),
                    arguments.len()
                ));
            }
            
            // Save current variables and return value
            let old_variables = self.variables.clone();
            let old_return_value = self.current_return_value.clone();
            self.current_return_value = None;
            
            // Bind parameters to arguments
            for (param, arg_value) in function.parameters.iter().zip(arguments.iter()) {
                self.variables.insert(param.name.clone(), arg_value.clone());
            }
            
            // Execute function body
            let result = self.interpret_block(&function.body)?;
            
            // Check if we have a return value
            let final_result = if let Some(return_val) = &self.current_return_value {
                return_val.clone()
            } else {
                result
            };
            
            // Restore old variables and return value
            self.variables = old_variables;
            self.current_return_value = old_return_value;
            
            Ok(final_result)
        } else {
            Err(anyhow::anyhow!("Undefined function: {}", function_name))
        }
    }
    
    /* fn interpret_function_call(&mut self, function: &Function, arguments: &[Expression]) -> Result<Value> {
        // Check argument count
        if arguments.len() != function.parameters.len() {
            return Err(anyhow::anyhow!(
                "Function {} expects {} arguments, got {}",
                function.name,
                function.parameters.len(),
                arguments.len()
            ));
        }
        
        // Save current variables and return value
        let old_variables = self.variables.clone();
        let old_return_value = self.current_return_value.clone();
        self.current_return_value = None;
        
        // Bind parameters to arguments
        for (param, arg) in function.parameters.iter().zip(arguments.iter()) {
            let value = self.interpret_expression(arg)?;
            self.variables.insert(param.name.clone(), value);
        }
        
        // Execute function body
        let result = self.interpret_block(&function.body)?;
        
        // Check if we have a return value
        let final_result = if let Some(return_val) = &self.current_return_value {
            return_val.clone()
        } else {
            result
        };
        
        // Restore variables and return value
        self.variables = old_variables;
        self.current_return_value = old_return_value;
        
        // Return the result (not wrapped in array)
        Ok(final_result)
    } */

    // fn interpret_print_call(&mut self, args: &[Expression]) -> Result<()> {
    //     for (i, arg) in args.iter().enumerate() {
    //         let value = self.interpret_expression(arg)?;
    //         if i > 0 {
    //             print!(" ");
    //         }
    //         print!("{}", value);
    //     }
    //     println!();
    //     Ok(())
    // }
    
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
                    // Graceful handling: return null for division by zero
                    Ok(Value::Null)
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
                    // Graceful handling: return null for modulo by zero
                    Ok(Value::Null)
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
                        if idx < arr.len() && *n >= 0.0 {
                            Ok(arr[idx].clone())
                        } else {
                            // Graceful handling: return null for out-of-bounds access
                            Ok(Value::Null)
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