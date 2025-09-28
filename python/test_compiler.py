#!/usr/bin/env python3
"""
Test script for Cortex compiler functionality.
"""

import sys
import os
from pathlib import Path

# Add the project root to the Python path
project_root = Path(__file__).parent
sys.path.insert(0, str(project_root))

def test_compiler():
    """Test the basic compiler functionality."""
    try:
        from compiler.parser import Parser
        from compiler.codegen import compile_to_llvm_ir
        
        # Test source code
        source = """
        func main[] |
          let x := 10
          let y := 20
          let sum := x + y
          print["Sum: " + str[sum]]
        ^
        
        main[]
        """
        
        print("Testing Cortex compiler...")
        print("=" * 50)
        
        # Parse the source
        parser = Parser(source)
        program = parser.parse()
        print("✅ Parsing successful")
        
        # Generate LLVM IR
        llvm_ir = compile_to_llvm_ir(source)
        print("✅ LLVM IR generation successful")
        print(f"Generated {len(llvm_ir.split(chr(10)))} lines of LLVM IR")
        
        # Show a snippet of the generated IR
        lines = llvm_ir.split('\n')[:10]
        print("\nLLVM IR snippet:")
        for line in lines:
            if line.strip():
                print(f"  {line}")
        
        print("\n✅ Compiler test completed successfully!")
        return True
        
    except ImportError as e:
        print(f"❌ Import error: {e}")
        print("Make sure to install required dependencies:")
        print("  pip install llvmlite")
        return False
    except Exception as e:
        print(f"❌ Compiler test failed: {e}")
        return False

def test_examples():
    """Test compiling example files."""
    examples_dir = Path("../docs/shared/examples")
    if not examples_dir.exists():
        print("❌ Examples directory not found")
        return False
    
    example_files = ["simple_arithmetic.ctx", "simple_loops.ctx", "hello_world.ctx", "function_demo.ctx"]
    
    print("\nTesting example compilation...")
    print("=" * 50)
    
    for example_file in example_files:
        example_path = examples_dir / example_file
        if example_path.exists():
            try:
                with open(example_path, 'r') as f:
                    source = f.read()
                
                from compiler.parser import Parser
                from compiler.codegen import compile_to_llvm_ir
                
                parser = Parser(source)
                program = parser.parse()
                llvm_ir = compile_to_llvm_ir(source)
                
                print(f"✅ {example_file} - compiled successfully")
            except Exception as e:
                print(f"❌ {example_file} - compilation failed: {e}")
                return False
        else:
            print(f"❌ {example_file} - file not found")
            return False
    
    print("\n✅ All examples compiled successfully!")
    return True

if __name__ == "__main__":
    print("Cortex Compiler Test Suite")
    print("=" * 60)
    
    success = True
    
    # Test basic compiler functionality
    success &= test_compiler()
    
    # Test example files
    success &= test_examples()
    
    print("\n" + "=" * 60)
    if success:
        print("🎉 All tests passed! Cortex compiler is working correctly.")
        print("\nNext steps:")
        print("1. Install dependencies: pip install -r requirements.txt")
        print("2. Compile examples: python cortexc.py build examples/arithmetic.ctx -o arithmetic")
        print("3. Run compiled binary: ./arithmetic")
    else:
        print("❌ Some tests failed. Check the errors above.")
        sys.exit(1)
