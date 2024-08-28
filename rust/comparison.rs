use std::time::Instant;

fn main() {
    let num_tests = 100_000_000;
    let x = 7;
    let str = "seven";
    let mut result = 0;

    // Measure time for if-else with int
    let start = Instant::now();
    for _ in 0..num_tests {
        if x == 1 {
            result = 1;
        } else if x == 2 {
            result = 2;
        } else if x == 3 {
            result = 3;
        } else if x == 4 {
            result = 4;
        } else if x == 5 {
            result = 5;
        } else if x == 6 {
            result = 6;
        } else if x == 7 {
            result = 7;
        } else if x == 8 {
            result = 8;
        } else if x == 9 {
            result = 9;
        }
    }
    println!(
        "if-else (int) time: {:<25} ms",
        format!("{:.10}", start.elapsed().as_secs_f64())
    );
    // Measure time for match-case with int
    let start = Instant::now();
    for _ in 0..num_tests {
        result = match x {
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 7,
            8 => 8,
            9 => 9,
            _ => 0,
        }
    }
    println!(
        "match-case (int) time: {:<25} ms",
        format!("{:.10}", start.elapsed().as_secs_f64())
    );
    // Measure time for if-else with string
    let start = Instant::now();
    for _ in 0..num_tests {
        if str == "one" {
            result = 1;
        } else if str == "two" {
            result = 2;
        } else if str == "three" {
            result = 3;
        } else if str == "four" {
            result = 4;
        } else if str == "five" {
            result = 5;
        } else if str == "six" {
            result = 6;
        } else if str == "seven" {
            result = 7;
        } else if str == "eight" {
            result = 8;
        } else if str == "nine" {
            result = 9;
        }
    }
    println!(
        "if-else (string) time: {:<25} ms",
        format!("{:.10}", start.elapsed().as_secs_f64())
    );

    // Measure time for match-case with string
    let start = Instant::now();
    for _ in 0..num_tests {
        result = match str {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        }
    }
    println!(
        "match-case (string) time: {:<25} ms",
        format!("{:.10}", start.elapsed().as_secs_f64())
    );
}
