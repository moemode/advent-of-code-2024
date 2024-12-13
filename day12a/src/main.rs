use core::str;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::str::FromStr;
use std::{collections::VecDeque, error, vec};

struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Grid {
    fn new(width: usize, height: usize, data: Vec<char>) -> Grid {
        Grid {
            width,
            height,
            data,
        }
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x > 0 {
            result.push((x - 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        result
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.data[y * self.width + x]
    }
}

impl FromStr for Grid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = lines.len();
        let width = lines[0].len();
        let data: Vec<char> = lines.into_iter().flatten().collect();
        Ok(Grid::new(width, height, data))
    }
}

fn perimeter_increase(grid: &Grid, x: usize, y: usize) -> usize {
    4 - grid
        .adjacent(x, y)
        .iter()
        .filter(|(xn, yn)| grid.get(x, y) == grid.get(*xn, *yn))
        .count()
}

fn measure_plot(
    grid: &Grid,
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    visited.insert((x, y));
    let mut size = 1;
    let mut perim = perimeter_increase(grid, x, y);
    for (xn, yn) in grid.adjacent(x, y) {
        if visited.contains(&(xn, yn)) || grid.get(x, y) != grid.get(xn, yn) {
            continue;
        }
        let (size_inc, perim_inc) = measure_plot(grid, xn, yn, visited);
        size += size_inc;
        perim += perim_inc;
    }
    return (size, perim);
}

fn measure_map(grid: &Grid) -> Vec<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if visited.contains(&(x, y)) {
                continue;
            }
            let (size, perim) = measure_plot(grid, x, y, &mut visited);
            result.push((size, perim));
        }
    }
    result
}

fn price_map(grid: &Grid) -> usize {
    let measures = measure_map(grid);
    measures.iter().map(|(size, perim)| size * perim).sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let grid: Grid = content.parse().unwrap();
    let price = price_map(&grid);
    println!("{:?}", price);
}
