use std::time::Instant;

fn main() {
    let start = Instant::now();

    let (mut col1, mut col2): (Vec<_>, Vec<_>) = include_str!("input.txt")
        .lines()
        .map(|l| {
            let mut iter = l.split("   ");
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    col1.sort();
    col2.sort();

    let ans = col2
        .iter()
        .zip(col1.iter())
        .map(|a| (a.0 - a.1).abs())
        .sum::<i32>();

    let end = start.elapsed();

    println!("{}", ans);
    println!("{:?}", end);
}
