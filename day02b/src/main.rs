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


fn differences(x: &[i64]) -> Vec<i64> {
    x.windows(2).map(|w| w[1] - w[0]).collect()
}

/// calculate the changes between successive numbers
/// +1 for increasing, -1 for decreasing, 0 for same
/// use differences
fn changes(diffs: &[i64]) -> Vec<i64> {
    diffs.iter().map(|&d| {
        if d > 0 {
            1
        } else if d < 0 {
            -1
        } else {
            0
        }
    }).collect()
}

// ensure successive numbers have differences between 1 and 3
fn has_small_differences(x: &[i64]) -> bool {
    x.windows(2).all(|w| {
        let d = (w[1] - w[0]).abs();
        1 <= d && d <= 3
    })
}

/// check if all but one change are in the same direction
/// else there is no hope
/// reason in terms of differences: difference between successive numbers and 
/// changes which are the sign of the differences
fn can_be_safe(x: &[i64]) -> bool {
    if (x.len() as i64) <= 1 {
        return true;
    }
    let diffs = differences(x);
    let chs = changes(&diffs);
    let total_change = chs.iter().sum::<i64>();
    let n_changes = x.len() as i64 - 1;
    // is strictly monotonically increasing or decreasing
    if total_change.abs() == n_changes {
        // if the left or rightmost point is too far from the rest, we can remove it
        // within a monotonically increasing or decreasing sequence removing makes things worse
        if (x[0]- x[1]).abs() > 3 {
            return has_small_differences(&x[1..]);
        } else if (x[x.len()-1] - x[x.len()-2]).abs() > 3 {
            return has_small_differences(&x[..x.len()-1]);
        }
        return has_small_differences(x)
    }
    // all changes except 1 are in the same direction (the other change could be 0, so >=)
    if total_change.abs() >= n_changes - 2 {
        let trend = if total_change > 0 { 1 } else { -1 };
        // find first element in ch that is 0 or opposite to trend (-trend)
        let mut i = 0;
        while i < chs.len() && chs[i] != 0 && chs[i] != -trend {
            i += 1;
        }
        // i-th change corresponds to i-th and i+1-th element
        // only hope is that removing one of them makes the rest safe
        let safe_remove_i = is_safe(&x[..i].iter().chain(&x[i+1..]).copied().collect::<Vec<i64>>());
        let safe_remove_i1 = is_safe(&x[..i+1].iter().chain(&x[i+2..]).copied().collect::<Vec<i64>>());
        return safe_remove_i || safe_remove_i1;
    }
    false
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
        if can_be_safe(&numbers) {
            safe_count += 1;
        }
    }

    println!("Number of rows that can be safe: {}", safe_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[7, 6, 4, 2, 1])); // Safe because the levels are all decreasing by 1 or 2.
        assert!(!is_safe(&[1, 2, 7, 8, 9])); // Unsafe because 2 7 is an increase of 5.
        assert!(!is_safe(&[9, 7, 6, 2, 1])); // Unsafe because 6 2 is a decrease of 4.
        assert!(!is_safe(&[1, 3, 2, 4, 5])); // Unsafe because 1 3 is increasing but 3 2 is decreasing.
        assert!(!is_safe(&[8, 6, 4, 4, 1])); // Unsafe because 4 4 is neither an increase or a decrease.
        assert!(is_safe(&[1, 3, 6, 7, 9])); // Safe because the levels are all increasing by 1, 2, or 3.
    }

    #[test]
    fn test_can_be_safe() {
        assert!(can_be_safe(&[7, 6, 4, 2, 1])); // Safe as is.
        assert!(can_be_safe(&[1, 3, 2, 4, 5])); // Removing 3 makes it safe.
        assert!(can_be_safe(&[5, 4, 2, 3, 1])); // Removing 2 makes it safe.
        assert!(can_be_safe(&[8, 6, 4, 4, 1])); // Removing 4 makes it safe.
        assert!(can_be_safe(&[1]));
        assert!(can_be_safe(&[]));
        assert!(!can_be_safe(&[1, 2, 7, 8, 9])); // Unsafe because 2 7 is an increase of 5.
        assert!(!can_be_safe(&[9, 7, 6, 2, 1])); // Unsafe because 6 2 is a decrease of 4.
        assert!(!can_be_safe(&[1, 3, 2, 5, 4])); // Unsafe because either 3, 2 or 5, 4 will remain.
    }
}

