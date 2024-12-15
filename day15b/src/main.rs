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

    fn h_connected(&self, first_box: Position, direction: (i64, i64)) -> HashSet<Position> {
        let mut current_pos = first_box;
        let mut connected = HashSet::new();
        let step = (2 * direction.0, 0);
        while self.boxes.contains(&current_pos) {
            connected.insert(current_pos);
            current_pos = (current_pos.0 + step.0, current_pos.1);
        }
        connected
    }

    fn v_adjacent(&self, box_pos: Position, v_offset: i64) -> Vec<Position>{
        let mut adjacent = vec![];
        for dx in -1..=1 {
            let next_pos = (box_pos.0 + dx, box_pos.1 + v_offset);
            if self.boxes.contains(&next_pos) {
                adjacent.push(next_pos);
            }
        }
        adjacent
    }

    fn v_connected(&self, first_box: Position, direction: (i64, i64)) -> HashSet<Position> {
        let mut level_set = HashSet::new();
        level_set.insert(first_box);
        let mut next_level = HashSet::new();
        let mut connected = HashSet::new();
        while !level_set.is_empty() {
            for box_pos in level_set.iter() {
                connected.insert(*box_pos);
                for b in self.v_adjacent(*box_pos, direction.1) {
                    next_level.insert(b);
                }
            }
            level_set = next_level;
            next_level = HashSet::new();
        }
        connected
    }

    fn connected_boxes(&self, first_box: Position, direction: (i64, i64)) -> HashSet<Position> {
        if direction == (-1, 0) || direction == (1, 0) {
            return self.h_connected(first_box, direction);
        } else {
            return self.v_connected(first_box, direction);
        }
    }


    fn total_boxes_gps(&self) -> i64 {
        self.boxes.iter().map(|(x, y)| 100 * y + x).sum()
    }
}

fn robot_step(robot: Position, instruction: Instruction, grid: &mut Grid) -> Position {
    let new_pos = (robot.0 + instruction.0, robot.1 + instruction.1);
    match grid.get(new_pos) {
        Cell::Free => new_pos,
        Cell::Box => {
            if let Some(shove_to) = grid.position_past_boxes(new_pos, instruction) {
                grid.set(new_pos, Cell::Free);
                grid.set(shove_to, Cell::Box);
                new_pos
            } else {
                robot
            }
        }
        Cell::Wall => robot,
    }
}

fn robot_walk(robot: Position, instructions: &[Instruction], grid: &mut Grid) -> Position {
    let mut current_pos = robot;
    for &instruction in instructions {
        current_pos = robot_step(current_pos, instruction, grid);
    }
    current_pos
}

fn total_gps_after_walk(robot: Position, instructions: &[Instruction], grid: &mut Grid) -> i64 {
    robot_walk(robot, instructions, grid);
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
        let mut x_grid = 0;
        for (x, ch) in line.chars().enumerate() {
            let pos = (x_grid, y as i64);
            match ch {
                '#' => {
                    obstacles.insert(pos);
                    obstacles.insert((x_grid + 1, y as i64));
                    x_grid += 2;
                }
                'O' => {
                    boxes.insert(pos);
                    x_grid += 2;
                }
                '@' => {
                    robot = pos;
                    x_grid += 1;
                }
                _ => {
                    x_grid += 2;
                }
            };
        }
    }

    let instructions = instructions_str
        .chars()
        .map(|ch| match ch {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
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
        assert_eq!(grid.walls, expected_walls);

        let expected_boxes: HashSet<Position> =
            vec![(3, 1), (5, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .into_iter()
                .collect();
        assert_eq!(grid.boxes, expected_boxes);
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
