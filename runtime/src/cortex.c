/**
 * Cortex Runtime Implementation
 * 
 * Core runtime implementation for the Cortex programming language.
 * Provides tensor operations, mathematical functions, and AI/ML primitives.
 */

#include "cortex.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

// Global error state
static char* g_error_message = NULL;

// Internal utility functions
static void* cortex_malloc(size_t size) {
    void* ptr = malloc(size);
    if (!ptr) {
        cortex_set_error("Memory allocation failed");
    }
    return ptr;
}

static void* cortex_calloc(size_t count, size_t size) {
    void* ptr = calloc(count, size);
    if (!ptr) {
        cortex_set_error("Memory allocation failed");
    }
    return ptr;
}

// Memory management
cortex_tensor_t* cortex_tensor_create(int64_t* shape, int32_t ndim) {
    if (!shape || ndim <= 0) {
        cortex_set_error("Invalid tensor shape");
        return NULL;
    }
    
    cortex_tensor_t* tensor = (cortex_tensor_t*)cortex_malloc(sizeof(cortex_tensor_t));
    if (!tensor) return NULL;
    
    tensor->ndim = ndim;
    tensor->shape = (int64_t*)cortex_malloc(ndim * sizeof(int64_t));
    if (!tensor->shape) {
        free(tensor);
        return NULL;
    }
    
    memcpy(tensor->shape, shape, ndim * sizeof(int64_t));
    
    tensor->size = 1;
    for (int32_t i = 0; i < ndim; i++) {
        tensor->size *= shape[i];
    }
    
    tensor->data = (cortex_float_t*)cortex_calloc(tensor->size, sizeof(cortex_float_t));
    if (!tensor->data) {
        free(tensor->shape);
        free(tensor);
        return NULL;
    }
    
    tensor->requires_grad = false;
    return tensor;
}

void cortex_tensor_free(cortex_tensor_t* tensor) {
    if (tensor) {
        if (tensor->data) free(tensor->data);
        if (tensor->shape) free(tensor->shape);
        free(tensor);
    }
}

cortex_tensor_t* cortex_tensor_copy(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot copy NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* copy = cortex_tensor_create(tensor->shape, tensor->ndim);
    if (!copy) return NULL;
    
    memcpy(copy->data, tensor->data, tensor->size * sizeof(cortex_float_t));
    copy->requires_grad = tensor->requires_grad;
    
    return copy;
}

// Tensor creation functions
cortex_tensor_t* cortex_zeros(int64_t* shape, int32_t ndim) {
    cortex_tensor_t* tensor = cortex_tensor_create(shape, ndim);
    if (!tensor) return NULL;
    
    // Data is already zero-initialized by calloc
    return tensor;
}

cortex_tensor_t* cortex_ones(int64_t* shape, int32_t ndim) {
    cortex_tensor_t* tensor = cortex_tensor_create(shape, ndim);
    if (!tensor) return NULL;
    
    for (int64_t i = 0; i < tensor->size; i++) {
        tensor->data[i] = 1.0;
    }
    
    return tensor;
}

cortex_tensor_t* cortex_randn(int64_t* shape, int32_t ndim) {
    cortex_tensor_t* tensor = cortex_tensor_create(shape, ndim);
    if (!tensor) return NULL;
    
    // Simple random number generator (in production, use a proper RNG)
    srand((unsigned int)time(NULL));
    
    for (int64_t i = 0; i < tensor->size; i++) {
        // Box-Muller transform for normal distribution
        double u1 = (double)rand() / RAND_MAX;
        double u2 = (double)rand() / RAND_MAX;
        tensor->data[i] = sqrt(-2.0 * log(u1)) * cos(2.0 * M_PI * u2);
    }
    
    return tensor;
}

cortex_tensor_t* cortex_eye(int64_t size) {
    int64_t shape[2] = {size, size};
    cortex_tensor_t* tensor = cortex_tensor_create(shape, 2);
    if (!tensor) return NULL;
    
    for (int64_t i = 0; i < size; i++) {
        tensor->data[i * size + i] = 1.0;
    }
    
    return tensor;
}

cortex_tensor_t* cortex_arange(cortex_float_t start, cortex_float_t stop, cortex_float_t step) {
    int64_t size = (int64_t)((stop - start) / step);
    int64_t shape[1] = {size};
    
    cortex_tensor_t* tensor = cortex_tensor_create(shape, 1);
    if (!tensor) return NULL;
    
    for (int64_t i = 0; i < size; i++) {
        tensor->data[i] = start + i * step;
    }
    
    return tensor;
}

// Basic tensor operations
cortex_tensor_t* cortex_tensor_add(const cortex_tensor_t* a, const cortex_tensor_t* b) {
    if (!a || !b) {
        cortex_set_error("Cannot add NULL tensors");
        return NULL;
    }
    
    if (a->size != b->size) {
        cortex_set_error("Tensor size mismatch for addition");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(a);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] += b->data[i];
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_subtract(const cortex_tensor_t* a, const cortex_tensor_t* b) {
    if (!a || !b) {
        cortex_set_error("Cannot subtract NULL tensors");
        return NULL;
    }
    
    if (a->size != b->size) {
        cortex_set_error("Tensor size mismatch for subtraction");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(a);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] -= b->data[i];
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_multiply(const cortex_tensor_t* a, const cortex_tensor_t* b) {
    if (!a || !b) {
        cortex_set_error("Cannot multiply NULL tensors");
        return NULL;
    }
    
    if (a->size != b->size) {
        cortex_set_error("Tensor size mismatch for multiplication");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(a);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] *= b->data[i];
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_divide(const cortex_tensor_t* a, const cortex_tensor_t* b) {
    if (!a || !b) {
        cortex_set_error("Cannot divide NULL tensors");
        return NULL;
    }
    
    if (a->size != b->size) {
        cortex_set_error("Tensor size mismatch for division");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(a);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        if (b->data[i] == 0.0) {
            cortex_set_error("Division by zero");
            cortex_tensor_free(result);
            return NULL;
        }
        result->data[i] /= b->data[i];
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_power(const cortex_tensor_t* a, const cortex_tensor_t* b) {
    if (!a || !b) {
        cortex_set_error("Cannot raise NULL tensors to power");
        return NULL;
    }
    
    if (a->size != b->size) {
        cortex_set_error("Tensor size mismatch for power operation");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(a);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] = pow(result->data[i], b->data[i]);
    }
    
    return result;
}

// Scalar operations
cortex_tensor_t* cortex_tensor_add_scalar(const cortex_tensor_t* tensor, cortex_float_t scalar) {
    if (!tensor) {
        cortex_set_error("Cannot add scalar to NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] += scalar;
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_multiply_scalar(const cortex_tensor_t* tensor, cortex_float_t scalar) {
    if (!tensor) {
        cortex_set_error("Cannot multiply NULL tensor by scalar");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] *= scalar;
    }
    
    return result;
}

// Matrix operations
cortex_tensor_t* cortex_tensor_matmul(const cortex_tensor_t* a, const cortex_tensor_t* b) {
    if (!a || !b) {
        cortex_set_error("Cannot multiply NULL tensors");
        return NULL;
    }
    
    if (a->ndim != 2 || b->ndim != 2) {
        cortex_set_error("Matrix multiplication requires 2D tensors");
        return NULL;
    }
    
    if (a->shape[1] != b->shape[0]) {
        cortex_set_error("Matrix dimension mismatch for multiplication");
        return NULL;
    }
    
    int64_t result_shape[2] = {a->shape[0], b->shape[1]};
    cortex_tensor_t* result = cortex_tensor_create(result_shape, 2);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < a->shape[0]; i++) {
        for (int64_t j = 0; j < b->shape[1]; j++) {
            cortex_float_t sum = 0.0;
            for (int64_t k = 0; k < a->shape[1]; k++) {
                sum += a->data[i * a->shape[1] + k] * b->data[k * b->shape[1] + j];
            }
            result->data[i * result->shape[1] + j] = sum;
        }
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_transpose(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot transpose NULL tensor");
        return NULL;
    }
    
    if (tensor->ndim != 2) {
        cortex_set_error("Transpose requires 2D tensor");
        return NULL;
    }
    
    int64_t result_shape[2] = {tensor->shape[1], tensor->shape[0]};
    cortex_tensor_t* result = cortex_tensor_create(result_shape, 2);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < tensor->shape[0]; i++) {
        for (int64_t j = 0; j < tensor->shape[1]; j++) {
            result->data[j * result->shape[1] + i] = tensor->data[i * tensor->shape[1] + j];
        }
    }
    
    return result;
}

// Mathematical functions
cortex_tensor_t* cortex_tensor_exp(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute exp of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] = exp(result->data[i]);
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_log(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute log of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        if (result->data[i] <= 0.0) {
            cortex_set_error("Log of non-positive number");
            cortex_tensor_free(result);
            return NULL;
        }
        result->data[i] = log(result->data[i]);
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_sqrt(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute sqrt of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        if (result->data[i] < 0.0) {
            cortex_set_error("Sqrt of negative number");
            cortex_tensor_free(result);
            return NULL;
        }
        result->data[i] = sqrt(result->data[i]);
    }
    
    return result;
}

// Activation functions
cortex_tensor_t* cortex_tensor_relu(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute ReLU of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] = fmax(0.0, result->data[i]);
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_sigmoid(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute sigmoid of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] = 1.0 / (1.0 + exp(-result->data[i]));
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_tanh(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute tanh of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] = tanh(result->data[i]);
    }
    
    return result;
}

cortex_tensor_t* cortex_tensor_softmax(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute softmax of NULL tensor");
        return NULL;
    }
    
    cortex_tensor_t* result = cortex_tensor_copy(tensor);
    if (!result) return NULL;
    
    // Find maximum value for numerical stability
    cortex_float_t max_val = result->data[0];
    for (int64_t i = 1; i < result->size; i++) {
        if (result->data[i] > max_val) {
            max_val = result->data[i];
        }
    }
    
    // Compute exponentials and sum
    cortex_float_t sum = 0.0;
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] = exp(result->data[i] - max_val);
        sum += result->data[i];
    }
    
    // Normalize
    for (int64_t i = 0; i < result->size; i++) {
        result->data[i] /= sum;
    }
    
    return result;
}

// Statistical functions
cortex_float_t cortex_tensor_mean(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute mean of NULL tensor");
        return 0.0;
    }
    
    return cortex_tensor_sum(tensor) / tensor->size;
}

cortex_float_t cortex_tensor_std(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute std of NULL tensor");
        return 0.0;
    }
    
    cortex_float_t mean_val = cortex_tensor_mean(tensor);
    cortex_float_t sum_sq_diff = 0.0;
    
    for (int64_t i = 0; i < tensor->size; i++) {
        cortex_float_t diff = tensor->data[i] - mean_val;
        sum_sq_diff += diff * diff;
    }
    
    return sqrt(sum_sq_diff / tensor->size);
}

cortex_float_t cortex_tensor_var(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute variance of NULL tensor");
        return 0.0;
    }
    
    cortex_float_t std_val = cortex_tensor_std(tensor);
    return std_val * std_val;
}

cortex_float_t cortex_tensor_min(const cortex_tensor_t* tensor) {
    if (!tensor || tensor->size == 0) {
        cortex_set_error("Cannot compute min of NULL or empty tensor");
        return 0.0;
    }
    
    cortex_float_t min_val = tensor->data[0];
    for (int64_t i = 1; i < tensor->size; i++) {
        if (tensor->data[i] < min_val) {
            min_val = tensor->data[i];
        }
    }
    
    return min_val;
}

cortex_float_t cortex_tensor_max(const cortex_tensor_t* tensor) {
    if (!tensor || tensor->size == 0) {
        cortex_set_error("Cannot compute max of NULL or empty tensor");
        return 0.0;
    }
    
    cortex_float_t max_val = tensor->data[0];
    for (int64_t i = 1; i < tensor->size; i++) {
        if (tensor->data[i] > max_val) {
            max_val = tensor->data[i];
        }
    }
    
    return max_val;
}

cortex_float_t cortex_tensor_sum(const cortex_tensor_t* tensor) {
    if (!tensor) {
        cortex_set_error("Cannot compute sum of NULL tensor");
        return 0.0;
    }
    
    cortex_float_t sum = 0.0;
    for (int64_t i = 0; i < tensor->size; i++) {
        sum += tensor->data[i];
    }
    
    return sum;
}

// Loss functions
cortex_float_t cortex_mse_loss(const cortex_tensor_t* predictions, const cortex_tensor_t* targets) {
    if (!predictions || !targets) {
        cortex_set_error("Cannot compute MSE loss of NULL tensors");
        return 0.0;
    }
    
    if (predictions->size != targets->size) {
        cortex_set_error("Tensor size mismatch for MSE loss");
        return 0.0;
    }
    
    cortex_float_t sum_sq_diff = 0.0;
    for (int64_t i = 0; i < predictions->size; i++) {
        cortex_float_t diff = predictions->data[i] - targets->data[i];
        sum_sq_diff += diff * diff;
    }
    
    return sum_sq_diff / predictions->size;
}

cortex_float_t cortex_cross_entropy_loss(const cortex_tensor_t* predictions, const cortex_tensor_t* targets) {
    if (!predictions || !targets) {
        cortex_set_error("Cannot compute cross-entropy loss of NULL tensors");
        return 0.0;
    }
    
    if (predictions->size != targets->size) {
        cortex_set_error("Tensor size mismatch for cross-entropy loss");
        return 0.0;
    }
    
    cortex_float_t loss = 0.0;
    for (int64_t i = 0; i < predictions->size; i++) {
        if (predictions->data[i] > 0.0) {
            loss -= targets->data[i] * log(predictions->data[i]);
        }
    }
    
    return loss / predictions->size;
}

cortex_float_t cortex_binary_cross_entropy_loss(const cortex_tensor_t* predictions, const cortex_tensor_t* targets) {
    if (!predictions || !targets) {
        cortex_set_error("Cannot compute binary cross-entropy loss of NULL tensors");
        return 0.0;
    }
    
    if (predictions->size != targets->size) {
        cortex_set_error("Tensor size mismatch for binary cross-entropy loss");
        return 0.0;
    }
    
    cortex_float_t loss = 0.0;
    for (int64_t i = 0; i < predictions->size; i++) {
        cortex_float_t p = predictions->data[i];
        cortex_float_t t = targets->data[i];
        
        // Clamp predictions to avoid log(0)
        p = fmax(1e-8, fmin(1.0 - 1e-8, p));
        
        loss -= t * log(p) + (1.0 - t) * log(1.0 - p);
    }
    
    return loss / predictions->size;
}

// Utility functions
void cortex_tensor_print(const cortex_tensor_t* tensor) {
    if (!tensor) {
        printf("NULL tensor\n");
        return;
    }
    
    printf("Tensor shape: [");
    for (int32_t i = 0; i < tensor->ndim; i++) {
        printf("%ld", tensor->shape[i]);
        if (i < tensor->ndim - 1) printf(", ");
    }
    printf("]\n");
    
    printf("Data: [");
    for (int64_t i = 0; i < tensor->size; i++) {
        printf("%.6f", tensor->data[i]);
        if (i < tensor->size - 1) printf(", ");
    }
    printf("]\n");
}

// Error handling
void cortex_set_error(const char* message) {
    if (g_error_message) {
        free(g_error_message);
    }
    g_error_message = strdup(message);
}

const char* cortex_get_error(void) {
    return g_error_message;
}

void cortex_clear_error(void) {
    if (g_error_message) {
        free(g_error_message);
        g_error_message = NULL;
    }
}
