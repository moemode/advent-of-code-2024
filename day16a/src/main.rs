use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt;

type Position = (i64, i64);
type Orientation = (i64, i64);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    position: Position,
    orientation: Orientation,
}

impl State {
    fn new(position: Position, orientation: Orientation) -> Self {
        State {
            position,
            orientation,
        }
    }
}

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

fn turn_right(orientation: Orientation) -> Orientation {
    (orientation.1, -orientation.0)
}

fn turn_left(orientation: Orientation) -> Orientation {
    (-orientation.1, orientation.0)
}

fn turn_around(orientation: Orientation) -> Orientation {
    (-orientation.0, -orientation.1)
}

fn all_orientations(s: State, cost: i64) -> Vec<(State, i64)> {
    let mut res = Vec::new();
    res.push((State::new(s.position, s.orientation), cost));
    res.push((
        State::new(s.position, turn_right(s.orientation)),
        cost + 1000,
    ));
    res.push((
        State::new(s.position, turn_left(s.orientation)),
        cost + 1000,
    ));
    res.push((
        State::new(s.position, turn_around(s.orientation)),
        cost + 2000,
    ));
    res
}

/// find shortest path from start to end
/// return -1 if no path is found
fn shortest_path_score(grid: &Grid, start: Position, end: Position) -> i64 {
    let mut pq = PriorityQueue::new();
    all_orientations(State::new(start, (1, 0)), 0)
        .into_iter()
        .for_each(|(s, c)| {
            pq.push(s, Reverse(c));
        });
    let mut visited = HashSet::new();
    while let Some((state, Reverse(cost))) = pq.pop() {
        if state.position == end {
            return cost;
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        let (x, y) = state.position;
        let (dx, dy) = state.orientation;
        let new_pos = (x + dx, y + dy);
        if grid.get(new_pos) == Cell::Wall {
            continue;
        }
        let new_states = all_orientations(State::new(new_pos, state.orientation), 1);
        new_states
            .iter()
            .filter(|(s, _)| !visited.contains(s))
            .for_each(|(s, c)| {
                pq.push(s.clone(), Reverse(cost + c));
            });
    }
    -1
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let (grid, start, end) = parse_input(&input);
    let score = shortest_path_score(&grid, start, end);
    println!("Shortest path score: {}", score);
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
