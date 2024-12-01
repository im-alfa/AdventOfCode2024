use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    let vals = include_str!("input.txt")
        .lines()
        .map(|l| {
            let mut iter = l.split("   ");
            let col1 = iter.next().unwrap().parse::<i32>().unwrap();
            let col2 = iter.next().unwrap().parse::<i32>().unwrap();

            *frequencies.entry(col2).or_insert(0) += 1;

            col1
        })
        .collect::<Vec<i32>>();

    let ans = vals
        .into_iter()
        .map(|val| val * frequencies.get(&val).unwrap_or(&0))
        .sum::<i32>();

    let elapsed = start.elapsed();

    println!("{}", ans);
    println!("{:?}", elapsed);
}
