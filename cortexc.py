#!/usr/bin/env python3
"""
Cortex Compiler and Interpreter

Command-line interface for the Cortex programming language.
"""

import sys
import os
import argparse
from pathlib import Path

# Add the project root to the Python path
project_root = Path(__file__).parent
sys.path.insert(0, str(project_root))

from compiler.parser import Parser, ParseError
from compiler.interpreter import run_cortex_program


def main():
    """Main entry point for the Cortex compiler/interpreter."""
    parser = argparse.ArgumentParser(
        description="Cortex Programming Language Compiler and Interpreter",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  cortexc run hello_world.ctx          # Run a Cortex program
  cortexc build hello_world.ctx -o hello  # Compile to executable (future)
  cortexc repl                         # Start interactive REPL
        """
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Run command
    run_parser = subparsers.add_parser('run', help='Run a Cortex program')
    run_parser.add_argument('file', help='Cortex source file (.ctx)')
    run_parser.add_argument('-v', '--verbose', action='store_true', help='Verbose output')
    
    # Build command (placeholder for future)
    build_parser = subparsers.add_parser('build', help='Compile a Cortex program')
    build_parser.add_argument('file', help='Cortex source file (.ctx)')
    build_parser.add_argument('-o', '--output', help='Output executable name')
    build_parser.add_argument('-O', '--optimize', choices=['0', '1', '2', '3'], 
                             default='1', help='Optimization level')
    build_parser.add_argument('-g', '--debug', action='store_true', help='Include debug info')
    build_parser.add_argument('-v', '--verbose', action='store_true', help='Verbose output')
    
    # REPL command
    repl_parser = subparsers.add_parser('repl', help='Start interactive REPL')
    repl_parser.add_argument('-v', '--verbose', action='store_true', help='Verbose output')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return 1
    
    try:
        if args.command == 'run':
            return run_command(args)
        elif args.command == 'build':
            return build_command(args)
        elif args.command == 'repl':
            return repl_command(args)
        else:
            parser.print_help()
            return 1
    
    except KeyboardInterrupt:
        print("\nInterrupted by user")
        return 1
    except Exception as e:
        print(f"Error: {e}")
        return 1


def run_command(args):
    """Run a Cortex program."""
    if not os.path.exists(args.file):
        print(f"Error: File '{args.file}' not found")
        return 1
    
    if not args.file.endswith('.ctx'):
        print(f"Error: File '{args.file}' is not a Cortex source file (.ctx)")
        return 1
    
    try:
        with open(args.file, 'r', encoding='utf-8') as f:
            source = f.read()
        
        if args.verbose:
            print(f"Running Cortex program: {args.file}")
            print("=" * 50)
        
        result = run_cortex_program(source)
        
        if args.verbose and result is not None:
            print("=" * 50)
            print(f"Program returned: {result}")
        
        return 0
    
    except FileNotFoundError:
        print(f"Error: File '{args.file}' not found")
        return 1
    except Exception as e:
        print(f"Error running program: {e}")
        return 1


def build_command(args):
    """Build a Cortex program (placeholder for future LLVM compilation)."""
    print("Build command not yet implemented.")
    print("Currently, Cortex programs are interpreted, not compiled.")
    print("Use 'cortexc run' to execute programs.")
    return 0


def repl_command(args):
    """Start an interactive REPL."""
    print("Cortex Interactive REPL")
    print("Type 'exit' or 'quit' to exit, 'help' for help")
    print("=" * 50)
    
    # Simple REPL implementation
    while True:
        try:
            line = input("cortex> ").strip()
            
            if line.lower() in ['exit', 'quit']:
                print("Goodbye!")
                break
            elif line.lower() == 'help':
                print_help()
                continue
            elif not line:
                continue
            
            # Try to execute the line
            try:
                result = run_cortex_program(line)
                if result is not None:
                    print(f"Result: {result}")
            except Exception as e:
                print(f"Error: {e}")
        
        except KeyboardInterrupt:
            print("\nUse 'exit' or 'quit' to exit")
        except EOFError:
            print("\nGoodbye!")
            break
    
    return 0


def print_help():
    """Print REPL help."""
    print("""
Cortex REPL Help:
  exit, quit    - Exit the REPL
  help          - Show this help
  let x := 42   - Define a variable
  print["hello"] - Print a message
  func add[a, b] | return[a + b] ^ - Define a function
  
Examples:
  let x := 10
  let y := 20
  let sum := x + y
  print["Sum: " + str[sum]]
    """)


if __name__ == '__main__':
    sys.exit(main())
