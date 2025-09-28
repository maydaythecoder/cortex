"""
Cortex Compiler Package

This package contains the compiler components for the Cortex programming language.
The compiler is written in Python and targets LLVM for code generation.
"""

__version__ = "0.1.0"
__author__ = "Cortex Team"

from .parser import Parser
from .ast import ASTNode, Program, Function, Variable, Expression
from .interpreter import Interpreter, run_cortex_program

__all__ = [
    "Parser",
    "ASTNode", "Program", "Function", "Variable", "Expression",
    "Interpreter", "run_cortex_program"
]
