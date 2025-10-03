//! Cortex AST (Abstract Syntax Tree)
//! 
//! Defines the AST nodes for the Cortex language.

use std::fmt;
use serde::{Serialize, Deserialize};

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub enum NodeType {
//     // Program structure
//     Program,
//     Function,
//     Variable,
//     
//     // Expressions
//     BinaryOp,
//     UnaryOp,
//     Call,
//     Array,
//     Dictionary,
//     Literal,
//     Identifier,
//     
//     // Statements
//     Block,
//     IfStatement,
//     WhileLoop,
//     ForLoop,
//     ReturnStatement,
//     Assignment,
//     ConstantAssignment,
//     
//     // Types
//     TypeAnnotation,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Array(Array),
    Dictionary(Dictionary),
    Index(IndexExpression),
    Range(RangeExpression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Function(Function),
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    ReturnStatement(ReturnStatement),
    Assignment(Assignment),
    ConstantAssignment(ConstantAssignment),
    IndexAssignment(IndexAssignment),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub return_type: Option<String>,
    pub body: Block,
}

impl Function {
    pub fn new(name: String, parameters: Vec<Variable>, return_type: Option<String>, body: Block) -> Self {
        Self { name, parameters, return_type, body }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
}

impl Variable {
    pub fn new(name: String, type_annotation: Option<TypeAnnotation>) -> Self {
        Self { name, type_annotation }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAnnotation {
    pub type_name: String,
}

impl TypeAnnotation {
    pub fn new(type_name: String) -> Self {
        Self { type_name }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl BinaryOp {
    pub fn new(left: Expression, operator: String, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryOp {
    pub operator: String,
    pub operand: Box<Expression>,
}

impl UnaryOp {
    pub fn new(operator: String, operand: Expression) -> Self {
        Self {
            operator,
            operand: Box::new(operand),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Array {
    pub elements: Vec<Expression>,
}

impl Array {
    pub fn new(elements: Vec<Expression>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dictionary {
    pub pairs: Vec<(Expression, Expression)>,
}

impl Dictionary {
    pub fn new(pairs: Vec<(Expression, Expression)>) -> Self {
        Self { pairs }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexExpression {
    pub container: Box<Expression>,
    pub index: Box<Expression>,
}

impl IndexExpression {
    pub fn new(container: Expression, index: Expression) -> Self {
        Self {
            container: Box::new(container),
            index: Box::new(index),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeExpression {
    pub start: Box<Expression>,
    pub end: Box<Expression>,
}

impl RangeExpression {
    pub fn new(start: Expression, end: Expression) -> Self {
        Self {
            start: Box::new(start),
            end: Box::new(end),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub value: LiteralValue,
    pub literal_type: String,
}

impl Literal {
    pub fn new(value: LiteralValue, literal_type: String) -> Self {
        Self { value, literal_type }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

impl IfStatement {
    pub fn new(condition: Expression, then_block: Block, else_block: Option<Block>) -> Self {
        Self { condition, then_block, else_block }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
}

impl WhileLoop {
    pub fn new(condition: Expression, body: Block) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForLoop {
    pub variable: String,
    pub iterable: Option<Expression>,
    pub body: Block,
}

impl ForLoop {
    pub fn new(variable: String, iterable: Option<Expression>, body: Block) -> Self {
        Self { variable, iterable, body }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

impl ReturnStatement {
    pub fn new(value: Option<Expression>) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub variable: String,
    pub value: Expression,
}

impl Assignment {
    pub fn new(variable: String, value: Expression) -> Self {
        Self { variable, value }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstantAssignment {
    pub variable: String,
    pub value: Expression,
}

impl ConstantAssignment {
    pub fn new(variable: String, value: Expression) -> Self {
        Self { variable, value }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexAssignment {
    pub container: Box<Expression>,
    pub index: Box<Expression>,
    pub value: Box<Expression>,
}

impl IndexAssignment {
    pub fn new(container: Expression, index: Expression, value: Expression) -> Self {
        Self {
            container: Box::new(container),
            index: Box::new(index),
            value: Box::new(value),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_literal_creation() {
        let number_literal = Literal::new(LiteralValue::Number(42.0), "number".to_string());
        assert_eq!(number_literal.literal_type, "number");
        
        let string_literal = Literal::new(LiteralValue::String("hello".to_string()), "string".to_string());
        assert_eq!(string_literal.literal_type, "string");
    }
    
    #[test]
    fn test_binary_op_creation() {
        let left = Expression::Literal(Literal::new(LiteralValue::Number(1.0), "number".to_string()));
        let right = Expression::Literal(Literal::new(LiteralValue::Number(2.0), "number".to_string()));
        let binary_op = BinaryOp::new(left, "+".to_string(), right);
        
        assert_eq!(binary_op.operator, "+");
    }
    
    #[test]
    fn test_function_creation() {
        let params = vec![
            Variable::new("a".to_string(), None),
            Variable::new("b".to_string(), None),
        ];
        let body = Block::new(vec![]);
        let function = Function::new("add".to_string(), params, Some("number".to_string()), body);
        
        assert_eq!(function.name, "add");
        assert_eq!(function.parameters.len(), 2);
        assert_eq!(function.return_type, Some("number".to_string()));
    }
}
