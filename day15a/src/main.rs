use std::collections::HashSet;

type Position = (i64, i64);
type Instruction = (i64, i64);

#[derive(Debug, PartialEq)]
enum Cell {
    Free,
    Wall,
    Box,
}

struct Grid {
    width: i64,
    height: i64,
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
}

impl Grid {
    fn new(width: i64, height: i64, walls: HashSet<Position>, boxes: HashSet<Position>) -> Self {
        Grid {
            width,
            height,
            walls,
            boxes,
        }
    }

    fn get(&self, pos: Position) -> Cell {
        if self.walls.contains(&pos) {
            Cell::Wall
        } else if self.boxes.contains(&pos) {
            Cell::Box
        } else {
            Cell::Free
        }
    }

    fn set(&mut self, pos: Position, value: Cell) -> Cell {
        let old_value = self.get(pos);
        match old_value {
            Cell::Wall => {
                self.walls.remove(&pos);
            }
            Cell::Box => {
                self.boxes.remove(&pos);
            }
            _ => {}
        }
        match value {
            Cell::Wall => {
                self.walls.insert(pos);
            }
            Cell::Box => {
                self.boxes.insert(pos);
            }
            _ => {}
        }
        old_value
    }
    fn position_past_boxes(&self, first_box: Position, direction: (i64, i64)) -> Option<Position> {
        let mut current_pos = first_box;
        loop {
            let next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
            match self.get(next_pos) {
                Cell::Box => {
                    current_pos = next_pos;
                }
                Cell::Wall => {
                    return None;
                }
                Cell::Free => {
                    return Some(next_pos);
                }
            }
        }
    }

    fn total_boxes_gps(&self) -> i64 {
        //The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map plus its distance from the left edge of the map
        self.boxes.iter().map(|(x, y)| 100 * y + x).sum()
    }


}

fn robot_step(robot: Position, instruction: Instruction, grid: &mut Grid) -> Position {
    let new_pos = (robot.0 + instruction.0, robot.1 + instruction.1);
    let cell = grid.get(new_pos);
    match cell {
        Cell::Free => {
            new_pos
        }
        Cell::Box => {
            if let Some(shove_to) = grid.position_past_boxes(new_pos, instruction) {
                grid.set(new_pos, Cell::Free);
                grid.set(shove_to, Cell::Box);
                new_pos
            } else {
                robot
            }
        }
        Cell::Wall => {
            robot
        }
    }
}

fn robot_walk(robot: Position, instructions: &[Instruction], grid: &mut Grid) -> Position {
    let mut current_pos = robot;
    for instruction in instructions {
        current_pos = robot_step(current_pos, *instruction, grid);
    }
    current_pos
}

fn total_gps_after_walk(robot: Position, instructions: &[Instruction], grid: &mut Grid) -> i64 {
    let final_pos = robot_walk(robot, instructions, grid);
    grid.total_boxes_gps()
}

fn parse_input(input: &str) -> (Grid, Position, Vec<Instruction>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let grid_lines: Vec<&str> = parts[0].lines().collect();
    let instructions_str = parts[1].trim();

    let height = grid_lines.len() as i64;
    let width = grid_lines[0].len() as i64;

    let mut robot = (0, 0);
    let mut obstacles = HashSet::new();
    let mut boxes = HashSet::new();

    for (y, line) in grid_lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x as i64, y as i64);
            match ch {
                '#' => {
                    obstacles.insert(pos);
                }
                'O' => {
                    boxes.insert(pos);
                }
                '@' => {
                    robot = pos;
                }
                _ => {}
            }
        }
    }

    let instructions = instructions_str
        .chars()
        .map(|ch| {
            match ch {
                '^' => (0, -1),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => (0, 0), // Ignore invalid instructions
            }
        })
        .collect();

    (
        Grid::new(width, height, obstacles, boxes),
        robot,
        instructions,
    )
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let (mut grid, robot, instructions) = parse_input(&input);
    println!("{}", total_gps_after_walk(robot, &instructions, &mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        let (grid, robot, instructions) = parse_input(input);

        assert_eq!(grid.width, 8);
        assert_eq!(grid.height, 8);
        assert_eq!(robot, (2, 2));
        assert!(grid.walls.contains(&(0, 0)));
        assert!(grid.walls.contains(&(7, 7)));
        //
        let mut expected_walls = HashSet::new();
        for x in 0..8 {
            expected_walls.insert((x, 0));
            expected_walls.insert((x, 7));
        }
        for y in 0..8 {
            expected_walls.insert((0, y));
            expected_walls.insert((7, y));
        }
        expected_walls.insert((1, 2));
        expected_walls.insert((2, 4));
        println!("{:?}", expected_walls.difference(&grid.walls));
        println!("{:?}", &grid.walls.difference(&expected_walls));

        assert_eq!(grid.walls, expected_walls);
        let expected_boxes: HashSet<Position> =
            vec![(3, 1), (5, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .into_iter()
                .collect();
        assert!(expected_boxes == grid.boxes);
        assert_eq!(instructions.len(), 15);
        assert_eq!(instructions[0], (-1, 0));
        assert_eq!(instructions[1], (0, -1));
        assert_eq!(instructions[2], (0, -1));
        assert_eq!(instructions[3], (1, 0));
        assert_eq!(instructions[4], (1, 0));
        assert_eq!(instructions[5], (1, 0));
        assert_eq!(instructions[6], (0, 1));
        assert_eq!(instructions[7], (0, 1));
        assert_eq!(instructions[8], (-1, 0));
        assert_eq!(instructions[9], (0, 1));
        assert_eq!(instructions[10], (1, 0));
        assert_eq!(instructions[11], (1, 0));
        assert_eq!(instructions[12], (0, 1));
        assert_eq!(instructions[13], (-1, 0));
        assert_eq!(instructions[14], (-1, 0));
    }

    // ...existing code...
}
