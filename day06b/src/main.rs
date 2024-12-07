use std::{collections::HashSet, time::Instant};
use itertools::Itertools;

// the grid dimensions, the start position and the positions of obstacles
fn parse_input(input: &[u8]) -> ((i64, i64), (i64, i64), HashSet<(i64, i64)>) {
    let mut obstacles = HashSet::new();
    let mut start_position = (0, 0);
    let width = input.split(|&b| b == b'\n').next().unwrap().len() as i64;
    let mut height = 0;
    for (y, line) in input.split(|&b| b == b'\n').enumerate() {
        height += 1;
        for (x, &ch) in line.iter().enumerate() {
            match ch {
                b'#' => {
                    obstacles.insert((x as i64, y as i64));
                }
                b'^' => {
                    start_position = (x as i64, y as i64);
                }
                _ => {}
            }
        }
    }
    ((width, height), start_position, obstacles)
}


fn turn_right(dir: (i64, i64)) -> (i64, i64) {
    (-dir.1, dir.0)
}

fn is_inbounds(pos: (i64, i64), width: i64, height: i64) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

/// given starting point and direction compute position where one hits next obstacle
/// None if out of bounds
fn walk_straight(
    start: (i64, i64),
    dir: (i64, i64),
    obstacles: &HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) -> Option<(i64, i64)> {
    let mut current_pos = start;
    // implement using walk_iter
    for (prev, next) in walk_iter(start, dir, obstacles, width, height).tuple_windows() {
        if prev.0 == next.0 {
            return Some(prev.0)
        }
    }
    None
}

fn step(pos: (i64, i64), dir: (i64, i64), obstacles: &HashSet<(i64, i64)>, width: i64, height: i64) -> Option<((i64, i64), (i64, i64))> {
    let next = (pos.0 + dir.0, pos.1 + dir.1);
    if obstacles.contains(&next) {
        return Some((pos, turn_right(dir)));
    }
    if is_inbounds(next, width, height) {
        return Some((next, dir));
    }
    return None;
}

fn walk_iter(start: (i64, i64), dir: (i64, i64), obstacles: &HashSet<(i64, i64)>, width: i64, height: i64) -> impl Iterator<Item = ((i64, i64), (i64, i64))> + '_ {
    std::iter::successors(Some((start, dir)), move |&(pos, dir)| {
        step(pos, dir, obstacles, width, height)
    })
}


/// Given start position and direction, walk until cycle is detected or out of bounds
/// If cycle is detected returned position is in cycle
/// If out of bounds returned position is position from which one went straight out of bounds
fn walk(
    start: (i64, i64),
    mut dir: (i64, i64),
    obstacles: &HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) -> ((i64, i64), bool) {
    let mut position = start;
    let mut corner_points = HashSet::new();
    while let Some(next) = walk_straight(position, dir, obstacles, width, height) {
        if corner_points.contains(&(position, dir)) {
            return (position, true);
        }
        corner_points.insert((position, dir));
        position = next;
        dir = turn_right(dir);
    }
    (position, false)
}

/// Given start position and direction, compute where along the way exactly one obstacle
/// could be placed to create lead to a cycle
fn obstacles_for_cycle(
    start: (i64, i64),
    dir: (i64, i64),
    obstacles: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) -> HashSet<(i64, i64)> {
    let mut visited = HashSet::new();
    let mut obstacles_for_cycle = HashSet::new();
    let mut position = start;
    let mut obstacles = obstacles;
    let mut dir = dir;
    while is_inbounds(position, width, height) {
        visited.insert(position);
        let in_front = (position.0 + dir.0, position.1 + dir.1);
        if obstacles.contains(&in_front) {
            dir = turn_right(dir);
        } else {
            if !visited.contains(&in_front) {
                // optimization: if we already visited position with dir its a cycle
                // but there are other ways in which one can get cycle
                obstacles.insert(in_front);
                let (_, cycle) = walk(position, turn_right(dir), &obstacles, width, height);
                obstacles.remove(&in_front);
                if cycle {
                    obstacles_for_cycle.insert(in_front);
                }
            }
            position = (position.0 + dir.0, position.1 + dir.1);   
        } 
    }
    return obstacles_for_cycle;
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let (grid_dimensions, start_position, obstacles) = parse_input(bytes);
    let start_time = Instant::now();
    let cycle_obstacles = obstacles_for_cycle(
        start_position,
        (0, -1),
        obstacles,
        grid_dimensions.0,
        grid_dimensions.1,
    );
    let duration = start_time.elapsed();

    println!("Time elapsed in obstacles_for_cycle() is: {:?}", duration);
    println!("{}", cycle_obstacles.len());
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
        assert_eq!(
            walk(
                expected_start_position,
                (0, -1),
                &expected_obstacles,
                expected_width,
                expected_height
            ),
            ((7, 7), false)
        );
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
        assert_eq!(
            walk(
                expected_start_position,
                (0, -1),
                &expected_obstacles,
                expected_width,
                expected_height
            ),
            ((2, 2), true)
        );
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
        dbg!(obstacles_for_cycle(
            expected_start_position,
            (0, -1),
            expected_obstacles.clone(),
            expected_width,
            expected_height
        ));
        assert_eq!(
            obstacles_for_cycle(
                expected_start_position,
                (0, -1),
                expected_obstacles,
                expected_width,
                expected_height
            )
            .len(),
            6
        );
    }
}
