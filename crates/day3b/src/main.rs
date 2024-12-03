use std::time::Instant;

use regex::bytes::Regex;

fn main() {
    let start = Instant::now();
    let re = Regex::new(r"don\'t\(\)|do\(\)|(mul\((\d*),(\d*)\))").unwrap();

    let mut enabled = true;
    let mut total = 0;

    re.captures_iter(include_bytes!("input.txt"))
        .for_each(|match_| {
            match (match_.get(0), match_.get(1), match_.get(2), match_.get(3)) {
                (Some(a), None, None, None) => {
                    enabled = a.as_bytes() == b"do()";
                }
                _ => {
                    if enabled {
                        total += atoi::atoi::<usize>(match_.get(2).unwrap().as_bytes()).unwrap()
                            * atoi::atoi::<usize>(match_.get(3).unwrap().as_bytes()).unwrap()
                    }
                }
            };
        });

    let elapsed = start.elapsed();

    dbg!(elapsed, total);
}
