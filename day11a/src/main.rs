use core::str;
use std::{collections::VecDeque, error, vec};

fn parse_input(input: &[u8]) -> Result<VecDeque<u64>, Box<dyn error::Error>> {
    let s = str::from_utf8(input)?;
    let substrings: Vec<&str> = s.split_whitespace().collect();
    let numbers: VecDeque<u64> = substrings
        .into_iter()
        .map(|substr| substr.parse())
        .collect::<Result<VecDeque<u64>, _>>()?;
    Ok(numbers)
}

fn ndigits(x: u64) -> u64 {
    let mut n = 1;
    let mut x = x;
    while x > 9 {
        x /= 10;
        n += 1;
    }
    n
}

fn split(x: u64) -> (u64, u64) {
    let n = ndigits(x);
    let d = 10u64.pow((n / 2).try_into().unwrap());
    (x / d, x % d)
}

fn blink(x: u64) -> Vec<u64> {
    if x == 0 {
        return vec![1];
    } else if ndigits(x) % 2 == 0 {
        let (a, b) = split(x);
        return vec![a, b];
    }
    return vec![x * 2024];
}

fn blink_many(x: &VecDeque<u64>, k: u64) -> u64 {
    let mut x = x.clone();
    for _ in 0..k {
        let xlen = x.len();
        for _ in 0..xlen {
            let i = x.pop_front().unwrap();
            for j in blink(i) {
                x.push_back(j);
            }
        }
    }
    x.len() as u64
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let numbers = parse_input(bytes).unwrap();
    println!{"{}", blink_many(&numbers, 25)};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndigits() {
        assert_eq!(ndigits(0), 1);
        assert_eq!(ndigits(9), 1);
        assert_eq!(ndigits(10), 2);
        assert_eq!(ndigits(99), 2);
        assert_eq!(ndigits(100), 3);
        assert_eq!(ndigits(999), 3);
        assert_eq!(ndigits(1000), 4);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(1234), (12, 34));
        assert_eq!(split(56789), (567, 89));
        assert_eq!(split(0), (0, 0));
        assert_eq!(split(10), (1, 0));
        assert_eq!(split(1000), (10, 0));
    }
}
