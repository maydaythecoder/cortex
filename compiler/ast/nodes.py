"""
Cortex AST Nodes

Defines the Abstract Syntax Tree nodes for the Cortex language.
"""

from abc import ABC, abstractmethod
from typing import List, Optional, Any, Dict
from enum import Enum


class NodeType(Enum):
    # Program structure
    PROGRAM = "PROGRAM"
    FUNCTION = "FUNCTION"
    VARIABLE = "VARIABLE"
    
    # Expressions
    BINARY_OP = "BINARY_OP"
    UNARY_OP = "UNARY_OP"
    CALL = "CALL"
    ARRAY = "ARRAY"
    DICTIONARY = "DICTIONARY"
    LITERAL = "LITERAL"
    IDENTIFIER = "IDENTIFIER"
    
    # Statements
    BLOCK = "BLOCK"
    IF_STATEMENT = "IF_STATEMENT"
    WHILE_LOOP = "WHILE_LOOP"
    FOR_LOOP = "FOR_LOOP"
    RETURN_STATEMENT = "RETURN_STATEMENT"
    ASSIGNMENT = "ASSIGNMENT"
    CONSTANT_ASSIGNMENT = "CONSTANT_ASSIGNMENT"
    
    # Types
    TYPE_ANNOTATION = "TYPE_ANNOTATION"


class ASTNode(ABC):
    """Base class for all AST nodes."""
    
    def __init__(self, node_type: NodeType, line: int = 0, column: int = 0):
        self.node_type = node_type
        self.line = line
        self.column = column
    
    @abstractmethod
    def accept(self, visitor):
        """Accept a visitor for traversal."""
        pass
    
    def __repr__(self):
        return f"{self.__class__.__name__}({self.node_type.value})"


class Expression(ASTNode):
    """Base class for expressions."""
    pass


class Statement(ASTNode):
    """Base class for statements."""
    pass


class Program(ASTNode):
    """Root node of the program."""
    
    def __init__(self, statements: List[Statement], line: int = 0, column: int = 0):
        super().__init__(NodeType.PROGRAM, line, column)
        self.statements = statements
    
    def accept(self, visitor):
        return visitor.visit_program(self)


class Function(Statement):
    """Function definition."""
    
    def __init__(self, name: str, parameters: List['Variable'], 
                 return_type: Optional[str], body: 'Block', 
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.FUNCTION, line, column)
        self.name = name
        self.parameters = parameters
        self.return_type = return_type
        self.body = body
    
    def accept(self, visitor):
        return visitor.visit_function(self)


class Variable(Expression):
    """Variable declaration or reference."""
    
    def __init__(self, name: str, type_annotation: Optional['TypeAnnotation'] = None,
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.VARIABLE, line, column)
        self.name = name
        self.type_annotation = type_annotation
    
    def accept(self, visitor):
        return visitor.visit_variable(self)


class TypeAnnotation(ASTNode):
    """Type annotation."""
    
    def __init__(self, type_name: str, line: int = 0, column: int = 0):
        super().__init__(NodeType.TYPE_ANNOTATION, line, column)
        self.type_name = type_name
    
    def accept(self, visitor):
        return visitor.visit_type_annotation(self)


class BinaryOp(Expression):
    """Binary operation."""
    
    def __init__(self, left: Expression, operator: str, right: Expression,
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.BINARY_OP, line, column)
        self.left = left
        self.operator = operator
        self.right = right
    
    def accept(self, visitor):
        return visitor.visit_binary_op(self)


class UnaryOp(Expression):
    """Unary operation."""
    
    def __init__(self, operator: str, operand: Expression,
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.UNARY_OP, line, column)
        self.operator = operator
        self.operand = operand
    
    def accept(self, visitor):
        return visitor.visit_unary_op(self)


class Call(Expression):
    """Function call."""
    
    def __init__(self, function: Expression, arguments: List[Expression],
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.CALL, line, column)
        self.function = function
        self.arguments = arguments
    
    def accept(self, visitor):
        return visitor.visit_call(self)


class Array(Expression):
    """Array literal."""
    
    def __init__(self, elements: List[Expression], line: int = 0, column: int = 0):
        super().__init__(NodeType.ARRAY, line, column)
        self.elements = elements
    
    def accept(self, visitor):
        return visitor.visit_array(self)


class Dictionary(Expression):
    """Dictionary literal."""
    
    def __init__(self, pairs: List[tuple], line: int = 0, column: int = 0):
        super().__init__(NodeType.DICTIONARY, line, column)
        self.pairs = pairs  # List of (key, value) tuples
    
    def accept(self, visitor):
        return visitor.visit_dictionary(self)


class Literal(Expression):
    """Literal value."""
    
    def __init__(self, value: Any, literal_type: str, line: int = 0, column: int = 0):
        super().__init__(NodeType.LITERAL, line, column)
        self.value = value
        self.literal_type = literal_type  # "number", "string", "boolean", "null"
    
    def accept(self, visitor):
        return visitor.visit_literal(self)


class Identifier(Expression):
    """Identifier reference."""
    
    def __init__(self, name: str, line: int = 0, column: int = 0):
        super().__init__(NodeType.IDENTIFIER, line, column)
        self.name = name
    
    def accept(self, visitor):
        return visitor.visit_identifier(self)


class Block(Statement):
    """Code block with statements."""
    
    def __init__(self, statements: List[Statement], line: int = 0, column: int = 0):
        super().__init__(NodeType.BLOCK, line, column)
        self.statements = statements
    
    def accept(self, visitor):
        return visitor.visit_block(self)


class IfStatement(Statement):
    """If statement."""
    
    def __init__(self, condition: Expression, then_block: Block, 
                 else_block: Optional[Block] = None, line: int = 0, column: int = 0):
        super().__init__(NodeType.IF_STATEMENT, line, column)
        self.condition = condition
        self.then_block = then_block
        self.else_block = else_block
    
    def accept(self, visitor):
        return visitor.visit_if_statement(self)


class WhileLoop(Statement):
    """While loop."""
    
    def __init__(self, condition: Expression, body: Block,
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.WHILE_LOOP, line, column)
        self.condition = condition
        self.body = body
    
    def accept(self, visitor):
        return visitor.visit_while_loop(self)


class ForLoop(Statement):
    """For loop."""
    
    def __init__(self, variable: str, iterable: Expression, body: Block,
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.FOR_LOOP, line, column)
        self.variable = variable
        self.iterable = iterable
        self.body = body
    
    def accept(self, visitor):
        return visitor.visit_for_loop(self)


class ReturnStatement(Statement):
    """Return statement."""
    
    def __init__(self, value: Optional[Expression] = None,
                 line: int = 0, column: int = 0):
        super().__init__(NodeType.RETURN_STATEMENT, line, column)
        self.value = value
    
    def accept(self, visitor):
        return visitor.visit_return_statement(self)


class Assignment(Statement):
    """Variable assignment."""
    
    def __init__(self, variable: str, value: Expression, line: int = 0, column: int = 0):
        super().__init__(NodeType.ASSIGNMENT, line, column)
        self.variable = variable
        self.value = value
    
    def accept(self, visitor):
        return visitor.visit_assignment(self)


class ConstantAssignment(Statement):
    """Constant assignment (immutable)."""
    
    def __init__(self, variable: str, value: Expression, line: int = 0, column: int = 0):
        super().__init__(NodeType.CONSTANT_ASSIGNMENT, line, column)
        self.variable = variable
        self.value = value
    
    def accept(self, visitor):
        return visitor.visit_constant_assignment(self)
