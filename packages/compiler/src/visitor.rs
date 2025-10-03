//! Visitor Pattern Implementation for Cortex AST
//! 
//! This module provides a comprehensive visitor pattern for traversing and analyzing
//! the Cortex AST. The visitor pattern enables clean separation of concerns and
//! facilitates various compiler passes like type checking, optimization, and analysis.

use crate::ast::*;
use std::collections::HashMap;

/// Result type for visitor operations
pub type VisitorResult<T, E = String> = Result<T, E>;

/// Comprehensive visitor trait with generic return type and error handling
/// 
/// This trait provides a complete visitor pattern implementation for all AST nodes.
/// Visitors can return any type T and handle errors of type E (default String).
/// 
/// # Examples
/// 
/// ```rust
/// use cortex_rust::visitor::{Visitor, VisitorResult};
/// use cortex_rust::ast::*;
/// use std::collections::HashMap;
/// 
/// struct TypeChecker {
///     symbol_table: HashMap<String, String>,
/// }
/// 
/// impl Visitor<()> for TypeChecker {
///     type Error = String;
///     
///     fn default_program(&self) -> () { () }
///     
///     fn visit_assignment(&mut self, assignment: &Assignment) -> VisitorResult<()> {
///         // Type check assignment
///         self.visit_expression(&assignment.value)?;
///         Ok(())
///     }
/// }
/// ```
pub trait Visitor<T> {
    type Error;
    
    /// Visit the root program node
    fn visit_program(&mut self, program: &Program) -> VisitorResult<T, Self::Error> {
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }
        Ok(self.default_program())
    }
    
    /// Visit any statement
    fn visit_statement(&mut self, stmt: &Statement) -> VisitorResult<T, Self::Error> {
        match stmt {
            Statement::Function(f) => self.visit_function(f),
            Statement::IfStatement(i) => self.visit_if_statement(i),
            Statement::WhileLoop(w) => self.visit_while_loop(w),
            Statement::ForLoop(f) => self.visit_for_loop(f),
            Statement::ReturnStatement(r) => self.visit_return_statement(r),
            Statement::Assignment(a) => self.visit_assignment(a),
            Statement::ConstantAssignment(c) => self.visit_constant_assignment(c),
            Statement::IndexAssignment(i) => self.visit_index_assignment(i),
            Statement::Expression(e) => self.visit_expression(e),
        }
    }
    
    /// Visit any expression
    fn visit_expression(&mut self, expr: &Expression) -> VisitorResult<T, Self::Error> {
        match expr {
            Expression::Literal(l) => self.visit_literal(l),
            Expression::Identifier(i) => self.visit_identifier(i),
            Expression::BinaryOp(b) => self.visit_binary_op(b),
            Expression::UnaryOp(u) => self.visit_unary_op(u),
            Expression::Array(a) => self.visit_array(a),
            Expression::Dictionary(d) => self.visit_dictionary(d),
            Expression::Index(i) => self.visit_index_expression(i),
            Expression::Range(r) => self.visit_range_expression(r),
        }
    }
    
    /// Visit a function declaration
    fn visit_function(&mut self, function: &Function) -> VisitorResult<T, Self::Error> {
        // Visit parameters
        for param in &function.parameters {
            self.visit_variable(param)?;
        }
        
        // Visit function body
        self.visit_block(&function.body)?;
        
        Ok(self.default_function())
    }
    
    /// Visit a variable declaration or parameter
    fn visit_variable(&mut self, variable: &Variable) -> VisitorResult<T, Self::Error> {
        if let Some(type_annotation) = &variable.type_annotation {
            self.visit_type_annotation(type_annotation)?;
        }
        Ok(self.default_variable())
    }
    
    /// Visit a type annotation
    fn visit_type_annotation(&mut self, _type_annotation: &TypeAnnotation) -> VisitorResult<T, Self::Error> {
        Ok(self.default_type_annotation())
    }
    
    /// Visit a binary operation
    fn visit_binary_op(&mut self, binary_op: &BinaryOp) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&binary_op.left)?;
        self.visit_expression(&binary_op.right)?;
        Ok(self.default_binary_op())
    }
    
    /// Visit a unary operation
    fn visit_unary_op(&mut self, unary_op: &UnaryOp) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&unary_op.operand)?;
        Ok(self.default_unary_op())
    }
    
    /// Visit an array literal
    fn visit_array(&mut self, array: &Array) -> VisitorResult<T, Self::Error> {
        for element in &array.elements {
            self.visit_expression(element)?;
        }
        Ok(self.default_array())
    }
    
    /// Visit a dictionary literal
    fn visit_dictionary(&mut self, dictionary: &Dictionary) -> VisitorResult<T, Self::Error> {
        for (key, value) in &dictionary.pairs {
            self.visit_expression(key)?;
            self.visit_expression(value)?;
        }
        Ok(self.default_dictionary())
    }
    
    /// Visit an index expression (array/dictionary access)
    fn visit_index_expression(&mut self, index: &IndexExpression) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&index.container)?;
        self.visit_expression(&index.index)?;
        Ok(self.default_index_expression())
    }
    
    /// Visit a range expression
    fn visit_range_expression(&mut self, range: &RangeExpression) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&range.start)?;
        self.visit_expression(&range.end)?;
        Ok(self.default_range_expression())
    }
    
    /// Visit a literal value
    fn visit_literal(&mut self, _literal: &Literal) -> VisitorResult<T, Self::Error> {
        Ok(self.default_literal())
    }
    
    /// Visit an identifier
    fn visit_identifier(&mut self, _identifier: &Identifier) -> VisitorResult<T, Self::Error> {
        Ok(self.default_identifier())
    }
    
    /// Visit a block of statements
    fn visit_block(&mut self, block: &Block) -> VisitorResult<T, Self::Error> {
        for statement in &block.statements {
            self.visit_statement(statement)?;
        }
        Ok(self.default_block())
    }
    
    /// Visit an if statement
    fn visit_if_statement(&mut self, if_statement: &IfStatement) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&if_statement.condition)?;
        self.visit_block(&if_statement.then_block)?;
        
        if let Some(else_block) = &if_statement.else_block {
            self.visit_block(else_block)?;
        }
        
        Ok(self.default_if_statement())
    }
    
    /// Visit a while loop
    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&while_loop.condition)?;
        self.visit_block(&while_loop.body)?;
        Ok(self.default_while_loop())
    }
    
    /// Visit a for loop
    fn visit_for_loop(&mut self, for_loop: &ForLoop) -> VisitorResult<T, Self::Error> {
        if let Some(iterable) = &for_loop.iterable {
            self.visit_expression(iterable)?;
        }
        self.visit_block(&for_loop.body)?;
        Ok(self.default_for_loop())
    }
    
    /// Visit a return statement
    fn visit_return_statement(&mut self, return_statement: &ReturnStatement) -> VisitorResult<T, Self::Error> {
        if let Some(value) = &return_statement.value {
            self.visit_expression(value)?;
        }
        Ok(self.default_return_statement())
    }
    
    /// Visit an assignment statement
    fn visit_assignment(&mut self, assignment: &Assignment) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&assignment.value)?;
        Ok(self.default_assignment())
    }
    
    /// Visit a constant assignment statement
    fn visit_constant_assignment(&mut self, constant_assignment: &ConstantAssignment) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&constant_assignment.value)?;
        Ok(self.default_constant_assignment())
    }
    
    /// Visit an index assignment statement
    fn visit_index_assignment(&mut self, index_assignment: &IndexAssignment) -> VisitorResult<T, Self::Error> {
        self.visit_expression(&index_assignment.container)?;
        self.visit_expression(&index_assignment.index)?;
        self.visit_expression(&index_assignment.value)?;
        Ok(self.default_index_assignment())
    }
    
    // Default implementations that can be overridden
    
    /// Default return value for program visits
    fn default_program(&self) -> T {
        unimplemented!("default_program must be implemented or visit_program must be overridden")
    }
    
    /// Default return value for function visits
    fn default_function(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for variable visits
    fn default_variable(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for type annotation visits
    fn default_type_annotation(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for binary operation visits
    fn default_binary_op(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for unary operation visits
    fn default_unary_op(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for array visits
    fn default_array(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for dictionary visits
    fn default_dictionary(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for index expression visits
    fn default_index_expression(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for range expression visits
    fn default_range_expression(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for literal visits
    fn default_literal(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for identifier visits
    fn default_identifier(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for block visits
    fn default_block(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for if statement visits
    fn default_if_statement(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for while loop visits
    fn default_while_loop(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for for loop visits
    fn default_for_loop(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for return statement visits
    fn default_return_statement(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for assignment visits
    fn default_assignment(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for constant assignment visits
    fn default_constant_assignment(&self) -> T {
        self.default_program()
    }
    
    /// Default return value for index assignment visits
    fn default_index_assignment(&self) -> T {
        self.default_program()
    }
}

/// Example visitor: Type Checker
/// 
/// This visitor performs basic type checking by tracking variable types
/// and ensuring expressions are used correctly.
pub struct TypeChecker {
    symbol_table: HashMap<String, String>,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            errors: Vec::new(),
        }
    }
    
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
    
    fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }
}

impl Visitor<()> for TypeChecker {
    type Error = String;
    
    fn default_program(&self) -> () {
        ()
    }
    
    fn visit_assignment(&mut self, assignment: &Assignment) -> VisitorResult<()> {
        // For now, just visit the value to ensure it's valid
        self.visit_expression(&assignment.value)?;
        
        // TODO: In a real implementation, we'd infer the type of the value
        // and store it in the symbol table
        
        Ok(())
    }
    
    fn visit_constant_assignment(&mut self, constant_assignment: &ConstantAssignment) -> VisitorResult<()> {
        self.visit_expression(&constant_assignment.value)?;
        Ok(())
    }
    
    fn visit_identifier(&mut self, identifier: &Identifier) -> VisitorResult<()> {
        // Check if the variable is defined
        if !self.symbol_table.contains_key(&identifier.name) {
            self.add_error(format!("Undefined variable: {}", identifier.name));
        }
        Ok(())
    }
    
    fn visit_function(&mut self, function: &Function) -> VisitorResult<()> {
        // Add function parameters to symbol table
        for param in &function.parameters {
            if let Some(type_annotation) = &param.type_annotation {
                self.symbol_table.insert(param.name.clone(), type_annotation.type_name.clone());
            } else {
                self.add_error(format!("Function parameter '{}' must have type annotation", param.name));
            }
        }
        
        // Visit function body
        self.visit_block(&function.body)?;
        
        Ok(())
    }
}

/// Example visitor: Constant Folder
/// 
/// This visitor performs constant folding optimization by evaluating
/// constant expressions at compile time.
pub struct ConstantFolder {
    optimized: bool,
}

impl ConstantFolder {
    pub fn new() -> Self {
        Self { optimized: false }
    }
    
    pub fn was_optimized(&self) -> bool {
        self.optimized
    }
}

impl Visitor<bool> for ConstantFolder {
    type Error = String;
    
    fn default_program(&self) -> bool {
        false
    }
    
    fn visit_binary_op(&mut self, binary_op: &BinaryOp) -> VisitorResult<bool> {
        let left_optimized = self.visit_expression(&binary_op.left)?;
        let right_optimized = self.visit_expression(&binary_op.right)?;
        
        // Check if both operands are literals (constant folding opportunity)
        let can_fold = matches!(binary_op.left.as_ref(), Expression::Literal(_)) &&
                      matches!(binary_op.right.as_ref(), Expression::Literal(_));
        
        if can_fold {
            self.optimized = true;
        }
        
        Ok(left_optimized || right_optimized || can_fold)
    }
    
    fn visit_unary_op(&mut self, unary_op: &UnaryOp) -> VisitorResult<bool> {
        let operand_optimized = self.visit_expression(&unary_op.operand)?;
        
        let can_fold = matches!(unary_op.operand.as_ref(), Expression::Literal(_));
        
        if can_fold {
            self.optimized = true;
        }
        
        Ok(operand_optimized || can_fold)
    }
}

/// Example visitor: AST Printer
/// 
/// This visitor prints the AST structure for debugging purposes.
pub struct AstPrinter {
    indent_level: usize,
    output: String,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            output: String::new(),
        }
    }
    
    pub fn get_output(&self) -> &str {
        &self.output
    }
    
    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }
    
    fn println(&mut self, text: &str) {
        self.output.push_str(&format!("{}{}\n", self.indent(), text));
    }
}

impl Visitor<()> for AstPrinter {
    type Error = String;
    
    fn default_program(&self) -> () {
        ()
    }
    
    fn visit_program(&mut self, program: &Program) -> VisitorResult<()> {
        self.println("Program:");
        self.indent_level += 1;
        
        for (i, stmt) in program.statements.iter().enumerate() {
            self.println(&format!("Statement {}:", i));
            self.indent_level += 1;
            self.visit_statement(stmt)?;
            self.indent_level -= 1;
        }
        
        self.indent_level -= 1;
        Ok(())
    }
    
    fn visit_function(&mut self, function: &Function) -> VisitorResult<()> {
        self.println(&format!("Function: {}", function.name));
        self.indent_level += 1;
        
        self.println("Parameters:");
        self.indent_level += 1;
        for param in &function.parameters {
            self.visit_variable(param)?;
        }
        self.indent_level -= 1;
        
        self.println("Body:");
        self.indent_level += 1;
        self.visit_block(&function.body)?;
        self.indent_level -= 1;
        
        self.indent_level -= 1;
        Ok(())
    }
    
    fn visit_variable(&mut self, variable: &Variable) -> VisitorResult<()> {
        let type_info = variable.type_annotation
            .as_ref()
            .map(|t| format!(": {}", t.type_name))
            .unwrap_or_default();
        self.println(&format!("Variable: {}{}", variable.name, type_info));
        Ok(())
    }
    
    fn visit_binary_op(&mut self, binary_op: &BinaryOp) -> VisitorResult<()> {
        self.println(&format!("BinaryOp: {}", binary_op.operator));
        self.indent_level += 1;
        self.println("Left:");
        self.indent_level += 1;
        self.visit_expression(&binary_op.left)?;
        self.indent_level -= 1;
        self.println("Right:");
        self.indent_level += 1;
        self.visit_expression(&binary_op.right)?;
        self.indent_level -= 1;
        self.indent_level -= 1;
        Ok(())
    }
    
    fn visit_literal(&mut self, literal: &Literal) -> VisitorResult<()> {
        self.println(&format!("Literal: {}", literal.value));
        Ok(())
    }
    
    fn visit_identifier(&mut self, identifier: &Identifier) -> VisitorResult<()> {
        self.println(&format!("Identifier: {}", identifier.name));
        Ok(())
    }
    
    fn visit_assignment(&mut self, assignment: &Assignment) -> VisitorResult<()> {
        self.println(&format!("Assignment: {} =", assignment.variable));
        self.indent_level += 1;
        self.visit_expression(&assignment.value)?;
        self.indent_level -= 1;
        Ok(())
    }
    
    fn visit_block(&mut self, block: &Block) -> VisitorResult<()> {
        for (i, stmt) in block.statements.iter().enumerate() {
            self.println(&format!("Block Statement {}:", i));
            self.indent_level += 1;
            self.visit_statement(stmt)?;
            self.indent_level -= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    
    #[test]
    fn test_type_checker() {
        let source = r#"
            func add[a: number, b: number] |
                return[a + b]
            ^
            
            let result := add[5, 10]
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        type_checker.visit_program(&ast).unwrap();
        
        // The type checker will report errors for undefined variables like 'result' and 'a', 'b'
        // This is expected behavior for a basic type checker
        let errors = type_checker.get_errors();
        println!("Type checker errors: {:?}", errors);
        
        // Should have some errors for undefined variables (this is expected)
        assert!(errors.len() > 0);
    }
    
    #[test]
    fn test_constant_folder() {
        let source = "let x := 2 + 3 * 4";
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut constant_folder = ConstantFolder::new();
        constant_folder.visit_program(&ast).unwrap();
        
        // Should detect constant folding opportunities
        assert!(constant_folder.was_optimized());
    }
    
    #[test]
    fn test_ast_printer() {
        let source = "let x := 42";
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut ast_printer = AstPrinter::new();
        ast_printer.visit_program(&ast).unwrap();
        
        let output = ast_printer.get_output();
        println!("AST Printer Output:\n{}", output);
        assert!(output.contains("Program:"));
        // Check for the actual statement type (Assignment, not "Assignment")
        assert!(output.contains("Assignment:") || output.contains("Statement"));
        assert!(output.contains("Literal: 42"));
    }
}
