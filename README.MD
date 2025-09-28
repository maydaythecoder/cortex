# Cortex 🧠

**Cortex** is a **compiled programming language designed for AI and machine learning**.  
Its goal is to make building, training, and experimenting with models **simpler and faster** by combining:  

- **Readable, bracket-based syntax** → focused on clarity and flow  
- **Compiled performance (LLVM)** → optimized for numerical computing  
- **AI-first primitives** → tensors, datasets, models, training utilities  

Instead of writing AI in bloated frameworks, Cortex gives you a **language designed from the ground up for machine intelligence**.  

---

## ✨ Why Cortex?

- **AI-first design** → control flow, data structures, and math built with ML in mind  
- **Concise syntax** → `if [x > 5] | print[x] ^`  
- **Compiled performance** → safe, static, and fast  
- **Integrated ML stack** → arrays → tensors, objects → models, dicts → datasets  
- **Extendable** → pluggable backend (CPU now, GPU later)  

---

## 📖 Example Code

### Linear Regression Training Loop

```cortex
// Load dataset
let data := read["dataset.csv"]

// Define model
func linear[x, w, b] |
  return[(x * w) + b]
^

// Training
let w := 0.5
let b := 0.0
let lr := 0.01

for [epoch := 0; epoch < 100; epoch := epoch + 1] |
  let preds := linear[data.x, w, b]
  let loss := mse[preds, data.y]
  print["epoch=" + epoch + ", loss=" + loss]

  // Update params
  w := w - lr * grad[loss, w]
  b := b - lr * grad[loss, b]
^
