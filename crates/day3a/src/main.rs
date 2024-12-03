use std::time::Instant;

use regex::bytes::Regex;

fn main() {
    let start = Instant::now();

    let rx = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let ans = rx
        .captures_iter(include_bytes!("input.txt"))
        .map(|c| {
            atoi::atoi::<usize>(c.get(1).unwrap().as_bytes()).unwrap()
                * atoi::atoi::<usize>(c.get(2).unwrap().as_bytes()).unwrap()
        })
        .sum::<usize>();

    let elapsed = start.elapsed();

    dbg!(ans, elapsed);
}
