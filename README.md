# 🧠 TSP 2-Opt Parallelism in Rust

This project benchmarks and compares the performance of **sequential vs. parallel** implementations of the **2-opt algorithm** for solving the Traveling Salesman Problem (TSP) in Rust. It also explores **hybrid strategies** using Genetic Algorithms (GA) with local refinement.

---

## 🎯 Project Objectives

- Implement a variety of sequential and parallel TSP solvers.
- Evaluate trade-offs in speed vs. tour quality.
- Test performance across different input sizes (`n = [50, 100, 200, 500, 1000]`).
- Test performance across different core counts (`[1, 2, 4, 8, 16, 32, 64*]`) using `taskset`.
- Combine global (GA) and local (2-opt) heuristics for hybrid optimization.
- Provide reproducible experiments with clean CLI and output logs.

---

## ⚙️ Logistics & Procedures

- All algorithms tested on random Euclidean graphs of size 50–1000.
- Parallelism is implemented using Rayon, with thread control via `taskset`.
- Each version logs tour cost and timing.
- Results are saved in:
  - `expected_outputs/main.txt`
  - `expected_outputs/scalability.txt`
  - `expected_outputs/parallelism_samples.txt`

### 🔧 Run scalability benchmarks:

```bash
cargo run --release --bin main_scalability <version_name>
```

### 🔧 Run all versions (n=1000) under specified CPU mask:
```bash
taskset -c 0 cargo run --release --bin main_parallelism
```

---

## 📊 Results & Conclusions (Summary)

| Version     | Type         | Cost (n=1000) | Time        | Note                         |
|-------------|--------------|---------------|-------------|------------------------------|
| `seq`       | Sequential   | ~26k          | ~112 ms     | Very fast                    |
| `topkplus`  | Parallel     | ~25–26k       | 3–7 sec     | Good quality, moderate speed |
| `mult1`     | Parallel     | ~79k          | 1.2–20 sec  | Often worse quality          |
| `mult2`     | Parallel     | ~498k         | <1 sec      | Fast but bad                 |
| `mult3`     | Parallel     | ~28k          | ~300 ms–3s  | Balanced                     |
| `mult4`     | Parallel     | ~26k          | ~380 ms–4s  | Best cost-quality balance    |
| `ga3`       | Hybrid (GA)  | ~26k          | ~3–48 sec   | Strong global+local          |

🧠 **Conclusion:**  
Sequential 2-opt remains best for small sizes. For `n ≥ 1000`, hybrid and `mult4` yield the best results. `mult2` is fastest but lowest quality.

---

## 📁 Folder Structure
For clarity and ease of viewing, we reorganized the code into a clean demo version. All original development code and raw results are preserved in the raw_dev folder.
```
TSP_2OPT_Parallelism/
├── src/
│   ├── all_versions/               # All algorithm variants
│   │   ├── two_opt_seq.rs
│   │   ├── par_prototype.rs
│   │   ├── par_topk.rs
│   │   ├── par_topkplus.rs
│   │   ├── optimized_multithread_2opt.rs
│   │   ├── optimized_ver2_multi2opt.rs
│   │   ├── random_insert_ver3_multi2opt.rs
│   │   ├── ga_baseline.rs
│   │   ├── ga_config.rs
│   │   ├── par_ga.rs
│   │   └── utils.rs
│   ├── main.rs                    # One-shot comparison of all
│   ├── main_scalability.rs       # Varying input sizes
│   └── main_parallelism.rs       # Fixed input (n=1000), taskset support
├── expected_outputs/             # Saved logs for report
│   ├── main.txt
│   ├── scalability.txt
│   └── parallelism_samples.txt
├── raw_dev/                      # Working files, experiments, drafts
├── Cargo.toml
└── README.md                     # ← You are here!
```

---

## 🛠 Dependencies

```toml
[dependencies]
rand = "0.8"
rayon = "1.7"
```

---
