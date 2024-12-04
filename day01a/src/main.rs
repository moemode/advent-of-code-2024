fn diff(l: &mut Vec<i64>, r: &mut Vec<i64>) -> i64 {
    l.sort_unstable();
    r.sort_unstable();
    l.iter().zip(r.iter()).map(|(a, b)| (a - b).abs()).sum()
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
    println!("{}", diff(&mut a, &mut b));
}
