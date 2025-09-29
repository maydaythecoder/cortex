"""
Cortex Code Generator

Generates LLVM IR from Cortex AST for compilation to binary.
"""

import llvmlite.ir as ir
import llvmlite.binding as llvm
from typing import Dict, Any, Optional, List
from .ast.nodes import *
from .interpreter import CortexRuntime


class CodeGenerator:
    """Generates LLVM IR from Cortex AST."""
    
    def __init__(self):
        # LLVM module and context
        self.module = ir.Module(name="cortex_module")
        
        # Set target triple for macOS ARM64
        import platform
        if platform.system() == "Darwin":
            if platform.machine() == "arm64":
                self.module.triple = "arm64-apple-macosx"
            else:
                self.module.triple = "x86_64-apple-macosx"
        else:
            self.module.triple = "x86_64-unknown-linux-gnu"
        
        self.builder = None
        
        # Type mappings
        self.type_map = {
            'number': ir.DoubleType(),
            'string': ir.IntType(8).as_pointer(),  # char*
            'boolean': ir.IntType(1),
            'void': ir.VoidType()
        }
        
        # Function and variable mappings
        self.functions = {}
        self.global_vars = {}
        self.local_vars = {}
        
        # Current function context
        self.current_function = None
        self.current_block = None
        
        # String constants
        self.string_constants = {}
        self.string_counter = 0
        
        # Setup LLVM
        self._setup_llvm()
    
    def _setup_llvm(self):
        """Initialize LLVM bindings."""
        # LLVM initialization is now handled automatically in newer versions
        # Only initialize if the functions exist
        try:
            llvm.initialize()
            llvm.initialize_native_target()
            llvm.initialize_native_asmprinter()
        except:
            # LLVM initialization is automatic in newer versions
            pass
    
    def generate(self, program: Program) -> str:
        """Generate LLVM IR from a Cortex program."""
        # First pass: collect all function definitions and global variables
        for statement in program.statements:
            if isinstance(statement, Function):
                self._generate_function_declaration(statement)
            elif isinstance(statement, Assignment) or isinstance(statement, ConstantAssignment):
                # Handle global variable assignments
                old_function = self.current_function
                self.current_function = None
                self._generate_assignment(statement)
                self.current_function = old_function
        
        # Second pass: generate function bodies
        for statement in program.statements:
            if isinstance(statement, Function):
                self._generate_function(statement)
        
        # Check if there's a user-defined main function
        has_user_main = any(isinstance(stmt, Function) and stmt.name == "main" 
                           for stmt in program.statements)
        
        if has_user_main:
            # Call the user-defined main function
            self._generate_main_wrapper(program)
        else:
            # Create a simple main function that executes top-level statements
            self._generate_simple_main(program)
        
        return str(self.module)
    
    def _generate_function_declaration(self, func: Function):
        """Generate function declaration only."""
        # Map parameter types
        param_types = []
        for param in func.parameters:
            # Default to number type for now
            param_types.append(self.type_map['number'])
        
        # Determine return type
        return_type = self.type_map['void']
        if func.return_type:
            return_type = self.type_map.get(func.return_type, self.type_map['number'])
        
        # Create function
        func_type = ir.FunctionType(return_type, param_types)
        
        # If this is the main function, rename it to avoid conflicts
        func_name = func.name
        if func.name == "main":
            func_name = "cortex_main"
        
        llvm_func = ir.Function(self.module, func_type, name=func_name)
        
        # Store function reference with original name
        self.functions[func.name] = llvm_func
    
    def _generate_main_wrapper(self, program: Program):
        """Generate a main function that calls the user-defined main."""
        # Global variables are already handled in the first pass
        
        # The user-defined main function is already created, we need to create a wrapper
        # Create a wrapper function with a different name
        main_wrapper_type = ir.FunctionType(self.type_map['void'], [])
        main_wrapper = ir.Function(self.module, main_wrapper_type, name="main_wrapper")
        main_block = main_wrapper.append_basic_block(name="entry")
        
        self.builder = ir.IRBuilder(main_block)
        self.current_function = main_wrapper
        self.current_block = main_block
        
        # Call the user-defined main function
        if "main" in self.functions:
            self.builder.call(self.functions["main"], [])
        
        # Add return statement
        self.builder.ret_void()
        
        # Now create the actual main function that calls our wrapper
        actual_main_type = ir.FunctionType(self.type_map['void'], [])
        actual_main = ir.Function(self.module, actual_main_type, name="main")
        actual_main_block = actual_main.append_basic_block(name="entry")
        
        old_builder = self.builder
        old_function = self.current_function
        old_block = self.current_block
        
        self.builder = ir.IRBuilder(actual_main_block)
        self.current_function = actual_main
        self.current_block = actual_main_block
        
        # Call the wrapper function
        self.builder.call(main_wrapper, [])
        self.builder.ret_void()
        
        # Restore context
        self.builder = old_builder
        self.current_function = old_function
        self.current_block = old_block
    
    def _generate_simple_main(self, program: Program):
        """Generate a simple main function for programs without user-defined main."""
        # Create main function
        main_func_type = ir.FunctionType(self.type_map['void'], [])
        main_func = ir.Function(self.module, main_func_type, name="main")
        main_block = main_func.append_basic_block(name="entry")
        
        self.builder = ir.IRBuilder(main_block)
        self.current_function = main_func
        self.current_block = main_block
        
        # Execute top-level statements
        for statement in program.statements:
            if not isinstance(statement, Function):
                self._generate_statement(statement)
        
        # Add return statement
        self.builder.ret_void()
    
    def _generate_function(self, func: Function):
        """Generate LLVM IR for a function definition."""
        # Get the existing function declaration
        if func.name not in self.functions:
            self._generate_function_declaration(func)
        
        llvm_func = self.functions[func.name]
        
        # Create entry block
        entry_block = llvm_func.append_basic_block(name="entry")
        
        # Generate function body
        old_builder = self.builder
        old_function = self.current_function
        old_block = self.current_block
        old_local_vars = self.local_vars.copy()
        
        self.builder = ir.IRBuilder(entry_block)
        self.current_function = llvm_func
        self.current_block = entry_block
        self.local_vars = {}  # Clear local variables for new function scope
        
        # Bind parameters
        for i, param in enumerate(func.parameters):
            param_var = self.builder.alloca(self.type_map['number'], name=param.name)
            self.builder.store(llvm_func.args[i], param_var)
            self.local_vars[param.name] = param_var
        
        # Generate function body
        for statement in func.body.statements:
            self._generate_statement(statement)
        
        # Add return if no explicit return
        if not any(isinstance(stmt, ReturnStatement) for stmt in func.body.statements):
            return_type = self.type_map['void']
            if func.return_type:
                return_type = self.type_map.get(func.return_type, self.type_map['number'])
            
            if return_type == self.type_map['void']:
                self.builder.ret_void()
            else:
                # Return 0 for non-void functions
                zero = ir.Constant(self.type_map['number'], 0.0)
                self.builder.ret(zero)
        
        # Restore context
        self.builder = old_builder
        self.current_function = old_function
        self.current_block = old_block
        self.local_vars = old_local_vars
    
    def _generate_statement(self, stmt: Statement):
        """Generate LLVM IR for a statement."""
        if isinstance(stmt, Assignment):
            self._generate_assignment(stmt)
        elif isinstance(stmt, ConstantAssignment):
            self._generate_constant_assignment(stmt)
        elif isinstance(stmt, IfStatement):
            self._generate_if_statement(stmt)
        elif isinstance(stmt, WhileLoop):
            self._generate_while_loop(stmt)
        elif isinstance(stmt, ReturnStatement):
            self._generate_return_statement(stmt)
        elif isinstance(stmt, Expression):
            self._generate_expression(stmt)
    
    def _generate_assignment(self, assignment: Assignment):
        """Generate LLVM IR for variable assignment."""
        value = self._generate_expression(assignment.value)
        
        if assignment.variable in self.local_vars:
            # Update existing local variable
            self.builder.store(value, self.local_vars[assignment.variable])
        elif assignment.variable in self.global_vars:
            # Update existing global variable
            self.builder.store(value, self.global_vars[assignment.variable])
        else:
            # Create new variable (global if not in function, local if in function)
            if self.current_function is None:
                # Global variable
                var = ir.GlobalVariable(self.module, value.type, assignment.variable)
                var.initializer = value
                self.global_vars[assignment.variable] = var
            else:
                # Local variable
                var = self.builder.alloca(value.type, name=assignment.variable)
                self.builder.store(value, var)
                self.local_vars[assignment.variable] = var
    
    def _generate_constant_assignment(self, assignment: ConstantAssignment):
        """Generate LLVM IR for constant assignment."""
        # For now, treat constants the same as variables
        self._generate_assignment(assignment)
    
    def _generate_if_statement(self, if_stmt: IfStatement):
        """Generate LLVM IR for if statement."""
        condition = self._generate_expression(if_stmt.condition)
        
        # Create blocks
        then_block = self.current_function.append_basic_block(name="then")
        else_block = self.current_function.append_basic_block(name="else") if if_stmt.else_block else None
        merge_block = self.current_function.append_basic_block(name="merge")
        
        # Branch based on condition
        if else_block:
            self.builder.cbranch(condition, then_block, else_block)
        else:
            self.builder.cbranch(condition, then_block, merge_block)
        
        # Generate then block
        self.builder.position_at_end(then_block)
        for stmt in if_stmt.then_block.statements:
            self._generate_statement(stmt)
        self.builder.branch(merge_block)
        
        # Generate else block if exists
        if else_block:
            self.builder.position_at_end(else_block)
            for stmt in if_stmt.else_block.statements:
                self._generate_statement(stmt)
            self.builder.branch(merge_block)
        
        # Continue in merge block
        self.builder.position_at_end(merge_block)
    
    def _generate_while_loop(self, while_loop: WhileLoop):
        """Generate LLVM IR for while loop."""
        # Create blocks
        condition_block = self.current_function.append_basic_block(name="while_condition")
        body_block = self.current_function.append_basic_block(name="while_body")
        exit_block = self.current_function.append_basic_block(name="while_exit")
        
        # Jump to condition block
        self.builder.branch(condition_block)
        
        # Generate condition block
        self.builder.position_at_end(condition_block)
        condition = self._generate_expression(while_loop.condition)
        self.builder.cbranch(condition, body_block, exit_block)
        
        # Generate body block
        self.builder.position_at_end(body_block)
        for stmt in while_loop.body.statements:
            self._generate_statement(stmt)
        self.builder.branch(condition_block)
        
        # Continue in exit block
        self.builder.position_at_end(exit_block)
    
    def _generate_return_statement(self, return_stmt: ReturnStatement):
        """Generate LLVM IR for return statement."""
        if return_stmt.value:
            value = self._generate_expression(return_stmt.value)
            self.builder.ret(value)
        else:
            self.builder.ret_void()
    
    def _generate_expression(self, expr: Expression) -> ir.Value:
        """Generate LLVM IR for an expression."""
        if isinstance(expr, Literal):
            return self._generate_literal(expr)
        elif isinstance(expr, Identifier):
            return self._generate_identifier(expr)
        elif isinstance(expr, BinaryOp):
            return self._generate_binary_op(expr)
        elif isinstance(expr, UnaryOp):
            return self._generate_unary_op(expr)
        elif isinstance(expr, Call):
            return self._generate_call(expr)
        else:
            raise RuntimeError(f"Unsupported expression type: {type(expr)}")
    
    def _generate_literal(self, literal: Literal) -> ir.Value:
        """Generate LLVM IR for a literal."""
        if literal.literal_type == "number":
            return ir.Constant(self.type_map['number'], float(literal.value))
        elif literal.literal_type == "boolean":
            return ir.Constant(self.type_map['boolean'], bool(literal.value))
        elif literal.literal_type == "string":
            return self._get_string_constant(str(literal.value))
        else:
            raise RuntimeError(f"Unsupported literal type: {literal.literal_type}")
    
    def _generate_identifier(self, identifier: Identifier) -> ir.Value:
        """Generate LLVM IR for an identifier."""
        if identifier.name in self.local_vars:
            return self.builder.load(self.local_vars[identifier.name])
        elif identifier.name in self.global_vars:
            return self.builder.load(self.global_vars[identifier.name])
        else:
            raise RuntimeError(f"Undefined variable: {identifier.name}")
    
    def _generate_binary_op(self, op: BinaryOp) -> ir.Value:
        """Generate LLVM IR for binary operation."""
        left = self._generate_expression(op.left)
        right = self._generate_expression(op.right)
        
        # Handle string concatenation
        if op.operator == '+' and left.type == self.type_map['string'] and right.type == self.type_map['string']:
            # For now, just return the left string (simplified)
            return left
        
        if op.operator == '+':
            if left.type == self.type_map['number'] and right.type == self.type_map['number']:
                return self.builder.fadd(left, right)
            else:
                # String concatenation - simplified for now
                return left
        elif op.operator == '-':
            return self.builder.fsub(left, right)
        elif op.operator == '*':
            return self.builder.fmul(left, right)
        elif op.operator == '/':
            return self.builder.fdiv(left, right)
        elif op.operator == '%':
            return self.builder.frem(left, right)
        elif op.operator == '**':
            # Power operation - use external function
            return self._call_pow_function(left, right)
        elif op.operator == '==':
            return self.builder.fcmp_ordered('==', left, right)
        elif op.operator == '!=':
            return self.builder.fcmp_ordered('!=', left, right)
        elif op.operator == '<':
            return self.builder.fcmp_ordered('<', left, right)
        elif op.operator == '<=':
            return self.builder.fcmp_ordered('<=', left, right)
        elif op.operator == '>':
            return self.builder.fcmp_ordered('>', left, right)
        elif op.operator == '>=':
            return self.builder.fcmp_ordered('>=', left, right)
        elif op.operator == '&&':
            # Logical AND
            return self.builder.and_(self._bool_to_int(left), self._bool_to_int(right))
        elif op.operator == '\|':
            # Logical OR
            return self.builder.or_(self._bool_to_int(left), self._bool_to_int(right))
        else:
            raise RuntimeError(f"Unsupported binary operator: {op.operator}")
    
    def _generate_unary_op(self, op: UnaryOp) -> ir.Value:
        """Generate LLVM IR for unary operation."""
        operand = self._generate_expression(op.operand)
        
        if op.operator == '-':
            zero = ir.Constant(self.type_map['number'], 0.0)
            return self.builder.fsub(zero, operand)
        elif op.operator == '!':
            # Logical NOT
            return self.builder.not_(self._bool_to_int(operand))
        else:
            raise RuntimeError(f"Unsupported unary operator: {op.operator}")
    
    def _generate_call(self, call: Call) -> ir.Value:
        """Generate LLVM IR for function call."""
        if isinstance(call.function, Identifier):
            func_name = call.function.name
            
            # Handle built-in functions
            if func_name == 'print':
                return self._generate_print_call(call.arguments)
            elif func_name == 'str':
                return self._generate_str_call(call.arguments)
            elif func_name in self.functions:
                # Call user-defined function
                args = [self._generate_expression(arg) for arg in call.arguments]
                return self.builder.call(self.functions[func_name], args)
            else:
                raise RuntimeError(f"Unknown function: {func_name}")
        else:
            raise RuntimeError("Complex function calls not supported")
    
    def _generate_str_call(self, arguments: List[Expression]) -> ir.Value:
        """Generate LLVM IR for str function call."""
        if len(arguments) != 1:
            raise RuntimeError("str() takes exactly one argument")
        
        arg = self._generate_expression(arguments[0])
        
        # Convert number to string representation
        if arg.type == self.type_map['number']:
            # Create a temporary buffer for the string conversion
            # For now, create a simple string representation
            # In a full implementation, we'd use sprintf or similar
            return self._get_string_constant("number")
        elif arg.type == self.type_map['string']:
            return arg
        else:
            # Default to string representation
            return self._get_string_constant("value")
    
    def _generate_print_call(self, arguments: List[Expression]) -> ir.Value:
        """Generate LLVM IR for print function call."""
        # Create printf function declaration if it doesn't exist
        if "printf" not in self.functions:
            printf_type = ir.FunctionType(ir.IntType(32), [ir.IntType(8).as_pointer()], var_arg=True)
            printf_func = ir.Function(self.module, printf_type, name="printf")
            self.functions["printf"] = printf_func
        else:
            printf_func = self.functions["printf"]
        
        if len(arguments) == 1:
            arg = self._generate_expression(arguments[0])
            
            # Check if argument is a string or number
            if arg.type == self.type_map['string']:
                # For strings, use %s format
                format_str = self._get_string_constant("%s\n")
                return self.builder.call(printf_func, [format_str, arg])
            else:
                # For numbers, use %f format
                format_str = self._get_string_constant("%f\n")
                return self.builder.call(printf_func, [format_str, arg])
        else:
            # Multiple arguments - use %s for first argument
            arg = self._generate_expression(arguments[0])
            format_str = self._get_string_constant("%s\n")
            return self.builder.call(printf_func, [format_str, arg])
    
    def _get_string_constant(self, string: str) -> ir.Value:
        """Get or create a string constant."""
        if string not in self.string_constants:
            # Create global string constant
            const_name = f"str_{self.string_counter}"
            self.string_counter += 1
            
            # Create null-terminated string
            null_term = string + '\0'
            const = ir.Constant(ir.ArrayType(ir.IntType(8), len(null_term)), 
                               bytearray(null_term.encode('utf-8')))
            global_var = ir.GlobalVariable(self.module, const.type, const_name)
            global_var.initializer = const
            global_var.global_constant = True
            
            self.string_constants[string] = global_var
        
        # Return pointer to string - handle case where builder might be None
        if self.builder is not None:
            return self.builder.gep(self.string_constants[string], 
                                   [ir.Constant(ir.IntType(32), 0), 
                                    ir.Constant(ir.IntType(32), 0)])
        else:
            # Return the global variable directly if no builder (for global initialization)
            return self.string_constants[string]
    
    def _bool_to_int(self, value: ir.Value) -> ir.Value:
        """Convert boolean value to integer."""
        if value.type == self.type_map['boolean']:
            return self.builder.zext(value, ir.IntType(32))
        return value
    
    def _call_pow_function(self, base: ir.Value, exponent: ir.Value) -> ir.Value:
        """Call pow function for exponentiation."""
        # Create pow function declaration
        pow_type = ir.FunctionType(ir.DoubleType(), [ir.DoubleType(), ir.DoubleType()])
        pow_func = ir.Function(self.module, pow_type, name="pow")
        
        return self.builder.call(pow_func, [base, exponent])


def compile_to_llvm_ir(source: str) -> str:
    """Compile Cortex source to LLVM IR."""
    from .parser import Parser
    
    # Parse the source
    parser = Parser(source)
    program = parser.parse()
    
    # Generate LLVM IR
    generator = CodeGenerator()
    return generator.generate(program)
