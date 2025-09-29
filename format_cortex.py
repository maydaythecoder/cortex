#!/usr/bin/env python3
"""
Cortex Code Formatter

Formats Cortex source code with consistent indentation, spacing, and structure.
"""

import sys
import re
import argparse
from pathlib import Path


class CortexFormatter:
    def __init__(self, indent_size=2):
        self.indent_size = indent_size
        self.current_indent = 0
        
    def format_file(self, file_path):
        """Format a Cortex file."""
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        formatted = self.format_content(content)
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(formatted)
        
        print(f"Formatted: {file_path}")
    
    def format_content(self, content):
        """Format Cortex source content."""
        lines = content.split('\n')
        formatted_lines = []
        self.current_indent = 0
        
        for line in lines:
            formatted_line = self.format_line(line.strip())
            if formatted_line:
                formatted_lines.append(formatted_line)
            elif not line.strip():  # Preserve empty lines
                formatted_lines.append('')
        
        return '\n'.join(formatted_lines)
    
    def format_line(self, line):
        """Format a single line of Cortex code."""
        if not line or line.startswith('//'):
            return line
        
        # Handle function definitions
        if line.startswith('func '):
            return self.format_function_definition(line)
        
        # Handle function calls
        if '[' in line and ']' in line and not line.startswith('//'):
            return self.format_function_call(line)
        
        # Handle variable assignments
        if ' := ' in line:
            return self.format_assignment(line)
        
        # Handle control structures
        if line.startswith(('if ', 'while ', 'for ')):
            return self.format_control_structure(line)
        
        # Handle closing brackets
        if line == '^':
            self.current_indent = max(0, self.current_indent - self.indent_size)
            return ' ' * self.current_indent + line
        
        # Handle return statements
        if line.startswith('return'):
            return self.format_return(line)
        
        # Default formatting
        return ' ' * self.current_indent + line
    
    def format_function_definition(self, line):
        """Format function definition."""
        # func name[params] |
        formatted = ' ' * self.current_indent + line
        self.current_indent += self.indent_size
        return formatted
    
    def format_function_call(self, line):
        """Format function call."""
        # Add proper spacing around brackets
        line = re.sub(r'\s*\[\s*', '[', line)
        line = re.sub(r'\s*\]\s*', ']', line)
        return ' ' * self.current_indent + line
    
    def format_assignment(self, line):
        """Format variable assignment."""
        # let var := value
        parts = line.split(' := ')
        if len(parts) == 2:
            formatted = f"{parts[0]} := {parts[1]}"
            return ' ' * self.current_indent + formatted
        return ' ' * self.current_indent + line
    
    def format_control_structure(self, line):
        """Format control structures."""
        # if [condition] |, while [condition] |
        formatted = ' ' * self.current_indent + line
        self.current_indent += self.indent_size
        return formatted
    
    def format_return(self, line):
        """Format return statement."""
        return ' ' * self.current_indent + line


def main():
    parser = argparse.ArgumentParser(description='Format Cortex source files')
    parser.add_argument('files', nargs='+', help='Cortex files to format')
    parser.add_argument('--indent', type=int, default=2, help='Indentation size (default: 2)')
    parser.add_argument('--check', action='store_true', help='Check formatting without modifying files')
    
    args = parser.parse_args()
    
    formatter = CortexFormatter(indent_size=args.indent)
    
    for file_path in args.files:
        if not file_path.endswith('.ctx'):
            print(f"Skipping non-Cortex file: {file_path}")
            continue
        
        if not Path(file_path).exists():
            print(f"File not found: {file_path}")
            continue
        
        if args.check:
            # Check mode - just validate formatting
            with open(file_path, 'r', encoding='utf-8') as f:
                original = f.read()
            
            formatted = formatter.format_content(original)
            
            if original != formatted:
                print(f"Needs formatting: {file_path}")
            else:
                print(f"Already formatted: {file_path}")
        else:
            # Format mode - modify files
            formatter.format_file(file_path)


if __name__ == '__main__':
    main()
