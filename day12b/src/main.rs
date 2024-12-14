use core::str;
use std::collections::HashMap;
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

/// Measures the plots in the first row of a grid.
///
/// This function takes a slice of characters representing the first row of a grid
/// and returns a tuple containing a vector of plot indices and a hashmap of plot statistics.
///
/// The vector of plot indices labels each character in the row with a unique plot index.
/// The hashmap of plot statistics maps each plot index to a tuple containing the plot size and perimeter.
///
/// # Arguments
///
/// * `row` - A slice of characters representing the first row of a grid.
///
/// # Returns
///
/// A tuple containing:
///
/// * A vector of plot indices labeling each character in the row.
/// * A hashmap of plot statistics mapping each plot index to a tuple of plot size and perimeter.
///
/// # Examples
///
/// ```
/// let row = vec!['a', 'a', 'b', 'b', 'c'];
/// let (labeled_row, plot_stats) = measure_first_row(&row);
/// assert_eq!(labeled_row, vec![0, 0, 1, 1, 2]);
/// assert_eq!(plot_stats.get(&0), Some(&(2, 4)));
/// assert_eq!(plot_stats.get(&1), Some(&(2, 4)));
/// assert_eq!(plot_stats.get(&2), Some(&(1, 4)));
/// ```
fn measure_first_row(row: &[char]) -> (Vec<usize>, HashMap<usize, (usize, usize)>) {
    let mut plot_idx = 0;
    let mut left = 0;
    let mut right = left;
    let mut plot_stats = HashMap::new();
    let mut labeled_row = vec![0; row.len()];
    while left < row.len() {
        let plot_char = row[left];
        while right < row.len() && row[right] == plot_char {
            labeled_row[right] = plot_idx;
            right += 1;
        }
        plot_stats.insert(plot_idx, (right - left, 4));
        plot_idx += 1;
        left = right;
    }
    (labeled_row, plot_stats)
}

fn additional_perim_left(curr: &[char], prev: &[char], left: usize) -> usize {
    if curr[left] != prev[left] {
        return 2;
    } else if left > 0 && curr[left] == prev[left - 1] {
        return 2;
    }
    return 0;
}

fn additional_perim_right(curr: &[char], prev: &[char], right: usize) -> usize {
    if curr[right] != prev[right] {
        return 2;
    } else if right < curr.len() - 1 && curr[right] == prev[right + 1] {
        return 2;
    }
    return 0;
}

fn relabel(prev_plot_ids: &mut [usize], connected: &HashSet<usize>, new_id: usize) {
    for i in 0..prev_plot_ids.len() {
        if connected.contains(&prev_plot_ids[i]) {
            prev_plot_ids[i] = new_id;
        }
    }
}

fn measure_row(
    curr: &[char],
    prev: &[char],
    prev_plot_ids: &mut Vec<usize>,
    prev_plot_stats: &mut HashMap<usize, (usize, usize)>,
) -> (Vec<usize>, HashMap<usize, (usize, usize)>, usize) {
    // plots which are not continued in curr
    let mut discontinued = prev_plot_ids.iter().cloned().collect::<HashSet<_>>();
    let mut plot_ids = vec![0; curr.len()];
    let mut plot_stats = HashMap::new();
    let mut left = 0;
    let mut right = left;
    let mut unassigned_id = curr.len(); // cannot have been assigned in previous row
    while left < curr.len() {
        let mut connected = HashSet::new();
        let plot_char = curr[left];
        while right < curr.len() && curr[right] == plot_char {
            plot_ids[right] = unassigned_id;
            if prev[right] == plot_char {
                connected.insert(prev_plot_ids[right]);
            }
            right += 1;
        }
        // get the new stats and relabel the connected rectangles in prev
        let mut total_size = right - left;
        let rightmost = right - 1;
        let mut total_perim = additional_perim_left(curr, prev, left);
        if rightmost != left {
            total_perim += additional_perim_right(curr, prev, rightmost);
        }
        for pid in &connected {
            let stats = prev_plot_stats.remove(&pid);
            if let Some((size, perim)) = stats {
                total_size += size;
                total_perim += perim;
            }
        }
        discontinued = discontinued.difference(&connected).cloned().collect();
        plot_stats.insert(unassigned_id, (total_size, total_perim));
        prev_plot_stats.insert(unassigned_id, (total_size, total_perim));
        relabel(prev_plot_ids, &connected, unassigned_id);
        unassigned_id += 1;
        left = right;
    }
    let total_discontinued = discontinued
        .iter()
        .map(|pid| prev_plot_stats.get(pid).unwrap())
        .map(|(size, perim)| size * perim)
        .sum();
    let min_index = curr.len();
    // subtract curr.len() from all plots_ids
    for i in 0..plot_ids.len() {
        plot_ids[i] -= min_index;
    }
    // subtract curr.len() from all keys in plot_stats
    let plot_stats = plot_stats
        .into_iter()
        .map(|(key, value)| (key - min_index, value))
        .collect::<HashMap<_, _>>();
    (plot_ids, plot_stats, total_discontinued)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_row() {
        let curr = vec!['A', 'A', 'A', 'A'];
        let prev = vec!['A', 'B', 'B', 'A'];
        let mut prev_plot_ids = vec![0, 1, 1, 2];
        let mut prev_plot_stats = HashMap::new();
        prev_plot_stats.insert(0, (1, 4));
        prev_plot_stats.insert(1, (2, 4));
        prev_plot_stats.insert(2, (1, 4));
        let (plot_ids, plot_stats, total_discontinued) = measure_row(&curr, &prev, &mut prev_plot_ids, &mut prev_plot_stats);
        assert_eq!(plot_ids, vec![0, 0, 0, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(6, 8)));
        assert_eq!(total_discontinued, 4*2);
    }

    #[test]
    fn test_measure_first_row_single_plot() {
        let row = vec!['a', 'a', 'a', 'a'];
        let (labeled_row, plot_stats) = measure_first_row(&row);
        assert_eq!(labeled_row, vec![0, 0, 0, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(4, 4)));
    }

    #[test]
    fn test_measure_first_row_multiple_plots() {
        let row = vec!['a', 'a', 'b', 'b', 'c'];
        let (labeled_row, plot_stats) = measure_first_row(&row);
        assert_eq!(labeled_row, vec![0, 0, 1, 1, 2]);
        assert_eq!(plot_stats.get(&0), Some(&(2, 4)));
        assert_eq!(plot_stats.get(&1), Some(&(2, 4)));
        assert_eq!(plot_stats.get(&2), Some(&(1, 4)));
    }

    #[test]
    fn test_measure_first_row_alternating_plots() {
        let row = vec!['a', 'b', 'a', 'b'];
        let (labeled_row, plot_stats) = measure_first_row(&row);
        assert_eq!(labeled_row, vec![0, 1, 2, 3]);
        assert_eq!(plot_stats.get(&0), Some(&(1, 4)));
        assert_eq!(plot_stats.get(&1), Some(&(1, 4)));
        assert_eq!(plot_stats.get(&2), Some(&(1, 4)));
        assert_eq!(plot_stats.get(&3), Some(&(1, 4)));
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let grid: Grid = content.parse().unwrap();
    let price = price_map(&grid);
    println!("{:?}", price);
}
