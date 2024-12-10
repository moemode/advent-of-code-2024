use core::hash;
use std::{alloc::handle_alloc_error, cmp::min};

struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

// the grid dimensions, the start position and the positions of obstacles
fn parse_input(input: &[u8]) -> Vec<Equation> {
    let input_str = std::str::from_utf8(input).expect("Invalid UTF-8 sequence");
    input_str
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Invalid test value");
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Invalid number"))
                .collect();
            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn string_to_digits(input: &[u8]) -> Vec<u8> {
    let mut digits = Vec::new();
    for &byte in input {
        if byte.is_ascii_digit() {
            digits.push(byte - b'0');
        }
    }
    digits
}

fn cksum(input: &[u8]) -> usize {
    if input.is_empty() {
        return 0;
    }
    let (mut l, mut r) = (0, input.len() - 1);
    let mut n_at_r = input[r];
    let mut sum = 0;
    let mut hole_sum;
    // forget about trailing hole
    if r % 2 == 1 {
        r -= 1;
    }
    let mut pos = 0;
    while l < r {
        let lfile_index = l / 2;
        for _ in 0..input[l] {
            sum += lfile_index * pos;
            pos += 1;
        }
        l += 1;
        (pos, l, r, n_at_r, hole_sum) = fill_hole(pos, l, input[l], r, n_at_r, input);
        sum += hole_sum;
    }
    if l == r {
        let lfile_index = l / 2;
        for _ in 0..n_at_r {
            sum += lfile_index * pos;
            pos += 1;
        }
    }
    sum
}

fn fill_hole(mut pos: usize, mut l: usize, mut n_hole: u8, mut r: usize, mut n_at_r: u8, input: &[u8]) -> (usize, usize, usize, u8, usize) {
    let mut sum = 0;
    while n_hole > 0 && l < r {
        let r_index = r / 2;
        let n_move = min(n_hole, n_at_r);
        for _ in 0..n_move{
            sum += pos * r_index;
            pos += 1;
            n_hole -= 1;
            n_at_r -= 1;
        } 
        if n_at_r == 0 {
            r -= 2;
            n_at_r = input[r];
        }
    }
    l += 1;
    return (pos, l, r, n_at_r, sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_hole_with_one() {
        let input = [2, 3, 3];
        let (pos, l, r, n_at_r, sum) = fill_hole(2, 1, 3, 2, 3, &input);
        assert_eq!(pos, 5);
        assert_eq!(l, 2);
        assert_eq!(r, 0);
        assert_eq!(n_at_r, 2);
        assert_eq!(sum, 2+3+4);
    }

    #[test]
    fn test_fill_hole_exactly() {
        let input = [2, 5, 3, 2, 1];
        let (pos, l, r, n_at_r, sum) = fill_hole(2, 1, 5, 4, 1, &input);
        assert_eq!(pos, 6);
        assert_eq!(l, 2);
        assert_eq!(r, 0);
        assert_eq!(n_at_r, 2);
        assert_eq!(sum, 2*2 + 3*1 + 4*1+5*1);
    }

    #[test]
    fn test_cksum() {
        // 2333133121414131402
        let input = [2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        let s = cksum(&input);
        assert_eq!(s, 1928);
    }
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let hd = string_to_digits(bytes);
    dbg!(hd.len());
    let s = cksum(&hd);
    println!("{}", s);    
}