# Performance Comparison: `if-else` vs `switch-case`

This repository contains scripts that compare the performance of `if-else` statements with `switch-case` (or equivalent) structures in Python, Node.js, and Rust. The tests are performed for both integer and string types.

## Project Structure

- `python/`: Contains the Python script to compare performance.
- `nodejs/`: Contains the Node.js script to compare performance.
- `rust/`: Contains the Rust script to compare performance.

## Prerequisites

Make sure you have the following installed on your system:

- **Python 3.x**
- **Node.js**
- **Rust**

## Running the Tests

### Python

To run the Python tests, navigate to the `python/` directory and execute the script:

```bash
cd python
python3 comparison.py
```

### Node.js

To run the Node.js tests, navigate to the `nodejs/` directory and execute the script:

```bash
cd nodejs
node comparison.js
```

### Rust

To run the Rust tests, navigate to the `rust/` directory, compile the Rust script, and then execute it:

```bash
cd rust
rustc -C opt-level=3 comparison.rs -o comparison
./comparison
```

## Results

Each script will output the time taken by `if-else` and `switch-case` (or equivalent) structures for both integer and string types. The time is displayed in milliseconds (`ms`).

| Language   | If-Else (Int32)      | If-Else (String)    | switch-case (Int32)  | switch-case (String) |
| ---------- | -------------------- | ------------------- | -------------------- | -------------------- |
| Javascript | 195.211ms            | 202.952ms           | 145.006ms            | 145.954ms            |
| Python     | 11439.805030822754ms | 9776.126146316528ms | 18692.083835601807ms | 18676.21684074402ms  |
| Rust       | 0.0000000420ms       | 0.0000000830ms      | 0.0000000410ms       | 0.0000000410ms       |

### Conclusion

In JavaScript, the switch-case structure outperforms if-else for both integer and string types. The difference is relatively modest but consistent, with switch-case being about 25-30% faster. Python shows a significant performance difference, with if-else being faster than the dictionary-based switch-case approach, especially for integers. The performance gap is substantial, with switch-case taking nearly twice as long as if-else. Additionally, Python's performance for string comparisons is better than for integers, though still slower than JavaScript. Rust demonstrates near-instantaneous execution times for both if-else and match-case structures, with minimal differences between them. Rust's performance is orders of magnitude faster than both Python and JavaScript, likely due to its low-level optimizations and compiled nature. The consistency across both integers and strings highlights Rust's efficiency in handling these control structures. These results suggest that language choice and control structure usage can have a significant impact on performance, particularly in performance-critical applications. Rust is ideal for scenarios requiring high efficiency, while Python might be less suitable for tasks with stringent performance requirements.
