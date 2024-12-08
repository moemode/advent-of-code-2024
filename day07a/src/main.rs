struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

// the grid dimensions, the start position and the positions of obstacles
fn parse_input(input: &[u8]) -> Vec<Equation> {
    let input_str = std::str::from_utf8(input).expect("Invalid UTF-8 sequence");
    let mut equations = Vec::new();
    for line in input_str.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value = parts[0].parse::<i64>().expect("Invalid test value");
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|num| num.parse::<i64>().expect("Invalid number"))
            .collect();
        equations.push(Equation {
            test_value,
            numbers,
        });
    }

    equations
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let equations = parse_input(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = b"190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        let equations = parse_input(input);

        assert_eq!(equations.len(), 9);
        assert_eq!(equations[0].test_value, 190);
        assert_eq!(equations[0].numbers, vec![10, 19]);
        assert_eq!(equations[1].test_value, 3267);
        assert_eq!(equations[1].numbers, vec![81, 40, 27]);
        assert_eq!(equations[2].test_value, 83);
        assert_eq!(equations[2].numbers, vec![17, 5]);
        assert_eq!(equations[3].test_value, 156);
        assert_eq!(equations[3].numbers, vec![15, 6]);
        assert_eq!(equations[4].test_value, 7290);
        assert_eq!(equations[4].numbers, vec![6, 8, 6, 15]);
        assert_eq!(equations[5].test_value, 161011);
        assert_eq!(equations[5].numbers, vec![16, 10, 13]);
        assert_eq!(equations[6].test_value, 192);
        assert_eq!(equations[6].numbers, vec![17, 8, 14]);
        assert_eq!(equations[7].test_value, 21037);
        assert_eq!(equations[7].numbers, vec![9, 7, 18, 13]);
        assert_eq!(equations[8].test_value, 292);
        assert_eq!(equations[8].numbers, vec![11, 6, 16, 20]);
    }
}
