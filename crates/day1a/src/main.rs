use std::time::Instant;

fn main() {
    let start = Instant::now();
    let (mut col1, mut col2) = (Vec::with_capacity(1000), Vec::with_capacity(1000));

    include_str!("input.txt").lines().for_each(|l| {
        let mut iter = l.split("   ");
        col1.push(iter.next().unwrap().parse::<u32>().unwrap());
        col2.push(iter.next().unwrap().parse::<u32>().unwrap());
    });

    col1.sort_unstable();
    col2.sort_unstable();

    let ans = col2
        .iter()
        .zip(col1)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>();

    let end = start.elapsed();

    println!("{}", ans);
    println!("{:?}", end);
}
