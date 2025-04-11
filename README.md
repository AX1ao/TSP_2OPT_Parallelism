# TSP_2OPT_Parallelism

This project implements and benchmarks the **2-opt algorithm** for solving the **Traveling Salesman Problem (TSP)** in Rust. The focus is on evaluating how well the algorithm performs in a sequential baseline, and then parallelizing it to improve speed and scalability.

## 🧠 Problem Summary

The Traveling Salesman Problem (TSP) asks:
> "What is the shortest possible route that visits a list of cities exactly once and returns to the origin city?"

This problem is **NP-hard**, meaning no known algorithm can solve it optimally in polynomial time. Instead, we use a heuristic approach: **2-opt**, which iteratively improves the route by reversing segments to reduce total distance.

## 🎯 Project Objectives

- ✅ Implement a **sequential version** of 2-opt in Rust as a baseline
- ✅ Benchmark performance on city sets of varying sizes (e.g. 10, 50, 100, 500)
- 🚀 Implement a **parallel version** using Rust threads, `Rayon`, or other concurrency primitives
- 📈 Compare results in terms of execution time, speedup, and final path quality

## 📦 Current Project Status

| Component                | Status        |
|-------------------------|---------------|
| City & Distance Modeling | ✅ Done        |
| Random City Generator    | ✅ Done        |
| Sequential 2-opt         | 🚧 In progress |
| Benchmarking & Timing    | 🚧 In progress |
| Parallel 2-opt           | ❌ Not started |
| Final Comparison & Report| ❌ Not started |

## 🔧 Usage

```bash
cargo run --release
