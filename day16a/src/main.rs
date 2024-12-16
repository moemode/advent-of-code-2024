use std::collections::HashSet;
use std::fmt;

type Position = (i64, i64);
type Instruction = (i64, i64);

#[derive(Debug, PartialEq)]
enum Cell {
    Free,
    Wall,
}

struct Grid {
    width: i64,
    height: i64,
    walls: HashSet<Position>,
    start: Position,
    end: Position,
}

impl Grid {
    fn new(
        width: i64,
        height: i64,
        walls: HashSet<Position>,
        start: Position,
        end: Position,
    ) -> Self {
        Grid {
            width,
            height,
            walls,
            start,
            end,
        }
    }

    /// Get the content of the cell at position `pos`.
    /// Get the content of the cell at position `pos`.
    fn get(&self, pos: Position) -> Cell {
        if self.walls.contains(&pos) {
            Cell::Wall
        } else {
            Cell::Free
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if pos == self.start {
                    write!(f, "S")?;
                } else if pos == self.end {
                    write!(f, "E")?;
                } else {
                    match self.get(pos) {
                        Cell::Free => write!(f, ".")?,
                        Cell::Wall => write!(f, "#")?,
                    }
                }
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Parse the input string into a Grid, robot position, and instructions.
fn parse_input(input: &str) -> (Grid, Position, Position) {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i64;
    let width = lines[0].len() as i64;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x as i64, y as i64);
            match ch {
                '#' => {
                    walls.insert(pos);
                }
                'S' => {
                    start = Some(pos);
                }
                'E' => {
                    end = Some(pos);
                }
                _ => {}
            }
        }
    }
    let start = start.expect("Start position 'S' not found in input");
    let end = end.expect("End position 'E' not found in input");
    let grid = Grid::new(width, height, walls, start, end);
    (grid, start, end)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let (mut grid, start, end) = parse_input(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_display_grid() {
        let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let (grid, start, end) = parse_input(input);
        assert_eq!(grid.width, 15);
        assert_eq!(grid.height, 15);
        // check that fmt output is the same as input
        assert_eq!(format!("{}", grid), input);
    }
}
