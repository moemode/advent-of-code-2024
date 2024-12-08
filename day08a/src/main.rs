use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

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

fn antinodes(a0: &Position, a1: &Position) -> (Position, Position) {
    let diff = (a1.0 - a0.0, a1.1 - a0.1);
    let antinode1 = (a0.0 - diff.0, a0.1 - diff.1); // First antinode
    let antinode2 = (a1.0 + diff.0, a1.1 + diff.1); // Second antinode
    (antinode1, antinode2) // Return both antinodes as a tuple
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
            let (a0, a1) = (pair[0], pair[1]);
            let (antinode1, antinode2) = antinodes(a0, a1);
            if is_inbounds(antinode1, width, height) {
                all_antinodes.insert(antinode1);
            }
            if is_inbounds(antinode2, width, height) {
                all_antinodes.insert(antinode2);
            }
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
    println!("Number of unique antinodes: {}", antis.len());
    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
