use std::collections::HashSet;
use std::fmt;

type Position = (i64, i64);
type Instruction = (i64, i64);

#[derive(Debug, PartialEq)]
enum Cell {
    Free,
    Wall,
    Box(Position),
}

struct Grid {
    width: i64,
    height: i64,
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
    robot: Position
}

impl Grid {
    fn new(width: i64, height: i64, walls: HashSet<Position>, boxes: HashSet<Position>, robot: Position) -> Self {
        Grid {
            width,
            height,
            walls,
            boxes,
            robot
        }
    }

    fn get(&self, pos: Position) -> Cell {
        if self.walls.contains(&pos) {
            Cell::Wall
        } else if self.boxes.contains(&pos) {
            Cell::Box(pos)
        } else {
            // either free or right half of a box
            if self.boxes.contains(&(pos.0 - 1, pos.1)) {
                Cell::Box((pos.0 - 1, pos.1))
            } else {
                Cell::Free
            }
        }
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

    fn v_adjacent(&self, box_pos: Position, v_offset: i64) -> Vec<Position> {
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

    fn shovable(&self, box_pos: Position, direction: (i64, i64)) -> bool {
        // ensure no wall
        let next_l = (box_pos.0 + direction.0, box_pos.1 + direction.1);
        let next_r = (box_pos.0 + 1 + direction.0, box_pos.1 + direction.1);
        return !self.walls.contains(&next_l) && !self.walls.contains(&next_r);
    }

    fn total_boxes_gps(&self) -> i64 {
        self.boxes.iter().map(|(x, y)| 100 * y + x).sum()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if pos == self.robot {
                    write!(f, "@")?;
                } else {
                    match self.get(pos) {
                        Cell::Free => write!(f, ".")?,
                        Cell::Wall => write!(f, "#")?,
                        Cell::Box(left_half) => {
                            if pos == left_half {
                                write!(f, "[")?;
                            } else {
                                write!(f, "]")?;
                            }
                        }
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn robot_step(robot: Position, instruction: Instruction, grid: &mut Grid) -> Position {
    let nboxes = grid.boxes.len();
    let new_pos = (robot.0 + instruction.0, robot.1 + instruction.1);
    match grid.get(new_pos) {
        Cell::Free => new_pos,
        Cell::Box(box_pos) => {
            let connected_boxes = grid.connected_boxes(box_pos, instruction);
            if connected_boxes.iter().all(|&b| grid.shovable(b, instruction)) {
                // get the position of the shoved boxes
                let shoved_boxes = connected_boxes.iter().map(|&b| (b.0 + instruction.0, b.1 + instruction.1));
                // remove all connected boxes from grid.boxes
                for &b in connected_boxes.iter() {
                    grid.boxes.remove(&b);
                }
                // insert the shoved boxes
                grid.boxes.extend(shoved_boxes);
                if grid.boxes.len() != nboxes {
                    panic!("Boxes count changed");
                }
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
        grid.robot = current_pos;
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
    let width = 2 * grid_lines[0].len() as i64;

    let mut robot = (0, 0);
    let mut obstacles = HashSet::new();
    let mut boxes = HashSet::new();

    for (y, line) in grid_lines.iter().enumerate() {
        let mut x = 0;
        for (_, ch) in line.chars().enumerate() {
            let pos = (x, y as i64);
            match ch {
                '#' => {
                    obstacles.insert(pos);
                    obstacles.insert((x + 1, y as i64));
                }
                // O becomes [], implemented by get
                'O' => {
                    boxes.insert(pos);
                }
                // @ becomes @.
                '@' => {
                    robot = (x, y as i64);
                }
                _ => {}
            };
            x += 2;
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
        Grid::new(width, height, obstacles, boxes, robot),
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
    fn test_parse_display() {
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

        let (grid, _, _instructions) = parse_input(input);
        let expected_output = "\
################
##....[]..[]..##
####@...[]....##
##......[]....##
##..##..[]....##
##......[]....##
##............##
################
";
        assert_eq!(format!("{}", grid), expected_output);
    }

    #[test]
    fn test_parse_display_2() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>";

        let (grid, robot, _instructions) = parse_input(input);
        let expected_output = "\
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
";
        println!("{}", grid);
        assert_eq!(format!("{}", grid), expected_output);
    }

    #[test]
    fn test_robot_step() {
        let input = "\
########
#..O.O.#
##O.O..#
#...O..#
#.#.O..#
#.@OO..#
#......#
########

^
";
        let (mut grid, robot, _instructions) = parse_input(input);
        println!("{}", grid);
        let robot = robot_step(robot, (1, 0), &mut grid);
        let robot = robot_step(robot, (1, 0), &mut grid);
        grid.robot = robot;
        //println!("{}", grid);
        assert_eq!(robot, (6, 5));
    }


    #[test]
    fn test_gps_smaller() {
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
        let (mut grid, robot, _instructions) = parse_input(input);
        assert_eq!(total_gps_after_walk(robot, &_instructions, &mut grid), 9021);
    }

    #[test]
    fn test_gps_larger() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (mut grid, robot, _instructions) = parse_input(input);
        println!("{}", grid);
        assert_eq!(total_gps_after_walk(robot, &_instructions, &mut grid), 9021);
    }

    #[test]
    fn test_gps_new() {
        let input = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let (mut grid, robot, _instructions) = parse_input(input);
        assert_eq!(total_gps_after_walk(robot, &_instructions, &mut grid), 100*1+5+100*2+7+100*3+6);
    }
}
