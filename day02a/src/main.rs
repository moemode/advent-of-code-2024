use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe(n: &[i64]) -> bool {
    if n.len() < 1 {
        return true;
    }
    let increasing = n[0] <= n[1];
    let multiplier = if increasing { 1 } else { -1 };
    n.windows(2).all(|w| {
        let d = multiplier*(w[1] - w[0]);
        1 <= d && d <= 3
    })
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let numbers: Vec<i64> = line.split_whitespace()
                                    .map(|s| s.parse().expect("Could not parse number"))
                                    .collect();
        if is_safe(&numbers) {
            safe_count += 1;
        }
    }
    println!("Number of safe rows: {}", safe_count);
}
