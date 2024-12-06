use std::collections::HashSet;

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


fn walk(start: (u64, u64), dir: (i64, i64), obstacles: HashSet<(u64, u64)>, width: u64, height: u64) -> u64 {
    let mut visited = HashSet::new();
    let mut position = start;
    let mut dir = dir;
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
        } else {
            position = next_position;
        }
    }
    visited.len() as u64
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let (grid_dimensions, start_position, obstacles) = parse_input(bytes);
    println!("{:?}", walk(start_position, (0, -1), obstacles, grid_dimensions.0, grid_dimensions.1));
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
    fn test_walk() {
        let start = (4, 6);
        let dir = (0, -1);
        let mut obstacles = HashSet::new();
        obstacles.insert((4, 0));
        obstacles.insert((9, 1));
        obstacles.insert((2, 3));
        obstacles.insert((7, 4));
        obstacles.insert((1, 6));
        obstacles.insert((8, 7));
        obstacles.insert((0, 8));
        obstacles.insert((6, 9));
        let width = 10;
        let height = 10;

        assert_eq!(walk(start, dir, obstacles, width, height), 41);
    }
}