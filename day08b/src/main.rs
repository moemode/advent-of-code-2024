use itertools::Itertools;
use num::Integer;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant; // We use the num crate for GCD calculation

type Position = (i64, i64);

fn parse_input(input: &str) -> (HashMap<char, HashSet<Position>>, i64, i64) {
    let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        height = y as i64 + 1;
        width = line.len() as i64;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().insert((x as i64, y as i64));
            }
        }
    }
    (antennas, width, height)
}

fn trace_line(
    start: Position,
    direction: (i64, i64),
    positions: &mut HashSet<Position>,
    width: i64,
    height: i64,
) {
    let mut current = start;
    while is_inbounds(current, width, height) {
        positions.insert(current);
        current = (current.0 + direction.0, current.1 + direction.1);
    }
}

fn antinodes(a0: Position, a1: Position, width: i64, height: i64) -> HashSet<Position> {
    let mut antis = HashSet::new();
    let diff = (a1.0 - a0.0, a1.1 - a0.1);
    let gcd = diff.0.abs().gcd(&diff.1.abs());
    let step_x = diff.0 / gcd;
    let step_y = diff.1 / gcd;
    trace_line(a0, (step_x, step_y), &mut antis, width, height);
    trace_line(a0, (-step_x, -step_y), &mut antis, width, height);
    antis
}

// Helper function to check if a position is within bounds
fn is_inbounds(position: Position, width: i64, height: i64) -> bool {
    let (x, y) = position;
    x >= 0 && x < width && y >= 0 && y < height
}

fn find_antinodes(
    antennas: &HashMap<char, HashSet<Position>>,
    width: i64,
    height: i64,
) -> HashSet<Position> {
    let mut all_antinodes = HashSet::new();
    for (_, positions) in antennas.iter() {
        for pair in positions.iter().combinations(2) {
            all_antinodes.extend(antinodes(*pair[0], *pair[1], width, height));
        }
    }
    all_antinodes
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let (antennas, width, height) = parse_input(&input);
    let start_time = Instant::now();
    let antis = find_antinodes(&antennas, width, height);
    let duration = start_time.elapsed();
    println!("Number of unique antinodes: {}", antis.len());
    println!("Time taken: {:?}", duration);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_antinodes() {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        let (antennas, width, height) = parse_input(&input);
        let antis = find_antinodes(&antennas, width, height);
        let expected_len = 1277; // Replace with the actual expected result for this input.
        assert_eq!(
            antis.len(),
            expected_len,
            "Antinode count does not match expected"
        );
    }
}
