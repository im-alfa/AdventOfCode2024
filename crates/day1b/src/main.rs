use std::time::Instant;

fn main() {
    let start = Instant::now();

    let (mut vals, mut frequencies) = (Vec::with_capacity(1000), [0; 100_000]);

    include_str!("input.txt").lines().for_each(|l| {
        let mut iter = l.split("   ");
        vals.push(iter.next().unwrap().parse::<usize>().unwrap());
        let col2 = iter.next().unwrap().parse::<usize>().unwrap();
        frequencies[col2] += 1;
    });

    let ans = vals
        .into_iter()
        .map(|val| val * frequencies[val])
        .sum::<usize>();

    let elapsed = start.elapsed();

    println!("{}", ans);
    println!("{:?}", elapsed);
}
