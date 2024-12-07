use std::collections::{HashSet, VecDeque};
use std::iter::successors;

fn is_inbounds(pos: (i64, i64), width: i64, height: i64) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

/// given starting point and position compute position where one hits next obstacle
/// None if out of bounds
fn walk_straight(start: (i64, i64), dir: (i64, i64), obstacles: &HashSet<(i64, i64)>, width: i64, height: i64) -> Option<(i64, i64)> {
    let mut position = start;
    loop {
        let next = (position.0 + dir.0, position.1 + dir.1);
        if !is_inbounds(next, width, height) {
            return None;
        }
        if obstacles.contains(&next) {
            return Some(position);
        }
        position = next;
    }
}

// store

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn is_cycle(distances: &VecDeque<i64>) -> bool {
    distances.len() == 4 && distances[0] == distances[2] && distances[1] == distances[3]
}

fn update_distances(distances: &mut VecDeque<i64>, new_distance: i64) {
    distances.push_back(new_distance);
    if distances.len() > 4 {
        distances.pop_front();
    }
}

fn walk(start: (i64, i64), mut dir: (i64, i64), obstacles: &HashSet<(i64, i64)>, width: i64, height: i64) -> ((i64, i64), bool) {
    let mut position = start;
    let mut last_four_distance = VecDeque::new();
    while let Some(next) = walk_straight(position, dir, obstacles, width, height) {
        let dist = distance(position, next);
        update_distances(&mut last_four_distance, dist);
        if is_cycle(&last_four_distance) {
            return (position, true);
        }
        position = next;
        dir = (-dir.1, dir.0);
    }
    (position, false)
}




// the grid dimensions, the start position and the positions of obstacles
fn parse_input(input: &[u8]) -> ((u64, u64), (u64, u64), HashSet<(u64, u64)>) {
    let mut obstacles = HashSet::new();
    let mut start_position = (0, 0);
    let mut width = 0;
    let mut height = 0;
    width = input.split(|&b| b == b'\n').next().unwrap().len() as u64;
    for (y, line) in input.split(|&b| b == b'\n').enumerate() {
        height += 1;
        for (x, &ch) in line.iter().enumerate() {
            match ch {
                b'#' => { obstacles.insert((x as u64, y as u64)); },
                b'^' => { start_position = (x as u64, y as u64); },
                _ => {},
            }
        }
    }
    ((width, height), start_position, obstacles)
}


fn obstacles_for_cycle(start: (u64, u64), dir: (i64, i64), obstacles: HashSet<(u64, u64)>, width: u64, height: u64) -> HashSet<(u64, u64)> {
    let mut turning_points = VecDeque::new();
    let mut visited = HashSet::new();
    let mut position = start;
    let mut dir = dir;
    let mut obstacles_for_cycle = HashSet::new();
    turning_points.push_back(position);
    loop {
        visited.insert(position);
        let next_position = (position.0.saturating_add_signed(dir.0), position.1.saturating_add_signed(dir.1));
        // stepped out left or up
        if next_position == position {
            break;
        }
        if next_position.0 >= width || next_position.1 >= height {
            break;
        }
        if obstacles.contains(&next_position) {
            // turn right
            dir = (-dir.1, dir.0);
            turning_points.push_back(position);
            if turning_points.len() > 3 {
                turning_points.pop_front();
            }
            if turning_points.len() == 3 {
                obstacles_for_cycle.extend(place_obstacle_for_cycle(turning_points[0], turning_points[2], dir, width, height, &obstacles));
            }
        } else {
            position = next_position;
        }
    }
    obstacles_for_cycle.remove(&start);
    obstacles_for_cycle
}

fn next_position(pos: &(u64, u64), dir: (i64, i64), width: u64, height: u64) -> Option<(u64, u64)> {
    let next = (pos.0.saturating_add_signed(dir.0), pos.1.saturating_add_signed(dir.1));
    if next == *pos || next.0 >= width || next.1 >= height {
        None
    } else {
        Some(next)
    }
}

// create an iterator which takes start position and direction and returns all positions until out of bounds
fn walk_positions(start: (u64, u64), dir: (i64, i64), width: u64, height: u64) -> impl Iterator<Item=(u64, u64)> {
    std::iter::successors(next_position(&start, dir, width, height), move |pos| next_position(pos, dir, width, height))
}

fn place_obstacle_for_cycle(p0: (u64, u64), p2: (u64, u64), dir: (i64, i64), width: u64, height: u64, obstacles: &HashSet<(u64, u64)>) -> HashSet<(u64, u64)> {
    let mut obstacles_for_cycle = HashSet::new();
    for p in walk_positions(p2, dir, width, height) {
        if obstacles.contains(&p) {
            break;
        }
        let prev = ((p.0 as i64 - dir.0) as u64, (p.1 as i64 - dir.1) as u64);
        // can one turn right from previous and go to the line between p0 and p1?
        for next in walk_positions(prev, turn_right(dir), width, height) {
            if obstacles.contains(&next) {
                break;
            }
            if next.0 == p0.0 || next.1 == p0.1 {
                if let Some(must_obstacle) = next_position(&next, turn_right(dir), width, height) {
                    if obstacles.contains(&must_obstacle) {
                        obstacles_for_cycle.insert(p);
                    }
                }
                break;
            }
        }
    }
    return obstacles_for_cycle;
}


fn turn_right(dir: (i64, i64)) -> (i64, i64) {
    (-dir.1, dir.0)
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let (grid_dimensions, start_position, obstacles) = parse_input(bytes);
    println!("{:?}", obstacles_for_cycle(start_position, (0, -1), obstacles, grid_dimensions.0, grid_dimensions.1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_parse_input() {
        let input = b"....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let expected_width = 10;
        let expected_height = 10;
        let expected_start_position = (4, 6);
        let mut expected_obstacles = HashSet::new();
        expected_obstacles.insert((4, 0));
        expected_obstacles.insert((9, 1));
        expected_obstacles.insert((2, 3));
        expected_obstacles.insert((7, 4));
        expected_obstacles.insert((1, 6));
        expected_obstacles.insert((8, 7));
        expected_obstacles.insert((0, 8));
        expected_obstacles.insert((6, 9));

        let (dimensions, start_position, obstacles) = parse_input(input);

        assert_eq!(dimensions, (expected_width, expected_height));
        assert_eq!(start_position, expected_start_position);
        assert_eq!(obstacles, expected_obstacles);
    }

    #[test]
    fn walk_test() {
        let expected_width = 10;
        let expected_height = 10;
        let expected_start_position = (4, 6);
        let mut expected_obstacles = HashSet::new();
        expected_obstacles.insert((4, 0));
        expected_obstacles.insert((9, 1));
        expected_obstacles.insert((2, 3));
        expected_obstacles.insert((7, 4));
        expected_obstacles.insert((1, 6));
        expected_obstacles.insert((8, 7));
        expected_obstacles.insert((0, 8));
        expected_obstacles.insert((6, 9));
        assert_eq!(walk(expected_start_position, (0, -1), &expected_obstacles, expected_width, expected_height), ((7,7), false));
    }

    #[test]
    fn walk_test_cycle() {
        let expected_width = 4;
        let expected_height = 4;
        let expected_start_position = (1, 2);
        let mut expected_obstacles = HashSet::new();
        expected_obstacles.insert((0, 2));
        expected_obstacles.insert((1, 0));
        expected_obstacles.insert((2, 3));
        expected_obstacles.insert((3, 1));
        assert_eq!(walk(expected_start_position, (0, -1), &expected_obstacles, expected_width, expected_height), ((2,2), true));
    }

    #[test]
    fn obstacles_for_cycle_test() {
        let expected_width = 10;
        let expected_height = 10;
        let expected_start_position = (4, 6);
        let mut expected_obstacles = HashSet::new();
        expected_obstacles.insert((4, 0));
        expected_obstacles.insert((9, 1));
        expected_obstacles.insert((2, 3));
        expected_obstacles.insert((7, 4));
        expected_obstacles.insert((1, 6));
        expected_obstacles.insert((8, 7));
        expected_obstacles.insert((0, 8));
        expected_obstacles.insert((6, 9));
        assert_eq!(obstacles_for_cycle(expected_start_position, (0, -1), expected_obstacles, expected_width, expected_height).len(), 6);
    }

    #[test]
    fn test_walk_positions_right() {
        let start = (0, 0);
        let dir = (1, 0);
        let width = 10;
        let height = 10;
        let positions: Vec<_> = walk_positions(start, dir, width, height).collect();
        let expected_positions: Vec<_> = (1..10).map(|x| (x, 0)).collect();
        assert_eq!(positions, expected_positions);
    }

    #[test]
    fn test_walk_positions_down() {
        let start = (0, 0);
        let dir = (0, 1);
        let width = 10;
        let height = 10;
        let positions: Vec<_> = walk_positions(start, dir, width, height).collect();
        let expected_positions: Vec<_> = (1..10).map(|y| (0, y)).collect();
        assert_eq!(positions, expected_positions);
    }

    #[test]
    fn test_walk_positions_diagonal() {
        let start = (0, 0);
        let dir = (1, 1);
        let width = 10;
        let height = 10;
        let positions: Vec<_> = walk_positions(start, dir, width, height).collect();
        let expected_positions: Vec<_> = (1..10).map(|i| (i, i)).collect();
        assert_eq!(positions, expected_positions);
    }

    #[test]
    fn test_walk_positions_out_of_bounds() {
        let start = (9, 9);
        let dir = (1, 1);
        let width = 10;
        let height = 10;
        let positions: Vec<_> = walk_positions(start, dir, width, height).collect();
        let expected_positions: Vec<_> = vec![];
        assert_eq!(positions, expected_positions);
    }
}