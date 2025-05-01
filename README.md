# ChronoGraph: A Parallel Temporal Graph Library in Rust

**Author:** Yifan Jiang  
**GitHub:** [AlvinJ404/chronograph](https://github.com/AlvinJ404/chronograph)

ChronoGraph is a Rust library that implements both **sequential** and **parallel** temporal graph structures. It supports time-sensitive edge insertions and queries, designed for scalable graph analytics over time-evolving datasets.

## Features

- `TemporalGraph`: Sequential graph implementation with timestamped edges.
- `ChronoGraph`: Parallel graph implementation using the Rayon library.
- Edge insertion and removal with timestamp control.
- Time-sensitive neighbor queries (`get_neighbors_at`).
- Thread-scalable performance with configurable parallelism.

## Project Structure

- `src/sequential.rs`: Defines the `TemporalGraph` (sequential) structure and core methods.
- `src/chrono.rs`: Defines the `ChronoGraph` (parallel) structure and core methods.
- `src/lib.rs`: Exposes public modules and includes unit + benchmark tests.
- `#[cfg(test)]`: Contains:
  - `tg_unit_tests`: Unit tests for `TemporalGraph`
  - `cg_unit_tests`: Unit tests for `ChronoGraph`
  - `benchmark_test_{1,2,3}`: Multiple benchmark scenarios (dense, variable timestamps, etc.)
  - `scalability_test`: Evaluates `ChronoGraph` performance under increasing thread counts.

## Running Tests and Benchmarks

To run all unit tests and benchmarks:

```bash
cargo test -- --nocapture
