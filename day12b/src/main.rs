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

    fn new_from_vecs(width: usize, height: usize, data: Vec<Vec<char>>) -> Grid {
        let data: Vec<char> = data.into_iter().flatten().collect();
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
            if prev[right] == plot_char {
                connected.insert(prev_plot_ids[right]);
            }
            right += 1;
        }
        // from connected keep the ones which are in plot_stats.keys
        // relableed will contain exactly one entry if we connect up to fields which have been relabeled because they are 
        // connected to other fields to the left of us with same character, e.g.
        // A B A
        // A A A
        let relabeled: Vec<usize> = connected.intersection(&plot_stats.keys().cloned().collect()).cloned().collect();
        let id = if relabeled.len() > 0 {
            *relabeled.iter().next().unwrap()
        } else {
            unassigned_id
        };
        for i in left..right {
            plot_ids[i] = id;
        }
        // get the new stats and relabel the connected rectangles in prev
        let mut total_size = right - left;
        let rightmost = right - 1;
        let mut total_perim = additional_perim_left(curr, prev, left);
        total_perim += additional_perim_right(curr, prev, rightmost);
        for pid in &connected {
            let stats = prev_plot_stats.remove(&pid);
            if let Some((size, perim)) = stats {
                total_size += size;
                total_perim += perim;
            }
        }
        discontinued = discontinued.difference(&connected).cloned().collect();
        plot_stats.insert(id, (total_size, total_perim));
        prev_plot_stats.insert(id, (total_size, total_perim));
        relabel(prev_plot_ids, &connected, id);
        unassigned_id += 1;
        left = right;
    }
    let discontinued_value = discontinued
        .iter()
        .map(|pid| prev_plot_stats.get(pid).unwrap())
        .map(|(size, perim)| size * perim)
        .sum();
    // print out all discontinued
    dbg!(&discontinued.iter().map(|pid| prev_plot_stats.get(pid).unwrap()).collect::<Vec<_>>());
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
    (plot_ids, plot_stats, discontinued_value)
}


fn price_map(grid: &Grid) -> usize {
    let mut value = 0;
    let (mut prev_plot_ids, mut prev_plot_stats) = measure_first_row(&grid.data[0..grid.width]);
    for y in 1..grid.height {
        let (plot_ids, plot_stats, discontinued) = measure_row(
            &grid.data[y * grid.width..(y + 1) * grid.width],
            &grid.data[(y - 1) * grid.width..y * grid.width],
            &mut prev_plot_ids,
            &mut prev_plot_stats,
        );
        prev_plot_ids = plot_ids;
        prev_plot_stats = plot_stats;
        value += discontinued;
    }
    let final_row_value = prev_plot_stats
        .values()
        .map(|(size, perim)| size * perim)
        .sum::<usize>();
    value + final_row_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_map_test_large() {
        let rows = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];
        let g = Grid::new_from_vecs(10, 10, rows);
        let price = price_map(&g);
        assert_eq!(price, 1206);
    }

    #[test]
    fn price_map_test() {
        let rows = vec![
            vec!['A', 'B', 'B', 'A'],
            vec!['A', 'A', 'A', 'A'],
        ];
        let g = Grid::new_from_vecs(4, 2, rows);
        let price = price_map(&g);
        assert_eq!(price, 8 + 6*8);
    }

    #[test]
    fn price_map_test_E() {
        let rows = vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'X', 'X', 'X', 'X'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'X', 'X', 'X', 'X'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ];
        let g = Grid::new_from_vecs(5, 5, rows);
        let price = price_map(&g);
        assert_eq!(price, 236);
    }

    #[test]
    fn price_map_test_complex() {
        let rows = vec![
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'B', 'B', 'A'],
            vec!['A', 'A', 'A', 'B', 'B', 'A'],
            vec!['A', 'B', 'B', 'A', 'A', 'A'],
            vec!['A', 'B', 'B', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
        ];
        let g = Grid::new_from_vecs(6, 6, rows);
        let price = price_map(&g);
        // Adjust the expected price based on your specific logic
        assert_eq!(price, 368);
    }

    #[test]
    fn test_measure_row() {
        let prev = vec!['A', 'B', 'B', 'A'];
        let curr = vec!['A', 'A', 'A', 'A'];
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
    fn test_measure_row_alt() {
        let prev = vec!['A', 'B', 'A', 'A', 'A', 'B', 'A'];
        let curr = vec!['A', 'A', 'A', 'B', 'A', 'A', 'A'];
        let mut prev_plot_ids = vec![0, 1, 2, 2, 2, 3, 4];
        let mut prev_plot_stats = HashMap::new();
        prev_plot_stats.insert(0, (1, 4));
        prev_plot_stats.insert(1, (1, 4));
        prev_plot_stats.insert(2, (3, 4));
        prev_plot_stats.insert(3, (1, 4));
        prev_plot_stats.insert(4, (1, 4));
        let (plot_ids, plot_stats, total_discontinued) = measure_row(&curr, &prev, &mut prev_plot_ids, &mut prev_plot_stats);
        assert_eq!(plot_ids, vec![0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(11, 16)));
        assert_eq!(plot_stats.get(&1), Some(&(1, 4)));
        assert_eq!(total_discontinued, 4 + 4); // the B's
    }

    #[test]
    fn test_measure_row_complex() {
        let prev = vec!['A', 'A', 'A', 'B', 'B', 'A'];
        let curr = vec!['A', 'A', 'A', 'A', 'A', 'A'];
        let mut prev_plot_ids = vec![0, 0, 0, 1, 1, 2];
        let mut prev_plot_stats = HashMap::new();
        prev_plot_stats.insert(0, (3, 4));
        prev_plot_stats.insert(1, (2, 4));
        prev_plot_stats.insert(2, (1, 4));
        let (plot_ids, plot_stats, total_discontinued) = measure_row(&curr, &prev, &mut prev_plot_ids, &mut prev_plot_stats);
        assert_eq!(plot_ids, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(10, 8)));
        assert_eq!(total_discontinued, 8);
    }

    #[test]
    fn test_measure_row_complex_interrupted() {
        let prev = vec!['A', 'A', 'A', 'B', 'B', 'A'];
        let curr = vec!['A', 'A', 'A', 'B', 'A', 'A'];
        let mut prev_plot_ids = vec![0, 0, 0, 1, 1, 0]; // simulate the last A being connected to the other A's by rows above
        let mut prev_plot_stats = HashMap::new();
        prev_plot_stats.insert(0, (14, 8));
        prev_plot_stats.insert(1, (2, 4));
        let (plot_ids, plot_stats, total_discontinued) = measure_row(&curr, &prev, &mut prev_plot_ids, &mut prev_plot_stats);
        assert_eq!(plot_ids, vec![0, 0, 0, 1, 0, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(14+5, 10)));
        assert_eq!(plot_stats.get(&1), Some(&(3, 6)));
        assert_eq!(total_discontinued, 0);
    }

    #[test]
    fn test_measure_row_complex_rev() {
        let prev = vec!['A', 'A', 'A', 'A', 'A', 'A'];
        let curr = vec!['A', 'A', 'A', 'B', 'B', 'A'];
        let mut prev_plot_ids = vec![0, 0, 0, 0, 0, 0];
        let mut prev_plot_stats = HashMap::new();
        prev_plot_stats.insert(0, (6, 4));
        let (plot_ids, plot_stats, total_discontinued) = measure_row(&curr, &prev, &mut prev_plot_ids, &mut prev_plot_stats);
        assert_eq!(plot_ids, vec![0, 0, 0, 1, 1, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(10, 8)));
        assert_eq!(plot_stats.get(&1), Some(&(2, 4)));
        assert_eq!(total_discontinued, 0);
    }

    #[test]
    fn test_measure_first_row_single_plot() {
        let row = vec!['a', 'a', 'a', 'a'];
        let (labeled_row, plot_stats) = measure_first_row(&row);
        assert_eq!(labeled_row, vec![0, 0, 0, 0]);
        assert_eq!(plot_stats.get(&0), Some(&(4, 4)));
    }

    #[test]
    fn test_measure_first_row_single_plot_mixed() {
        let row = vec!['a', 'a', 'a', 'a', 'b', 'a'];
        let (labeled_row, plot_stats) = measure_first_row(&row);
        assert_eq!(labeled_row, vec![0, 0, 0, 0, 1, 2]);
        assert_eq!(plot_stats.get(&0), Some(&(4, 4)));
        assert_eq!(plot_stats.get(&1), Some(&(1, 4)));
        assert_eq!(plot_stats.get(&2), Some(&(1, 4)));

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
