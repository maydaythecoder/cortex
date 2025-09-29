"""
Cortex Interpreter

Executes Cortex programs by interpreting the AST.
"""

from typing import Any, Dict, List, Optional
from .ast.nodes import *
from .parser import Parser, ParseError


class CortexRuntime:
    """Runtime environment for Cortex programs."""
    
    def __init__(self):
        self.variables = {}
        self.functions = {}
        self.constants = {}
        
        # Built-in functions
        self._setup_builtins()
    
    def _setup_builtins(self):
        """Setup built-in functions."""
        self.functions['print'] = self._builtin_print
        self.functions['str'] = self._builtin_str
        self.functions['len'] = self._builtin_len
        self.functions['range'] = self._builtin_range
    
    def _builtin_print(self, args: List[Any]) -> None:
        """Built-in print function."""
        if len(args) == 1:
            print(args[0], end='')
        else:
            print(*args, end='')
        return None
    
    def _builtin_str(self, args: List[Any]) -> str:
        """Built-in str function."""
        if len(args) != 1:
            raise RuntimeError("str() takes exactly one argument")
        return str(args[0])
    
    def _builtin_len(self, args: List[Any]) -> int:
        """Built-in len function."""
        if len(args) != 1:
            raise RuntimeError("len() takes exactly one argument")
        value = args[0]
        if isinstance(value, (list, str)):
            return len(value)
        raise RuntimeError(f"len() not supported for type {type(value)}")
    
    def _builtin_range(self, args: List[Any]) -> List[int]:
        """Built-in range function."""
        if len(args) == 1:
            # range(stop)
            stop = int(args[0])
            return list(range(stop))
        elif len(args) == 2:
            # range(start, stop)
            start = int(args[0])
            stop = int(args[1])
            return list(range(start, stop))
        elif len(args) == 3:
            # range(start, stop, step)
            start = int(args[0])
            stop = int(args[1])
            step = int(args[2])
            return list(range(start, stop, step))
        else:
            raise RuntimeError("range() takes 1-3 arguments")


class Interpreter:
    """Interpreter for Cortex programs."""
    
    def __init__(self):
        self.runtime = CortexRuntime()
    
    def interpret(self, program: Program) -> Any:
        """Interpret a Cortex program."""
        try:
            # First pass: collect function definitions
            for statement in program.statements:
                if isinstance(statement, Function):
                    self.runtime.functions[statement.name] = statement
            
            # Second pass: execute statements
            result = None
            for statement in program.statements:
                if not isinstance(statement, Function):
                    result = self._execute_statement(statement)
            
            return result
        except Exception as e:
            raise RuntimeError(f"Runtime error: {e}")
    
    def _execute_statement(self, statement: Statement) -> Any:
        """Execute a statement."""
        if isinstance(statement, Assignment):
            return self._execute_assignment(statement)
        elif isinstance(statement, ConstantAssignment):
            return self._execute_constant_assignment(statement)
        elif isinstance(statement, Block):
            return self._execute_block(statement)
        elif isinstance(statement, IfStatement):
            return self._execute_if_statement(statement)
        elif isinstance(statement, WhileLoop):
            return self._execute_while_loop(statement)
        elif isinstance(statement, ForLoop):
            return self._execute_for_loop(statement)
        elif isinstance(statement, ReturnStatement):
            return self._execute_return_statement(statement)
        elif isinstance(statement, Expression):
            return self._evaluate_expression(statement)
        else:
            raise RuntimeError(f"Unknown statement type: {type(statement)}")
    
    def _execute_assignment(self, assignment: Assignment) -> Any:
        """Execute variable assignment."""
        value = self._evaluate_expression(assignment.value)
        self.runtime.variables[assignment.variable] = value
        return value
    
    def _execute_constant_assignment(self, assignment: ConstantAssignment) -> Any:
        """Execute constant assignment."""
        value = self._evaluate_expression(assignment.value)
        self.runtime.constants[assignment.variable] = value
        return value
    
    def _execute_block(self, block: Block) -> Any:
        """Execute a block of statements."""
        result = None
        for statement in block.statements:
            result = self._execute_statement(statement)
        return result
    
    def _execute_if_statement(self, if_stmt: IfStatement) -> Any:
        """Execute if statement."""
        condition = self._evaluate_expression(if_stmt.condition)
        if self._is_truthy(condition):
            return self._execute_block(if_stmt.then_block)
        elif if_stmt.else_block:
            return self._execute_block(if_stmt.else_block)
        return None
    
    def _execute_while_loop(self, while_loop: WhileLoop) -> Any:
        """Execute while loop."""
        result = None
        while self._is_truthy(self._evaluate_expression(while_loop.condition)):
            result = self._execute_block(while_loop.body)
        return result
    
    def _execute_for_loop(self, for_loop: ForLoop) -> Any:
        """Execute for loop."""
        result = None
        # Simplified for loop implementation
        # In a full implementation, this would handle different iterable types
        return result
    
    def _execute_return_statement(self, return_stmt: ReturnStatement) -> Any:
        """Execute return statement."""
        if return_stmt.value:
            return self._evaluate_expression(return_stmt.value)
        return None
    
    def _evaluate_expression(self, expr: Expression) -> Any:
        """Evaluate an expression."""
        if isinstance(expr, Literal):
            return expr.value
        elif isinstance(expr, Identifier):
            return self._resolve_identifier(expr.name)
        elif isinstance(expr, BinaryOp):
            return self._evaluate_binary_op(expr)
        elif isinstance(expr, UnaryOp):
            return self._evaluate_unary_op(expr)
        elif isinstance(expr, Call):
            return self._evaluate_call(expr)
        elif isinstance(expr, Array):
            return self._evaluate_array(expr)
        elif isinstance(expr, Dictionary):
            return self._evaluate_dictionary(expr)
        else:
            raise RuntimeError(f"Unknown expression type: {type(expr)}")
    
    def _resolve_identifier(self, name: str) -> Any:
        """Resolve an identifier to its value."""
        if name in self.runtime.constants:
            return self.runtime.constants[name]
        elif name in self.runtime.variables:
            return self.runtime.variables[name]
        else:
            raise RuntimeError(f"Undefined variable: {name}")
    
    def _evaluate_binary_op(self, op: BinaryOp) -> Any:
        """Evaluate binary operation."""
        left = self._evaluate_expression(op.left)
        right = self._evaluate_expression(op.right)
        
        if op.operator == '+':
            return left + right
        elif op.operator == '-':
            return left - right
        elif op.operator == '*':
            return left * right
        elif op.operator == '/':
            return left / right
        elif op.operator == '%':
            return left % right
        elif op.operator == '**':
            return left ** right
        elif op.operator == '==':
            return left == right
        elif op.operator == '!=':
            return left != right
        elif op.operator == '<':
            return left < right
        elif op.operator == '<=':
            return left <= right
        elif op.operator == '>':
            return left > right
        elif op.operator == '>=':
            return left >= right
        elif op.operator == '&&':
            return self._is_truthy(left) and self._is_truthy(right)
        elif op.operator == '\|':
            return self._is_truthy(left) or self._is_truthy(right)
        else:
            raise RuntimeError(f"Unknown binary operator: {op.operator}")
    
    def _evaluate_unary_op(self, op: UnaryOp) -> Any:
        """Evaluate unary operation."""
        operand = self._evaluate_expression(op.operand)
        
        if op.operator == '!':
            return not self._is_truthy(operand)
        else:
            raise RuntimeError(f"Unknown unary operator: {op.operator}")
    
    def _evaluate_call(self, call: Call) -> Any:
        """Evaluate function call."""
        # Handle built-in functions
        if isinstance(call.function, Identifier):
            func_name = call.function.name
            if func_name in self.runtime.functions and callable(self.runtime.functions[func_name]):
                args = [self._evaluate_expression(arg) for arg in call.arguments]
                return self.runtime.functions[func_name](args)
        
        # Handle user-defined functions
        if isinstance(call.function, Identifier):
            func_name = call.function.name
            if func_name in self.runtime.functions:
                func_def = self.runtime.functions[func_name]
                if isinstance(func_def, Function):
                    return self._call_function(func_def, call.arguments)
        
        raise RuntimeError(f"Unknown function: {call.function}")
    
    def _call_function(self, func_def: Function, args: List[Expression]) -> Any:
        """Call a user-defined function."""
        if len(args) != len(func_def.parameters):
            raise RuntimeError(f"Function {func_def.name} expects {len(func_def.parameters)} arguments, got {len(args)}")
        
        # Create new scope
        old_variables = self.runtime.variables.copy()
        
        try:
            # Bind parameters
            for param, arg in zip(func_def.parameters, args):
                value = self._evaluate_expression(arg)
                self.runtime.variables[param.name] = value
            
            # Execute function body
            result = self._execute_block(func_def.body)
            return result
        
        finally:
            # Restore old scope
            self.runtime.variables = old_variables
    
    def _evaluate_array(self, array: Array) -> List[Any]:
        """Evaluate array literal."""
        return [self._evaluate_expression(elem) for elem in array.elements]
    
    def _evaluate_dictionary(self, dictionary: Dictionary) -> Dict[Any, Any]:
        """Evaluate dictionary literal."""
        result = {}
        for key_expr, value_expr in dictionary.pairs:
            key = self._evaluate_expression(key_expr)
            value = self._evaluate_expression(value_expr)
            result[key] = value
        return result
    
    def _is_truthy(self, value: Any) -> bool:
        """Check if a value is truthy."""
        if value is None:
            return False
        elif isinstance(value, bool):
            return value
        elif isinstance(value, (int, float)):
            return value != 0
        elif isinstance(value, str):
            return len(value) > 0
        elif isinstance(value, (list, dict)):
            return len(value) > 0
        else:
            return True


def run_cortex_program(source: str) -> Any:
    """Run a Cortex program from source code."""
    try:
        # Parse the source code
        parser = Parser(source)
        program = parser.parse()
        
        # Interpret the program
        interpreter = Interpreter()
        result = interpreter.interpret(program)
        
        return result
    except ParseError as e:
        raise RuntimeError(f"Parse error: {e}")
    except Exception as e:
        raise RuntimeError(f"Runtime error: {e}")
