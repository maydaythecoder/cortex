"""
Unit tests for the Cortex runtime.
"""

import unittest
import ctypes
import os
import sys

# Add the runtime directory to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../runtime'))

try:
    from cortex import *
except ImportError:
    # If the C library isn't built yet, skip these tests
    cortex = None


@unittest.skipIf(cortex is None, "Cortex runtime not available")
class TestCortexRuntime(unittest.TestCase):
    """Test cases for the Cortex runtime."""
    
    def setUp(self):
        """Set up test fixtures."""
        # Clear any previous errors
        cortex_clear_error()
    
    def test_tensor_creation(self):
        """Test tensor creation and basic properties."""
        shape = [2, 3]
        tensor = cortex_tensor_create(shape, 2)
        
        self.assertIsNotNone(tensor)
        self.assertEqual(tensor.ndim, 2)
        self.assertEqual(tensor.shape[0], 2)
        self.assertEqual(tensor.shape[1], 3)
        self.assertEqual(tensor.size, 6)
        self.assertFalse(tensor.requires_grad)
        
        cortex_tensor_free(tensor)
    
    def test_zeros_tensor(self):
        """Test creation of zero-filled tensor."""
        shape = [3, 4]
        tensor = cortex_zeros(shape, 2)
        
        self.assertIsNotNone(tensor)
        self.assertEqual(tensor.size, 12)
        
        # Check that all values are zero
        for i in range(tensor.size):
            self.assertEqual(tensor.data[i], 0.0)
        
        cortex_tensor_free(tensor)
    
    def test_ones_tensor(self):
        """Test creation of one-filled tensor."""
        shape = [2, 2]
        tensor = cortex_ones(shape, 2)
        
        self.assertIsNotNone(tensor)
        self.assertEqual(tensor.size, 4)
        
        # Check that all values are one
        for i in range(tensor.size):
            self.assertEqual(tensor.data[i], 1.0)
        
        cortex_tensor_free(tensor)
    
    def test_eye_tensor(self):
        """Test creation of identity matrix."""
        size = 3
        tensor = cortex_eye(size)
        
        self.assertIsNotNone(tensor)
        self.assertEqual(tensor.ndim, 2)
        self.assertEqual(tensor.shape[0], size)
        self.assertEqual(tensor.shape[1], size)
        
        # Check identity matrix properties
        for i in range(size):
            for j in range(size):
                expected = 1.0 if i == j else 0.0
                actual = tensor.data[i * size + j]
                self.assertEqual(actual, expected)
        
        cortex_tensor_free(tensor)
    
    def test_arange_tensor(self):
        """Test creation of arange tensor."""
        start, stop, step = 0.0, 5.0, 1.0
        tensor = cortex_arange(start, stop, step)
        
        self.assertIsNotNone(tensor)
        self.assertEqual(tensor.ndim, 1)
        self.assertEqual(tensor.size, 5)
        
        # Check values
        for i in range(tensor.size):
            expected = start + i * step
            self.assertEqual(tensor.data[i], expected)
        
        cortex_tensor_free(tensor)
    
    def test_tensor_operations(self):
        """Test basic tensor operations."""
        shape = [2, 2]
        a = cortex_ones(shape, 2)
        b = cortex_ones(shape, 2)
        
        # Test addition
        result = cortex_tensor_add(a, b)
        self.assertIsNotNone(result)
        for i in range(result.size):
            self.assertEqual(result.data[i], 2.0)
        cortex_tensor_free(result)
        
        # Test subtraction
        result = cortex_tensor_subtract(a, b)
        self.assertIsNotNone(result)
        for i in range(result.size):
            self.assertEqual(result.data[i], 0.0)
        cortex_tensor_free(result)
        
        # Test multiplication
        result = cortex_tensor_multiply(a, b)
        self.assertIsNotNone(result)
        for i in range(result.size):
            self.assertEqual(result.data[i], 1.0)
        cortex_tensor_free(result)
        
        cortex_tensor_free(a)
        cortex_tensor_free(b)
    
    def test_scalar_operations(self):
        """Test scalar operations on tensors."""
        shape = [2, 2]
        tensor = cortex_ones(shape, 2)
        
        # Test scalar addition
        result = cortex_tensor_add_scalar(tensor, 5.0)
        self.assertIsNotNone(result)
        for i in range(result.size):
            self.assertEqual(result.data[i], 6.0)
        cortex_tensor_free(result)
        
        # Test scalar multiplication
        result = cortex_tensor_multiply_scalar(tensor, 3.0)
        self.assertIsNotNone(result)
        for i in range(result.size):
            self.assertEqual(result.data[i], 3.0)
        cortex_tensor_free(result)
        
        cortex_tensor_free(tensor)
    
    def test_matrix_operations(self):
        """Test matrix operations."""
        # Create 2x2 matrices
        shape_a = [2, 2]
        shape_b = [2, 2]
        
        a = cortex_tensor_create(shape_a, 2)
        b = cortex_tensor_create(shape_b, 2)
        
        # Fill with test data
        a.data[0] = 1.0; a.data[1] = 2.0
        a.data[2] = 3.0; a.data[3] = 4.0
        
        b.data[0] = 5.0; b.data[1] = 6.0
        b.data[2] = 7.0; b.data[3] = 8.0
        
        # Test matrix multiplication
        result = cortex_tensor_matmul(a, b)
        self.assertIsNotNone(result)
        
        # Expected result: [[19, 22], [43, 50]]
        expected = [19.0, 22.0, 43.0, 50.0]
        for i in range(result.size):
            self.assertAlmostEqual(result.data[i], expected[i], places=6)
        
        cortex_tensor_free(result)
        
        # Test transpose
        result = cortex_tensor_transpose(a)
        self.assertIsNotNone(result)
        
        # Expected result: [[1, 3], [2, 4]]
        expected = [1.0, 3.0, 2.0, 4.0]
        for i in range(result.size):
            self.assertEqual(result.data[i], expected[i])
        
        cortex_tensor_free(result)
        
        cortex_tensor_free(a)
        cortex_tensor_free(b)
    
    def test_mathematical_functions(self):
        """Test mathematical functions."""
        shape = [2, 2]
        tensor = cortex_tensor_create(shape, 2)
        
        # Fill with test data
        tensor.data[0] = 1.0; tensor.data[1] = 4.0
        tensor.data[2] = 9.0; tensor.data[3] = 16.0
        
        # Test sqrt
        result = cortex_tensor_sqrt(tensor)
        self.assertIsNotNone(result)
        expected = [1.0, 2.0, 3.0, 4.0]
        for i in range(result.size):
            self.assertAlmostEqual(result.data[i], expected[i], places=6)
        cortex_tensor_free(result)
        
        # Test exp
        result = cortex_tensor_exp(tensor)
        self.assertIsNotNone(result)
        # exp(1) ≈ 2.718, exp(4) ≈ 54.598, etc.
        self.assertGreater(result.data[0], 2.7)
        self.assertGreater(result.data[1], 54.0)
        cortex_tensor_free(result)
        
        cortex_tensor_free(tensor)
    
    def test_activation_functions(self):
        """Test activation functions."""
        shape = [4]
        tensor = cortex_tensor_create(shape, 1)
        
        # Fill with test data: [-2, -1, 0, 1]
        tensor.data[0] = -2.0
        tensor.data[1] = -1.0
        tensor.data[2] = 0.0
        tensor.data[3] = 1.0
        
        # Test ReLU
        result = cortex_tensor_relu(tensor)
        self.assertIsNotNone(result)
        expected = [0.0, 0.0, 0.0, 1.0]
        for i in range(result.size):
            self.assertEqual(result.data[i], expected[i])
        cortex_tensor_free(result)
        
        # Test sigmoid
        result = cortex_tensor_sigmoid(tensor)
        self.assertIsNotNone(result)
        # sigmoid(0) = 0.5, sigmoid(1) ≈ 0.731
        self.assertAlmostEqual(result.data[2], 0.5, places=6)
        self.assertGreater(result.data[3], 0.7)
        cortex_tensor_free(result)
        
        cortex_tensor_free(tensor)
    
    def test_statistical_functions(self):
        """Test statistical functions."""
        shape = [4]
        tensor = cortex_tensor_create(shape, 1)
        
        # Fill with test data: [1, 2, 3, 4]
        for i in range(4):
            tensor.data[i] = i + 1.0
        
        # Test mean
        mean_val = cortex_tensor_mean(tensor)
        self.assertAlmostEqual(mean_val, 2.5, places=6)
        
        # Test sum
        sum_val = cortex_tensor_sum(tensor)
        self.assertEqual(sum_val, 10.0)
        
        # Test min/max
        min_val = cortex_tensor_min(tensor)
        max_val = cortex_tensor_max(tensor)
        self.assertEqual(min_val, 1.0)
        self.assertEqual(max_val, 4.0)
        
        cortex_tensor_free(tensor)
    
    def test_loss_functions(self):
        """Test loss functions."""
        shape = [3]
        predictions = cortex_tensor_create(shape, 1)
        targets = cortex_tensor_create(shape, 1)
        
        # Fill with test data
        predictions.data[0] = 0.8; predictions.data[1] = 0.2; predictions.data[2] = 0.9
        targets.data[0] = 1.0; targets.data[1] = 0.0; targets.data[2] = 1.0
        
        # Test MSE loss
        mse_loss = cortex_mse_loss(predictions, targets)
        self.assertGreater(mse_loss, 0.0)
        
        # Test cross-entropy loss
        ce_loss = cortex_cross_entropy_loss(predictions, targets)
        self.assertGreater(ce_loss, 0.0)
        
        cortex_tensor_free(predictions)
        cortex_tensor_free(targets)
    
    def test_error_handling(self):
        """Test error handling."""
        # Test NULL tensor operations
        result = cortex_tensor_add(None, None)
        self.assertIsNone(result)
        self.assertIsNotNone(cortex_get_error())
        
        cortex_clear_error()
        
        # Test division by zero
        shape = [2]
        a = cortex_ones(shape, 1)
        b = cortex_zeros(shape, 1)
        
        result = cortex_tensor_divide(a, b)
        self.assertIsNone(result)
        self.assertIsNotNone(cortex_get_error())
        
        cortex_tensor_free(a)
        cortex_tensor_free(b)
    
    def test_tensor_copy(self):
        """Test tensor copying."""
        shape = [2, 2]
        original = cortex_tensor_create(shape, 2)
        
        # Fill with test data
        for i in range(original.size):
            original.data[i] = i + 1.0
        
        # Create copy
        copy = cortex_tensor_copy(original)
        self.assertIsNotNone(copy)
        
        # Check that data is the same
        for i in range(original.size):
            self.assertEqual(original.data[i], copy.data[i])
        
        # Modify original
        original.data[0] = 999.0
        
        # Check that copy is unchanged
        self.assertNotEqual(original.data[0], copy.data[0])
        self.assertEqual(copy.data[0], 1.0)
        
        cortex_tensor_free(original)
        cortex_tensor_free(copy)


if __name__ == '__main__':
    unittest.main()
