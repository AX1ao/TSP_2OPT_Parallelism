# 🧠 TSP 2-Opt Parallelism in Rust

This project benchmarks and compares the performance of **sequential vs. parallel implementations** of the **2-opt algorithm** for solving the **Traveling Salesman Problem (TSP)** in Rust.

The 2-opt algorithm is a local search heuristic that iteratively improves a route by reversing segments to reduce total travel distance. This project:
- Implements a baseline sequential version in Rust
- Explores multiple parallelization strategies using Rayon
- Benchmarks tour cost and optimization time across various designs
- Organizes results for reproducibility and analysis

---

## 🎯 Objectives / Current Project Status

| Component                 | Status        |
|--------------------------|---------------|
| City & Distance Modeling | ✅ Done        |
| Random City Generator    | ✅ Done        |
| Sequential 2-opt         | ✅ Done        |
| Benchmarking & Timing    | ✅ Done        |
| Parallel 2-opt (Prototype) | ✅ Done      |
| Top-k Batching           | ✅ Done        |
| Top-k++ Thresholding     | ✅ Done        |
| Multithread version 2-opt | ✅ Done        |
| Results CSV Export       | ✅ Done        |
| Hybrid Strategy (Next)   | 🧭 Planned     |

---

## 🧠 Parallel Strategies Implemented

### ✅ Prototype (Naive Parallel 2-Opt)
- Parallel evaluation of all (i, j) swap candidates using `par_iter()`
- Applies the single best swap per iteration
- Repeats until no improving swaps remain (`delta > 1e-6`)
- Includes an iteration safety limit

### ✅ Top-K Batching
- Evaluates all improving swaps, selects top-k by delta
- Applies **non-overlapping** swaps from the top-k set
- Reduces iteration count and improves convergence speed

### ✅ Top-K++ (With Delta Threshold)
- Further filters swaps by minimum delta (`Δ > 1e-6`, `1e-5`, etc.)
- Skips weak swaps to speed up evaluation
- Adds tunable `k` and `delta_thresh` for better control

### ✅ Multithread 2-opt
- Instead of using a fixed initial route, we leverage multithreading and thread_rng to generate a different randomized initial tour for each thread.
- Each thread independently applies the 2-opt algorithm to improve its own shuffled route.
- To avoid the high overhead of full 2-opt sweeps, especially when using many threads, we adopt a random sampling strategy:
- The best tour among all threads is selected as the global result
- This method should be further optimized in order to get a competitive result compared to the sequential version
---

## 📊 Key Takeaways from Benchmark Results

### ✅ Sequential vs Parallel
- Sequential 2-Opt is **extremely fast** for `n < 500`
- Parallelism only starts to pay off beyond `n ≥ 1000`
- Parallel versions may find **slightly better routes**, but are **much slower**

### ✅ Top-K Batching
- Small `k` (2–3) often gives the best trade-off between cost and runtime
- Larger `k` can increase instability and runtime
- Good middle-ground for enhancing parallel 2-opt

### ✅ Top-K++ Optimization
- Threshold filtering (`Δ > 1e-6`) was mostly neutral in impact — weak swaps are rare anyway
- Best cost-performance balance often seen at `k = 3 or 10`
- Runtime for `n = 1000` ranged from **4s to 13s**, depending on `k`
- Sequential still dominates small instances, but TopK++ excels at **quality** in large `n`

### ⚠️ Trade-offs
- Every parallel version incurs **O(n²)** candidate generation and filtering
- Gains in cost often come at the expense of **longer runtimes**
- Combining 2-Opt with a **global search** strategy may yield better scalability

---

## 🧪 Example CLI Usage

```bash
# Run with default (50 cities)
cargo run --release

# Run with 100 cities
cargo run --release -- 100
```

The program outputs:
- Initial tour cost
- Final optimized tour cost
- Time taken for each strategy (sequential, parallel, etc.)

---

## 📂 Results
All experiment results are now stored in CSV format in the `results/` folder.
- Includes time (ms), cost, k, and threshold parameters
- Easy to import into Excel, Python (pandas), etc.

---

## 🛠 Dependencies

```toml
[dependencies]
rand = "0.8"
rayon = "1.7"
```

---
