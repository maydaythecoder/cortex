# Cortex Runtime Makefile

CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -O2 -fPIC
INCLUDES = -Iinclude
LDFLAGS = -shared
TARGET = libcortex.so
SOURCES = src/cortex.c
OBJECTS = $(SOURCES:.c=.o)

# Default target
all: $(TARGET)

# Build shared library
$(TARGET): $(OBJECTS)
	$(CC) $(LDFLAGS) -o $@ $^ -lm

# Compile object files
%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

# Install library
install: $(TARGET)
	cp $(TARGET) /usr/local/lib/
	cp include/cortex.h /usr/local/include/
	ldconfig

# Uninstall library
uninstall:
	rm -f /usr/local/lib/$(TARGET)
	rm -f /usr/local/include/cortex.h
	ldconfig

# Run tests
test: $(TARGET)
	@echo "Running tests..."
	@echo "Tests not implemented yet"

# Clean build artifacts
clean:
	rm -f $(OBJECTS) $(TARGET)

# Debug build
debug: CFLAGS += -g -DDEBUG
debug: $(TARGET)

# Release build
release: CFLAGS += -DNDEBUG
release: $(TARGET)

# Static analysis
analyze:
	@echo "Running static analysis..."
	@echo "Static analysis not implemented yet"

# Format code
format:
	@echo "Formatting code..."
	@echo "Code formatting not implemented yet"

# Documentation
docs:
	@echo "Generating documentation..."
	@echo "Documentation generation not implemented yet"

.PHONY: all install uninstall test clean debug release analyze format docs
