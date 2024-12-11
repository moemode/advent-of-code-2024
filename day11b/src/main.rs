use core::str;
use std::{
    collections::{HashMap, VecDeque},
    error,
};

fn parse_input(input: &[u8]) -> Result<VecDeque<u64>, Box<dyn error::Error>> {
    let s = str::from_utf8(input)?;
    let substrings: Vec<&str> = s.split_whitespace().collect();
    let numbers: VecDeque<u64> = substrings
        .into_iter()
        .map(|substr| substr.parse())
        .collect::<Result<VecDeque<u64>, _>>()?;
    Ok(numbers)
}

fn n_digits(x: u64) -> u64 {
    let mut n = 1;
    let mut x = x;
    while x > 9 {
        x /= 10;
        n += 1;
    }
    n
}

fn split(x: u64) -> (u64, u64) {
    let n = n_digits(x);
    let d = 10u64.pow((n / 2).try_into().unwrap());
    (x / d, x % d)
}

fn blink_memo(x: u64, k: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(&result) = memo.get(&(x, k)) {
        return result;
    }
    let result = if k == 0 {
        1
    } else if x == 0 {
        blink_memo(1, k - 1, memo)
    } else if n_digits(x) % 2 == 0 {
        let (a, b) = split(x);
        blink_memo(a, k - 1, memo) + blink_memo(b, k - 1, memo)
    } else {
        blink_memo(x * 2024, k - 1, memo)
    };
    memo.insert((x, k), result);
    result
}

fn blink_many(x: &VecDeque<u64>, k: u64) -> u64 {
    let mut memo = HashMap::new();
    let mut result = 0;
    for &x in x.iter() {
        result += blink_memo(x, k, &mut memo);
    }
    result
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let numbers = parse_input(bytes).unwrap();
    println! {"{}", blink_many(&numbers, 25)};
    println! {"{}", blink_many(&numbers, 75)};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndigits() {
        assert_eq!(n_digits(0), 1);
        assert_eq!(n_digits(9), 1);
        assert_eq!(n_digits(10), 2);
        assert_eq!(n_digits(99), 2);
        assert_eq!(n_digits(100), 3);
        assert_eq!(n_digits(999), 3);
        assert_eq!(n_digits(1000), 4);
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
