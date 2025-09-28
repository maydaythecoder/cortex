# Data Processing Example

This example demonstrates comprehensive data preprocessing and analysis workflows in Cortex.

## Overview

The data processing example shows:

- **Data Validation**: Missing values, infinite values, consistency checks
- **Preprocessing**: Normalization, standardization, outlier removal
- **Data Augmentation**: Noise addition, scaling variations
- **Dataset Splitting**: Train/validation/test splits
- **Feature Engineering**: Statistical and frequency domain features
- **Data Loaders**: Batch processing for training

## Key Features

### Dataset Structure

```cortex
struct Dataset |
  features: tensor
  labels: tensor
  metadata: dict
^
```

### Preprocessing Functions

```cortex
func normalize_data[data: tensor] -> tensor |
  let mean_val := mean[data, axis=0]
  let std_val := std[data, axis=0]
  return[(data - mean_val) / (std_val + epsilon)]
^
```

### Data Augmentation

```cortex
func augment_data[data: tensor, labels: tensor] |
  // Add noise, scaling, and original samples
  // Returns 3x augmented dataset
^
```

## Architecture

``` txt
Raw Data → Validation → Preprocessing → Augmentation → Splitting → Data Loaders
    ↓           ↓            ↓             ↓            ↓           ↓
  Features   Quality     Normalize    Add Noise    Train/Val   Batches
  Labels     Checks      Standardize  Scale        Test        for Training
```

## Usage

```bash
python3 cortexc.py run examples/data_processing.ctx
```

## Expected Output

``` txt
=== Data Processing in Cortex ===

Generated synthetic dataset with 1000 samples
=== Data Processing Pipeline ===
=== Dataset Analysis ===
Number of samples: 1000
Number of features: 10
Number of classes: 3
Feature means: [0.12, -0.05, 0.08, ...]
Feature stds: [1.02, 0.98, 1.01, ...]
Class 0: 333 samples
Class 1: 334 samples
Class 2: 333 samples
Dataset validation passed
Removing outliers...
After outlier removal: 950 samples
Normalizing features...
Augmenting data...
After augmentation: 2850 samples
Splitting dataset...
Train set: 2280 samples
Validation set: 285 samples
Test set: 285 samples
Created 71 training batches
Created 8 validation batches
Created 8 test batches

=== Data Processing Complete ===
Ready for training with processed data loaders
```

## Key Functions

### `validate_dataset[dataset]`

Checks for missing values, infinite values, and data consistency.

### `normalize_data[data]`

Applies z-score normalization to features.

### `standardize_data[data]`

Applies min-max scaling to features.

### `remove_outliers[data, threshold]`

Removes samples with z-scores above threshold.

### `augment_data[data, labels]`

Creates augmented dataset with noise and scaling.

### `split_dataset[dataset, train_ratio, val_ratio, test_ratio]`

Splits dataset into train/validation/test sets.

### `create_data_loader[dataset, batch_size]`

Creates batches for training.

### `extract_features[raw_data]`

Engineers features from raw data.

## Data Pipeline Steps

1. **Data Generation**: Synthetic classification data
2. **Validation**: Quality checks and consistency
3. **Analysis**: Statistical summaries and class distribution
4. **Outlier Removal**: Z-score based filtering
5. **Normalization**: Feature scaling
6. **Augmentation**: Data expansion techniques
7. **Splitting**: Train/validation/test division
8. **Batch Creation**: Training-ready data loaders

## Hyperparameters

- **Batch Size**: 32
- **Train Ratio**: 0.8
- **Validation Ratio**: 0.1
- **Test Ratio**: 0.1
- **Outlier Threshold**: 3.0 (z-score)
- **Noise Level**: 0.1
- **Scale Range**: 0.9 to 1.1

## Feature Engineering

### Statistical Features

- Mean, standard deviation
- Min, max values
- Skewness, kurtosis

### Frequency Domain Features

- FFT analysis
- Spectral statistics

### Data Quality Metrics

- Missing value detection
- Infinite value checks
- Class imbalance analysis

## Implementation Notes

- Uses synthetic data for demonstration
- Implements comprehensive validation pipeline
- Shows modern ML data preprocessing practices
- Demonstrates batch processing for efficiency
- Includes data augmentation techniques

## Data Quality Checks

### Validation Criteria

- No NaN values
- No infinite values
- Consistent feature/label dimensions
- Non-empty datasets
- Class distribution analysis

### Preprocessing Steps

- Outlier removal (z-score > 3)
- Z-score normalization
- Data augmentation (3x expansion)
- Random train/val/test splitting

## Future Enhancements

- Real dataset loading (CSV, JSON, etc.)
- More augmentation techniques
- Feature selection algorithms
- Data visualization
- Streaming data processing
- Distributed data loading
