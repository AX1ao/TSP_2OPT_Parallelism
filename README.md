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
| Benchmarking & Timing    | 🚧 In progress |
| Parallel 2-opt           | 🚧 In progress |
| Final Comparison & Report| ❌ Not started |

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

