# 🧠 TSP 2-Opt Parallelism in Rust

This project benchmarks and compares the performance of **sequential vs. parallel implementations** of the **2-opt algorithm** for solving the **Traveling Salesman Problem (TSP)** in Rust.

The 2-opt algorithm is a local search heuristic that iteratively improves a route by reversing segments to reduce total travel distance. This project:
- Implements a baseline sequential version in Rust
- Prepares for future parallel implementation using concurrency primitives
- Benchmarks tour cost and optimization time

---

## 🎯 Objectives

- ✅ Implement a **sequential** 2-opt algorithm
- ✅ Benchmark total cost reduction and runtime
- 🔜 Implement a **parallel** version in `two_opt_par.rs`
- 🔜 Compare time & cost vs. sequential baseline
- 🔜 (Optional) Add CLI options and visual output

---

## 📦 Current Project Status

| Component                | Status        |
|-------------------------|---------------|
| City & Distance Modeling | ✅ Done        |
| Random City Generator    | ✅ Done        |
| Sequential 2-opt         | ✅ Done        |
| Benchmarking & Timing    | ✅ Done        |
| Parallel 2-opt           | ✅ Done        |
| optimize parallel 2-opt  | 🚧 In progress |
| Parallel other algorithms| 🚧 In progress |
| Final Comparison & Report| ❌ Not started |

---

## 🧠 Parallel Implementation

We implemented a parallel version of the 2-opt algorithm using **Rayon** in Rust.

### ✅ Strategy:
- Generate all `(i, j)` candidate city-pair swaps
- Use `par_iter()` to evaluate improvement (`delta`) in parallel
- Apply the **single best** improving swap per iteration
- Repeat until no further meaningful improvement (`delta > 1e-6`) is found

### ⚠️ Notes:
- For small city counts (`n < 100`), parallelism is **slower** than sequential due to overhead
- Floating point precision issues may cause endless swaps without a meaningful cost drop — we use a `delta > 1e-6` threshold to avoid this
- A hard stop at 1000 iterations is added as a safety net

### 📊 Example Results:

| Cities | Version     | Final Cost | Time       |
|--------|-------------|------------|------------|
| 50     | Sequential  | 7115.20    | 130.53 µs  |
| 50     | Parallel    | 6451.21    | 51.93 ms   |
| 100    | Sequential  | 8047.78    | 772 µs     |
| 100    | Parallel    | 8315.54    | 145.75 ms  |
| 200    | Sequential  | 11772.32   | 2.97 ms    |
| 200    | Parallel    | 11782.87   | 481.04 ms  |
| 500    | Sequential  | 18728.50   | 21.64 ms   |
| 500    | Parallel    | 18081.89   | 3.68 s     |
| 1000   | Sequential  | 25863.88   | 87.80 ms   |
| 1000   | Parallel    | 29847.77   | 10.55 s    |

> ⚠️ Parallelism is **not faster** for small to medium `n` due to thread overhead and full O(n²) re-evaluation each loop.  
> ✅ Parallel sometimes finds a **slightly better local minimum** due to evaluating all swaps at once.  
> ⚠️ At larger scales, current parallel design converges slowly and may even return worse results if iteration limit is hit.

---

## 🔜 Next Steps

- Benchmark for `n = 200, 500, 1000` to see crossover point
- Optimize further by batching multiple swaps per iteration (if non-overlapping)
- Consider parallelizing other TSP algorithms like:
  - Simulated Annealing
  - Genetic Algorithms
  - Ant Colony Optimization

---

## 🚀 Usage

### ⚙️ Run with default (50 cities)
```bash
cargo run --release
```

### ⚙️ Run with custom number of cities (e.g., 100)
```bash
cargo run --release -- 100
```

> The program will generate random cities, compute the initial and optimized tour cost, and print execution time.

---

## 🧪 Example Output

```bash
Generating 100 cities...
Initial tour cost: 50292.04
Final tour cost: 8683.73
Time taken: 589.60µs
```

---

## 🧱 Project Structure

```
src/
├── main.rs             # CLI entry point
├── tsp.rs              # Shared data structures and 2-opt sequential logic
├── two_opt_par.rs      # (To be implemented) parallel version
```

---

## 🛠 Dependencies

```toml
[dependencies]
rand = "0.8"
```

