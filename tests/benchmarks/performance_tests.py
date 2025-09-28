"""
Performance benchmarks for Cortex runtime.
"""

import unittest
import time
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
class TestCortexPerformance(unittest.TestCase):
    """Performance benchmarks for Cortex runtime."""
    
    def setUp(self):
        """Set up test fixtures."""
        cortex_clear_error()
    
    def benchmark_tensor_creation(self):
        """Benchmark tensor creation performance."""
        sizes = [100, 1000, 10000]
        results = {}
        
        for size in sizes:
            shape = [size, size]
            
            start_time = time.time()
            tensor = cortex_tensor_create(shape, 2)
            creation_time = time.time() - start_time
            
            if tensor:
                results[size] = creation_time
                cortex_tensor_free(tensor)
            else:
                results[size] = float('inf')
        
        return results
    
    def benchmark_tensor_operations(self):
        """Benchmark tensor operation performance."""
        sizes = [100, 500, 1000]
        results = {}
        
        for size in sizes:
            shape = [size, size]
            
            # Create test tensors
            a = cortex_ones(shape, 2)
            b = cortex_ones(shape, 2)
            
            if not a or not b:
                results[size] = float('inf')
                continue
            
            # Benchmark addition
            start_time = time.time()
            result = cortex_tensor_add(a, b)
            addition_time = time.time() - start_time
            
            if result:
                cortex_tensor_free(result)
                results[size] = addition_time
            else:
                results[size] = float('inf')
            
            cortex_tensor_free(a)
            cortex_tensor_free(b)
        
        return results
    
    def benchmark_matrix_multiplication(self):
        """Benchmark matrix multiplication performance."""
        sizes = [50, 100, 200]
        results = {}
        
        for size in sizes:
            shape_a = [size, size]
            shape_b = [size, size]
            
            # Create test matrices
            a = cortex_randn(shape_a, 2)
            b = cortex_randn(shape_b, 2)
            
            if not a or not b:
                results[size] = float('inf')
                continue
            
            # Benchmark matrix multiplication
            start_time = time.time()
            result = cortex_tensor_matmul(a, b)
            matmul_time = time.time() - start_time
            
            if result:
                cortex_tensor_free(result)
                results[size] = matmul_time
            else:
                results[size] = float('inf')
            
            cortex_tensor_free(a)
            cortex_tensor_free(b)
        
        return results
    
    def benchmark_activation_functions(self):
        """Benchmark activation function performance."""
        sizes = [1000, 5000, 10000]
        results = {}
        
        for size in sizes:
            shape = [size]
            
            # Create test tensor
            tensor = cortex_randn(shape, 1)
            
            if not tensor:
                results[size] = float('inf')
                continue
            
            # Benchmark ReLU
            start_time = time.time()
            result = cortex_tensor_relu(tensor)
            relu_time = time.time() - start_time
            
            if result:
                cortex_tensor_free(result)
                results[size] = relu_time
            else:
                results[size] = float('inf')
            
            cortex_tensor_free(tensor)
        
        return results
    
    def benchmark_statistical_functions(self):
        """Benchmark statistical function performance."""
        sizes = [1000, 5000, 10000]
        results = {}
        
        for size in sizes:
            shape = [size]
            
            # Create test tensor
            tensor = cortex_randn(shape, 1)
            
            if not tensor:
                results[size] = float('inf')
                continue
            
            # Benchmark mean calculation
            start_time = time.time()
            mean_val = cortex_tensor_mean(tensor)
            mean_time = time.time() - start_time
            
            results[size] = mean_time
            
            cortex_tensor_free(tensor)
        
        return results
    
    def test_performance_regression(self):
        """Test for performance regressions."""
        # Run benchmarks
        creation_results = self.benchmark_tensor_creation()
        operation_results = self.benchmark_tensor_operations()
        matmul_results = self.benchmark_matrix_multiplication()
        activation_results = self.benchmark_activation_functions()
        stats_results = self.benchmark_statistical_functions()
        
        # Print results
        print("\n=== Performance Benchmark Results ===")
        
        print("\nTensor Creation (seconds):")
        for size, time_taken in creation_results.items():
            print(f"  {size}x{size}: {time_taken:.6f}")
        
        print("\nTensor Operations (seconds):")
        for size, time_taken in operation_results.items():
            print(f"  {size}x{size}: {time_taken:.6f}")
        
        print("\nMatrix Multiplication (seconds):")
        for size, time_taken in matmul_results.items():
            print(f"  {size}x{size}: {time_taken:.6f}")
        
        print("\nActivation Functions (seconds):")
        for size, time_taken in activation_results.items():
            print(f"  {size} elements: {time_taken:.6f}")
        
        print("\nStatistical Functions (seconds):")
        for size, time_taken in stats_results.items():
            print(f"  {size} elements: {time_taken:.6f}")
        
        # Basic performance assertions
        # These are loose bounds to catch major regressions
        
        # Tensor creation should be reasonably fast
        for size, time_taken in creation_results.items():
            if time_taken != float('inf'):
                self.assertLess(time_taken, 1.0, f"Tensor creation too slow for {size}x{size}")
        
        # Tensor operations should be reasonably fast
        for size, time_taken in operation_results.items():
            if time_taken != float('inf'):
                self.assertLess(time_taken, 1.0, f"Tensor operations too slow for {size}x{size}")
        
        # Matrix multiplication should be reasonably fast
        for size, time_taken in matmul_results.items():
            if time_taken != float('inf'):
                self.assertLess(time_taken, 5.0, f"Matrix multiplication too slow for {size}x{size}")
        
        # Activation functions should be reasonably fast
        for size, time_taken in activation_results.items():
            if time_taken != float('inf'):
                self.assertLess(time_taken, 0.1, f"Activation functions too slow for {size} elements")
        
        # Statistical functions should be reasonably fast
        for size, time_taken in stats_results.items():
            if time_taken != float('inf'):
                self.assertLess(time_taken, 0.1, f"Statistical functions too slow for {size} elements")
    
    def test_memory_usage(self):
        """Test memory usage patterns."""
        # Create large tensor
        shape = [1000, 1000]
        tensor = cortex_tensor_create(shape, 2)
        
        self.assertIsNotNone(tensor, "Failed to create large tensor")
        
        # Check that we can perform operations
        result = cortex_tensor_add(tensor, tensor)
        self.assertIsNotNone(result, "Failed to perform operation on large tensor")
        
        # Clean up
        cortex_tensor_free(tensor)
        cortex_tensor_free(result)
    
    def test_scalability(self):
        """Test scalability with increasing problem sizes."""
        sizes = [10, 50, 100, 200]
        times = []
        
        for size in sizes:
            shape = [size, size]
            
            # Create tensors
            a = cortex_randn(shape, 2)
            b = cortex_randn(shape, 2)
            
            if not a or not b:
                times.append(float('inf'))
                continue
            
            # Time matrix multiplication
            start_time = time.time()
            result = cortex_tensor_matmul(a, b)
            end_time = time.time()
            
            if result:
                times.append(end_time - start_time)
                cortex_tensor_free(result)
            else:
                times.append(float('inf'))
            
            cortex_tensor_free(a)
            cortex_tensor_free(b)
        
        # Check that times increase reasonably (not exponentially)
        for i in range(1, len(times)):
            if times[i] != float('inf') and times[i-1] != float('inf'):
                # Time should increase, but not dramatically
                ratio = times[i] / times[i-1]
                self.assertLess(ratio, 100, f"Performance scaling too poor: {ratio}x")
        
        print(f"\nScalability test results: {times}")


if __name__ == '__main__':
    unittest.main()
