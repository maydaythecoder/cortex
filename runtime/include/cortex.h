/**
 * Cortex Runtime Header
 * 
 * Core runtime definitions for the Cortex programming language.
 * Provides tensor operations, mathematical functions, and AI/ML primitives.
 */

#ifndef CORTEX_H
#define CORTEX_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Type definitions
typedef double cortex_float_t;
typedef int64_t cortex_int_t;
typedef bool cortex_bool_t;

// Tensor structure
typedef struct {
    cortex_float_t* data;
    int64_t* shape;
    int32_t ndim;
    int64_t size;
    bool requires_grad;
} cortex_tensor_t;

// Function result structure
typedef struct {
    bool success;
    char* error_message;
    void* result;
} cortex_result_t;

// Memory management
cortex_tensor_t* cortex_tensor_create(int64_t* shape, int32_t ndim);
void cortex_tensor_free(cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_copy(const cortex_tensor_t* tensor);

// Tensor creation functions
cortex_tensor_t* cortex_zeros(int64_t* shape, int32_t ndim);
cortex_tensor_t* cortex_ones(int64_t* shape, int32_t ndim);
cortex_tensor_t* cortex_randn(int64_t* shape, int32_t ndim);
cortex_tensor_t* cortex_eye(int64_t size);
cortex_tensor_t* cortex_arange(cortex_float_t start, cortex_float_t stop, cortex_float_t step);

// Basic tensor operations
cortex_tensor_t* cortex_tensor_add(const cortex_tensor_t* a, const cortex_tensor_t* b);
cortex_tensor_t* cortex_tensor_subtract(const cortex_tensor_t* a, const cortex_tensor_t* b);
cortex_tensor_t* cortex_tensor_multiply(const cortex_tensor_t* a, const cortex_tensor_t* b);
cortex_tensor_t* cortex_tensor_divide(const cortex_tensor_t* a, const cortex_tensor_t* b);
cortex_tensor_t* cortex_tensor_power(const cortex_tensor_t* a, const cortex_tensor_t* b);

// Scalar operations
cortex_tensor_t* cortex_tensor_add_scalar(const cortex_tensor_t* tensor, cortex_float_t scalar);
cortex_tensor_t* cortex_tensor_multiply_scalar(const cortex_tensor_t* tensor, cortex_float_t scalar);

// Matrix operations
cortex_tensor_t* cortex_tensor_matmul(const cortex_tensor_t* a, const cortex_tensor_t* b);
cortex_tensor_t* cortex_tensor_transpose(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_det(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_trace(const cortex_tensor_t* tensor);

// Mathematical functions
cortex_tensor_t* cortex_tensor_exp(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_log(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_sqrt(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_sin(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_cos(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_tan(const cortex_tensor_t* tensor);

// Activation functions
cortex_tensor_t* cortex_tensor_relu(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_sigmoid(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_tanh(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_softmax(const cortex_tensor_t* tensor);

// Statistical functions
cortex_float_t cortex_tensor_mean(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_std(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_var(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_min(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_max(const cortex_tensor_t* tensor);
cortex_float_t cortex_tensor_sum(const cortex_tensor_t* tensor);

// Loss functions
cortex_float_t cortex_mse_loss(const cortex_tensor_t* predictions, const cortex_tensor_t* targets);
cortex_float_t cortex_cross_entropy_loss(const cortex_tensor_t* predictions, const cortex_tensor_t* targets);
cortex_float_t cortex_binary_cross_entropy_loss(const cortex_tensor_t* predictions, const cortex_tensor_t* targets);

// Gradient operations
cortex_tensor_t* cortex_tensor_gradient(const cortex_tensor_t* loss, const cortex_tensor_t* parameters);

// Utility functions
void cortex_tensor_print(const cortex_tensor_t* tensor);
cortex_tensor_t* cortex_tensor_reshape(const cortex_tensor_t* tensor, int64_t* new_shape, int32_t new_ndim);
cortex_tensor_t* cortex_tensor_slice(const cortex_tensor_t* tensor, int64_t* start, int64_t* end);

// Error handling
void cortex_set_error(const char* message);
const char* cortex_get_error(void);
void cortex_clear_error(void);

#ifdef __cplusplus
}
#endif

#endif // CORTEX_H
