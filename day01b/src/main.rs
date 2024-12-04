use std::collections::HashMap;

fn sim(l: &Vec<i64>, r: &Vec<i64>) -> i64 {
    let mut r_counts = HashMap::new();
    for i in r.iter() {
        *r_counts.entry(i).or_insert(0) += 1;
    }
    l.iter().map(|i| i * r_counts.get(i).unwrap_or(&0)).sum()
}

fn main() {
    let (mut a, mut b) = (Vec::with_capacity(1000), Vec::with_capacity(1000));
    let num_len = include_bytes!("../input.txt")
        .iter()
        .position(|&b| b == b' ')
        .unwrap();

    for line in include_bytes!("../input.txt").split(|&b| b == b'\n') {
        a.push(atoi::atoi::<i64>(&line[0..num_len]).unwrap());
        b.push(atoi::atoi::<i64>(&line[num_len + 3..]).unwrap());
    }
    println!("{}", sim(&a, &b));
}
