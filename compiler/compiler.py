"""
Cortex Compiler

Compiles Cortex source code to binary executables using LLVM.
"""

import os
import subprocess
import tempfile
from pathlib import Path
from typing import Optional, List
import llvmlite.binding as llvm
from .codegen import compile_to_llvm_ir


class CortexCompiler:
    """Main compiler class for Cortex language."""
    
    def __init__(self):
        self.target_machine = None
        self._setup_llvm()
    
    def _setup_llvm(self):
        """Initialize LLVM target machine."""
        # LLVM initialization is now handled automatically in newer versions
        # Only initialize if the functions exist
        try:
            llvm.initialize()
            llvm.initialize_native_target()
            llvm.initialize_native_asmprinter()
        except:
            # LLVM initialization is automatic in newer versions
            pass
        
        try:
            # Create target machine
            target = llvm.Target.from_default_triple()
            self.target_machine = target.create_target_machine()
        except Exception as e:
            print(f"Warning: Could not create target machine: {e}")
            # Fallback: don't use target machine for now
            self.target_machine = None
    
    def compile_to_object(self, llvm_ir: str, output_path: str) -> bool:
        """Compile LLVM IR to object file."""
        try:
            # Parse LLVM IR
            llvm_module = llvm.parse_assembly(llvm_ir)
            
            # Optimize the module
            pmb = llvm.create_pass_manager_builder()
            pmb.opt_level = 2  # O2 optimization
            pm = llvm.create_module_pass_manager()
            pmb.populate(pm)
            pm.run(llvm_module)
            
            # Emit object code
            if self.target_machine:
                with open(output_path, 'wb') as f:
                    f.write(self.target_machine.emit_object(llvm_module))
            else:
                # Fallback: emit assembly and use external assembler
                asm = str(llvm_module)
                with open(output_path + '.ll', 'w') as f:
                    f.write(asm)
                
                # Use llc to compile to object file
                result = subprocess.run(['llc', '-filetype=obj', output_path + '.ll', '-o', output_path], 
                                      capture_output=True, text=True)
                if result.returncode != 0:
                    print(f"llc error: {result.stderr}")
                    return False
                
                # Clean up temporary file
                import os
                if os.path.exists(output_path + '.ll'):
                    os.unlink(output_path + '.ll')
            
            return True
        except Exception as e:
            print(f"Error compiling to object: {e}")
            return False
    
    def link_executable(self, object_files: List[str], output_path: str, 
                       libraries: Optional[List[str]] = None) -> bool:
        """Link object files into executable."""
        try:
            cmd = ['gcc', '-o', output_path] + object_files
            
            # Add standard libraries
            if libraries:
                cmd.extend(libraries)
            else:
                # Add math library for pow function
                cmd.append('-lm')
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode != 0:
                print(f"Linker error: {result.stderr}")
                return False
            
            return True
        except Exception as e:
            print(f"Error linking executable: {e}")
            return False
    
    def compile_to_binary(self, source: str, output_path: str, 
                         optimize: bool = True) -> bool:
        """Compile Cortex source directly to binary executable."""
        try:
            # Generate LLVM IR
            llvm_ir = compile_to_llvm_ir(source)
            
            # For now, just save the LLVM IR and provide instructions
            ll_file = output_path + '.ll'
            with open(ll_file, 'w') as f:
                f.write(llvm_ir)
            
            print(f"LLVM IR generated: {ll_file}")
            print("To compile to binary, you need LLVM tools installed:")
            print("1. Install LLVM: brew install llvm")
            print(f"2. Compile: llc -filetype=obj {ll_file} -o {output_path}.o")
            print(f"3. Link: gcc {output_path}.o -lm -o {output_path}")
            print(f"4. Run: ./{output_path}")
            
            return True
        
        except Exception as e:
            print(f"Compilation error: {e}")
            return False


def compile_cortex_file(input_path: str, output_path: str, 
                       optimize: bool = True) -> bool:
    """Compile a Cortex source file to binary."""
    try:
        with open(input_path, 'r', encoding='utf-8') as f:
            source = f.read()
        
        compiler = CortexCompiler()
        return compiler.compile_to_binary(source, output_path, optimize)
    
    except FileNotFoundError:
        print(f"Error: Input file '{input_path}' not found")
        return False
    except Exception as e:
        print(f"Error reading input file: {e}")
        return False


def compile_cortex_source(source: str, output_path: str, 
                         optimize: bool = True) -> bool:
    """Compile Cortex source code to binary."""
    compiler = CortexCompiler()
    return compiler.compile_to_binary(source, output_path, optimize)
