# 🧠 TSP 2-Opt Parallelism in Rust

This project benchmarks and compares the performance of **sequential vs. parallel implementations** of the **2-opt algorithm** for solving the **Traveling Salesman Problem (TSP)** in Rust.

The 2-opt algorithm is a local search heuristic that iteratively improves a route by reversing segments to reduce total travel distance. This project:
- Implements a baseline sequential version in Rust
- Prepares for future parallel implementation using concurrency primitives
- Benchmarks tour cost and optimization time

---

## 🎯 Objectives / Current Project Status

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

### 📊 Prototype Results:

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

### 📊 Top-k Batching Results

| Cities | Version            | k  | Final Cost | Time       |
|--------|--------------------|----|------------|------------|
| 50     | Sequential         | –  | 5607.75    | 159.87 µs  |
|        | Top-k Batching     | 2  | 9324.37    | 691.02 ms  |
|        | Top-k Batching     | 3  | 6000.31    | 18.69 ms   |
|        | Top-k Batching     | 5  | 6560.36    | 659.23 ms  |
|        | Top-k Batching     | 10 | 5727.18    | 18.90 ms   |
| 100    | Sequential         | –  | 8998.21    | 616.49 µs  |
|        | Top-k Batching     | 2  | 8413.43    | 103.94 ms  |
|        | Top-k Batching     | 3  | 8545.99    | 71.61 ms   |
|        | Top-k Batching     | 5  | 8776.58    | 58.55 ms   |
|        | Top-k Batching     | 10 | 28002.63   | 2.00 s     |
| 500    | Sequential         | –  | 18680.54   | 24.53 ms   |
|        | Top-k Batching     | 2  | 18194.39   | 2.59 s     |
|        | Top-k Batching     | 3  | 18192.61   | 1.87 s     |
|        | Top-k Batching     | 5  | 18531.14   | 1.28 s     |
|        | Top-k Batching     | 10 | 18687.99   | 900.63 ms  |
| 1000   | Sequential         | –  | 25831.19   | 87.51 ms   |
|        | Top-k Batching     | 2  | 25547.59   | 12.43 s    |
|        | Top-k Batching     | 3  | 25754.81   | 8.87 s     |
|        | Top-k Batching     | 5  | 26130.96   | 13.21 s    |
|        | Top-k Batching     | 10 | 27466.10   | 12.47 s    |

> ✅ **Top-k batching** applies multiple non-overlapping swaps per iteration, reducing loop count and potentially improving solution quality.  
> ✅ `k = 2 or 3` offers the best balance of **cost reduction** and **stability**, especially at `n ≥ 500`.  
> 🔁 Higher `k` values like `k=10` can introduce **swap interference**, leading to worse final cost or infinite loops.  
> ⚠️ At small `n`, high `k` may *accidentally work well* (as in `n = 50, k = 10`), but it's inconsistent.  
> 🐢 **Runtime increases quickly** for large `n` due to full re-evaluation of O(n²) swaps in every loop, even with batching.  
> 💡 Top-k is a **safe and tunable parallel upgrade** over the prototype version — good for experimentation and extension.

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

