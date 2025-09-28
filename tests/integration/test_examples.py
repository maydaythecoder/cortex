"""
Integration tests for Cortex examples.
"""

import unittest
import os
import sys
import subprocess

# Add the project root to the path
project_root = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))
sys.path.insert(0, project_root)


class TestCortexExamples(unittest.TestCase):
    """Integration tests for Cortex examples."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.examples_dir = os.path.join(project_root, 'examples')
        self.compiler_path = os.path.join(project_root, 'compiler')
    
    def test_linear_regression_syntax(self):
        """Test that linear regression example has valid syntax."""
        example_path = os.path.join(self.examples_dir, 'linear_regression.ctx')
        
        # Check that the file exists
        self.assertTrue(os.path.exists(example_path), "Linear regression example not found")
        
        # Read and validate basic syntax
        with open(example_path, 'r') as f:
            content = f.read()
        
        # Check for basic Cortex syntax elements
        self.assertIn('let', content, "Missing 'let' keyword")
        self.assertIn('func', content, "Missing 'func' keyword")
        self.assertIn('|', content, "Missing block delimiter")
        self.assertIn('^', content, "Missing block delimiter")
        self.assertIn('[', content, "Missing bracket syntax")
        self.assertIn(']', content, "Missing bracket syntax")
    
    def test_neural_network_syntax(self):
        """Test that neural network example has valid syntax."""
        example_path = os.path.join(self.examples_dir, 'neural_network.ctx')
        
        # Check that the file exists
        self.assertTrue(os.path.exists(example_path), "Neural network example not found")
        
        # Read and validate basic syntax
        with open(example_path, 'r') as f:
            content = f.read()
        
        # Check for basic Cortex syntax elements
        self.assertIn('struct', content, "Missing 'struct' keyword")
        self.assertIn('let', content, "Missing 'let' keyword")
        self.assertIn('func', content, "Missing 'func' keyword")
        self.assertIn('|', content, "Missing block delimiter")
        self.assertIn('^', content, "Missing block delimiter")
    
    def test_gradient_descent_syntax(self):
        """Test that gradient descent example has valid syntax."""
        example_path = os.path.join(self.examples_dir, 'gradient_descent.ctx')
        
        # Check that the file exists
        self.assertTrue(os.path.exists(example_path), "Gradient descent example not found")
        
        # Read and validate basic syntax
        with open(example_path, 'r') as f:
            content = f.read()
        
        # Check for basic Cortex syntax elements
        self.assertIn('let', content, "Missing 'let' keyword")
        self.assertIn('func', content, "Missing 'func' keyword")
        self.assertIn('|', content, "Missing block delimiter")
        self.assertIn('^', content, "Missing block delimiter")
        self.assertIn('for', content, "Missing 'for' loop")
    
    def test_data_processing_syntax(self):
        """Test that data processing example has valid syntax."""
        example_path = os.path.join(self.examples_dir, 'data_processing.ctx')
        
        # Check that the file exists
        self.assertTrue(os.path.exists(example_path), "Data processing example not found")
        
        # Read and validate basic syntax
        with open(example_path, 'r') as f:
            content = f.read()
        
        # Check for basic Cortex syntax elements
        self.assertIn('struct', content, "Missing 'struct' keyword")
        self.assertIn('let', content, "Missing 'let' keyword")
        self.assertIn('func', content, "Missing 'func' keyword")
        self.assertIn('|', content, "Missing block delimiter")
        self.assertIn('^', content, "Missing block delimiter")
    
    def test_example_completeness(self):
        """Test that examples are complete and well-formed."""
        examples = [
            'linear_regression.ctx',
            'neural_network.ctx',
            'gradient_descent.ctx',
            'data_processing.ctx'
        ]
        
        for example in examples:
            example_path = os.path.join(self.examples_dir, example)
            
            with open(example_path, 'r') as f:
                content = f.read()
            
            # Check for main function
            self.assertIn('func main[]', content, f"Missing main function in {example}")
            
            # Check for proper block structure
            pipe_count = content.count('|')
            caret_count = content.count('^')
            self.assertEqual(pipe_count, caret_count, 
                           f"Mismatched block delimiters in {example}")
            
            # Check for proper bracket structure
            left_bracket_count = content.count('[')
            right_bracket_count = content.count(']')
            self.assertEqual(left_bracket_count, right_bracket_count,
                           f"Mismatched brackets in {example}")
    
    def test_example_documentation(self):
        """Test that examples have proper documentation."""
        examples = [
            'linear_regression.ctx',
            'neural_network.ctx',
            'gradient_descent.ctx',
            'data_processing.ctx'
        ]
        
        for example in examples:
            example_path = os.path.join(self.examples_dir, example)
            
            with open(example_path, 'r') as f:
                content = f.read()
            
            # Check for comments
            self.assertIn('//', content, f"Missing comments in {example}")
            
            # Check for example description
            self.assertIn('Example', content, f"Missing example description in {example}")
    
    def test_example_constants(self):
        """Test that examples use proper constant declarations."""
        examples = [
            'linear_regression.ctx',
            'neural_network.ctx',
            'gradient_descent.ctx',
            'data_processing.ctx'
        ]
        
        for example in examples:
            example_path = os.path.join(self.examples_dir, example)
            
            with open(example_path, 'r') as f:
                content = f.read()
            
            # Check for constant declarations
            self.assertIn('::', content, f"Missing constant declarations in {example}")
    
    def test_example_ai_ml_features(self):
        """Test that examples demonstrate AI/ML features."""
        examples = [
            'linear_regression.ctx',
            'neural_network.ctx',
            'gradient_descent.ctx',
            'data_processing.ctx'
        ]
        
        for example in examples:
            example_path = os.path.join(self.examples_dir, example)
            
            with open(example_path, 'r') as f:
                content = f.read()
            
            # Check for AI/ML specific features
            ai_ml_features = ['tensor', 'matrix', 'gradient', 'loss', 'activation']
            found_features = [feature for feature in ai_ml_features if feature in content]
            
            self.assertGreater(len(found_features), 0, 
                             f"No AI/ML features found in {example}")
    
    def test_example_error_handling(self):
        """Test that examples include error handling."""
        examples = [
            'linear_regression.ctx',
            'neural_network.ctx',
            'gradient_descent.ctx',
            'data_processing.ctx'
        ]
        
        for example in examples:
            example_path = os.path.join(self.examples_dir, example)
            
            with open(example_path, 'r') as f:
                content = f.read()
            
            # Check for error handling patterns
            error_patterns = ['if [', 'guard [', 'try', 'catch']
            found_patterns = [pattern for pattern in error_patterns if pattern in content]
            
            # At least one example should have error handling
            if example == 'data_processing.ctx':
                self.assertGreater(len(found_patterns), 0,
                                 f"Missing error handling in {example}")


if __name__ == '__main__':
    unittest.main()
