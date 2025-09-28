#!/usr/bin/env python3
"""
Debug script for Cortex compiler.
"""

import sys
from pathlib import Path

# Add the project root to the Python path
project_root = Path(__file__).parent
sys.path.insert(0, str(project_root))

def debug_compiler():
    """Debug the compiler step by step."""
    try:
        from compiler.parser import Parser
        from compiler.codegen import compile_to_llvm_ir
        
        # Simple test source
        source = """
        func main[] |
          let x := 5
          print[x]
        ^
        
        main[]
        """
        
        print("=== DEBUGGING CORTEX COMPILER ===")
        print("Source code:")
        print(source)
        print("\n" + "="*50)
        
        # Parse the source
        print("1. Parsing source...")
        parser = Parser(source)
        program = parser.parse()
        print("✅ Parsing successful")
        
        # Show AST structure
        print("\n2. AST structure:")
        for i, stmt in enumerate(program.statements):
            print(f"  Statement {i}: {type(stmt).__name__}")
            if hasattr(stmt, 'name'):
                print(f"    Name: {stmt.name}")
        
        print("\n3. Generating LLVM IR...")
        llvm_ir = compile_to_llvm_ir(source)
        print("✅ LLVM IR generation successful")
        
        print("\n4. Generated LLVM IR:")
        print("-" * 50)
        print(llvm_ir)
        print("-" * 50)
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    debug_compiler()
