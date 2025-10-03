//! Security and resource management for the Cortex compiler
//! 
//! Provides configuration and enforcement of security limits to prevent
//! resource exhaustion attacks and ensure production safety.

use crate::error::{CompilerError, DetailedError, ErrorContext};

/// Security configuration for the Cortex compiler
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Maximum recursion depth for parsing (default: 1000)
    pub max_recursion_depth: usize,
    
    /// Maximum source code size in bytes (default: 1MB)
    pub max_source_size: usize,
    
    /// Maximum number of tokens (default: 100,000)
    pub max_token_count: usize,
    
    /// Maximum string literal length (default: 10KB)
    pub max_string_length: usize,
    
    /// Maximum identifier length (default: 1KB)
    pub max_identifier_length: usize,
    
    /// Maximum array/dictionary size (default: 10,000)
    pub max_collection_size: usize,
    
    /// Maximum function parameter count (default: 255)
    pub max_function_parameters: usize,
    
    /// Enable strict security mode (default: false)
    pub strict_mode: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_recursion_depth: 1000,
            max_source_size: 1024 * 1024, // 1MB
            max_token_count: 100_000,
            max_string_length: 10 * 1024, // 10KB
            max_identifier_length: 1024,   // 1KB
            max_collection_size: 10_000,
            max_function_parameters: 255,
            strict_mode: false,
        }
    }
}

impl SecurityConfig {
    /// Creates a new security configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a strict security configuration with more restrictive limits
    pub fn strict() -> Self {
        Self {
            max_recursion_depth: 100,
            max_source_size: 100 * 1024, // 100KB
            max_token_count: 10_000,
            max_string_length: 1024, // 1KB
            max_identifier_length: 256,
            max_collection_size: 1000,
            max_function_parameters: 32,
            strict_mode: true,
        }
    }
    
    /// Creates a permissive configuration for development
    pub fn permissive() -> Self {
        Self {
            max_recursion_depth: 10_000,
            max_source_size: 10 * 1024 * 1024, // 10MB
            max_token_count: 1_000_000,
            max_string_length: 100 * 1024, // 100KB
            max_identifier_length: 10 * 1024, // 10KB
            max_collection_size: 100_000,
            max_function_parameters: 1000,
            strict_mode: false,
        }
    }
    
    /// Validates source code size
    pub fn validate_source_size(&self, size: usize) -> Result<(), DetailedError> {
        if size > self.max_source_size {
            Err(self.create_resource_limit_error(
                format!("Source code size ({}) exceeds maximum allowed size ({})", 
                        size, self.max_source_size),
                "Consider breaking your code into smaller files or increasing the size limit"
            ))
        } else {
            Ok(())
        }
    }
    
    /// Validates string literal length
    pub fn validate_string_length(&self, length: usize, line: usize, column: usize) -> Result<(), DetailedError> {
        if length > self.max_string_length {
            Err(self.create_security_error(
                format!("String literal length ({}) exceeds maximum allowed length ({})", 
                        length, self.max_string_length),
                line,
                column,
                "Consider breaking long strings into multiple parts or using string concatenation"
            ))
        } else {
            Ok(())
        }
    }
    
    /// Validates identifier length
    pub fn validate_identifier_length(&self, length: usize, line: usize, column: usize) -> Result<(), DetailedError> {
        if length > self.max_identifier_length {
            Err(self.create_security_error(
                format!("Identifier length ({}) exceeds maximum allowed length ({})", 
                        length, self.max_identifier_length),
                line,
                column,
                "Consider using shorter, more descriptive names"
            ))
        } else {
            Ok(())
        }
    }
    
    /// Validates collection size (arrays, dictionaries)
    pub fn validate_collection_size(&self, size: usize, line: usize, column: usize) -> Result<(), DetailedError> {
        if size > self.max_collection_size {
            Err(self.create_security_error(
                format!("Collection size ({}) exceeds maximum allowed size ({})", 
                        size, self.max_collection_size),
                line,
                column,
                "Consider using smaller collections or splitting data across multiple collections"
            ))
        } else {
            Ok(())
        }
    }
    
    /// Validates function parameter count
    pub fn validate_function_parameters(&self, count: usize, line: usize, column: usize) -> Result<(), DetailedError> {
        if count > self.max_function_parameters {
            Err(self.create_security_error(
                format!("Function parameter count ({}) exceeds maximum allowed count ({})", 
                        count, self.max_function_parameters),
                line,
                column,
                "Consider using fewer parameters, objects, or function overloading"
            ))
        } else {
            Ok(())
        }
    }
    
    /// Validates token count
    pub fn validate_token_count(&self, count: usize) -> Result<(), DetailedError> {
        if count > self.max_token_count {
            Err(self.create_resource_limit_error(
                format!("Token count ({}) exceeds maximum allowed count ({})", 
                        count, self.max_token_count),
                "Source code is too complex, consider breaking it into smaller modules"
            ))
        } else {
            Ok(())
        }
    }
    
    /// Creates a security error with context
    fn create_security_error(&self, message: String, line: usize, column: usize, suggestion: &str) -> DetailedError {
        let error = CompilerError::SecurityError { message };
        let context = ErrorContext::new(format!("Line {}", line))
            .with_suggestion(suggestion.to_string())
            .with_help("Security limits help prevent resource exhaustion attacks".to_string());
        DetailedError::new(error).with_context(context)
    }
    
    /// Creates a resource limit error
    fn create_resource_limit_error(&self, message: String, suggestion: &str) -> DetailedError {
        let error = CompilerError::ResourceLimitExceeded { message };
        let context = ErrorContext::new("".to_string())
            .with_suggestion(suggestion.to_string())
            .with_help("Resource limits protect against denial-of-service attacks".to_string());
        DetailedError::new(error).with_context(context)
    }
}

/// Recursion depth tracker for preventing stack overflow attacks
#[derive(Debug, Clone)]
pub struct RecursionTracker {
    current_depth: usize,
    max_depth: usize,
}

impl RecursionTracker {
    /// Creates a new recursion tracker with the given maximum depth
    pub fn new(max_depth: usize) -> Self {
        Self {
            current_depth: 0,
            max_depth,
        }
    }
    
    /// Enters a new recursion level
    pub fn enter(&mut self) -> Result<(), DetailedError> {
        self.current_depth += 1;
        if self.current_depth > self.max_depth {
            Err(DetailedError::new(CompilerError::SecurityError {
                message: format!("Recursion depth ({}) exceeds maximum allowed depth ({})", 
                                self.current_depth, self.max_depth),
            }).with_context(ErrorContext::new("".to_string())
                .with_suggestion("Simplify nested expressions or increase recursion limit".to_string())
                .with_help("Deep recursion can cause stack overflow attacks".to_string())))
        } else {
            Ok(())
        }
    }
    
    /// Exits the current recursion level
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
    
    /// Gets the current recursion depth
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }
    
    /// Gets the maximum allowed depth
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
}

/// Security context that can be passed through parsing operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub config: SecurityConfig,
    pub recursion_tracker: RecursionTracker,
}

impl SecurityContext {
    /// Creates a new security context with default configuration
    pub fn new() -> Self {
        Self {
            config: SecurityConfig::new(),
            recursion_tracker: RecursionTracker::new(SecurityConfig::new().max_recursion_depth),
        }
    }
    
    /// Creates a new security context with custom configuration
    pub fn with_config(config: SecurityConfig) -> Self {
        let max_depth = config.max_recursion_depth;
        Self {
            config,
            recursion_tracker: RecursionTracker::new(max_depth),
        }
    }
    
    /// Creates a strict security context
    pub fn strict() -> Self {
        let config = SecurityConfig::strict();
        let max_depth = config.max_recursion_depth;
        Self {
            config,
            recursion_tracker: RecursionTracker::new(max_depth),
        }
    }
    
    /// Creates a permissive security context for development
    pub fn permissive() -> Self {
        let config = SecurityConfig::permissive();
        let max_depth = config.max_recursion_depth;
        Self {
            config,
            recursion_tracker: RecursionTracker::new(max_depth),
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::new();
        assert_eq!(config.max_recursion_depth, 1000);
        assert_eq!(config.max_source_size, 1024 * 1024);
        assert_eq!(config.max_token_count, 100_000);
        assert!(!config.strict_mode);
    }
    
    #[test]
    fn test_security_config_strict() {
        let config = SecurityConfig::strict();
        assert_eq!(config.max_recursion_depth, 100);
        assert_eq!(config.max_source_size, 100 * 1024);
        assert!(config.strict_mode);
    }
    
    #[test]
    fn test_security_config_permissive() {
        let config = SecurityConfig::permissive();
        assert_eq!(config.max_recursion_depth, 10_000);
        assert_eq!(config.max_source_size, 10 * 1024 * 1024);
        assert!(!config.strict_mode);
    }
    
    #[test]
    fn test_source_size_validation() {
        let config = SecurityConfig::new();
        
        // Valid size
        assert!(config.validate_source_size(1024).is_ok());
        
        // Invalid size
        assert!(config.validate_source_size(2 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_string_length_validation() {
        let config = SecurityConfig::new();
        
        // Valid length
        assert!(config.validate_string_length(100, 1, 1).is_ok());
        
        // Invalid length
        assert!(config.validate_string_length(20 * 1024, 1, 1).is_err());
    }
    
    #[test]
    fn test_recursion_tracker() {
        let mut tracker = RecursionTracker::new(10);
        
        // Enter recursion levels
        for i in 1..=10 {
            assert!(tracker.enter().is_ok());
            assert_eq!(tracker.current_depth(), i);
        }
        
        // Exceed limit (this increments depth to 11 but returns error)
        assert!(tracker.enter().is_err());
        
        // Exit levels (start from depth 11, exit to 0)
        for i in (1..=11).rev() {
            tracker.exit();
            assert_eq!(tracker.current_depth(), i - 1);
        }
    }
    
    #[test]
    fn test_security_context() {
        let context = SecurityContext::new();
        assert_eq!(context.config.max_recursion_depth, 1000);
        assert_eq!(context.recursion_tracker.max_depth(), 1000);
        
        let strict_context = SecurityContext::strict();
        assert_eq!(strict_context.config.max_recursion_depth, 100);
        assert!(strict_context.config.strict_mode);
    }
}
