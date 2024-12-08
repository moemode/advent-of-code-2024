use std::collections::HashSet;

struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

// the grid dimensions, the start position and the positions of obstacles
fn parse_input(input: &[u8]) -> Vec<Equation> {
    let input_str = std::str::from_utf8(input).expect("Invalid UTF-8 sequence");
    input_str
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Invalid test value");
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Invalid number"))
                .collect();
            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn is_solvable(eq: &Equation) -> bool {
    values_le_test_value(eq).contains(&eq.test_value)
}

fn concat(l: i64, r: i64) -> i64 {
    let mut shift = 1;
    let mut rc = r;
    while rc >= 10 {
        shift *= 10;
        rc /= 10;
    }
    l * shift * 10 + r
}

fn values_le_test_value(eq: &Equation) -> HashSet<i64> {
    eq.numbers.iter().fold(HashSet::new(), |mut values, &num| {
        if values.is_empty() {
            values.insert(num);
        } else {
            values = values
                .iter()
                .flat_map(|&value| vec![value + num, value * num, concat(value, num)])
                .filter(|&value| value <= eq.test_value)
                .collect();
        }
        values
    })
}

fn main() {
    let bytes = include_bytes!("../input.txt");
    let equations = parse_input(bytes);
    // time the call
    let start = std::time::Instant::now();
    let result: i64 = equations
        .iter()
        .filter(|eq| is_solvable(eq))
        .map(|eq| eq.test_value)
        .sum();
    let elapsed = start.elapsed();
    println!("Time: {:?}", elapsed);
    println!("{}", result);
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

    #[test]
    fn test_values() {
        let eq = Equation {
            test_value: 190,
            numbers: vec![10, 19],
        };
        let values = values_le_test_value(&eq);
        assert_eq!(values.len(), 2);
        assert!(values.contains(&190));
        assert!(values.contains(&29));
    }

    #[test]
    fn test_result() {
        let bytes = include_bytes!("../input.txt");
        let equations = parse_input(bytes);
        let result: i64 = equations
            .iter()
            .filter(|eq| is_solvable(eq))
            .map(|eq| eq.test_value)
            .sum();
        assert_eq!(result, 303766880536);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(23, 34), 2334);
    }

    #[test]
    fn test_concat2() {
        assert_eq!(concat2(23, 34), 2334);
    }
}
