use rayon::prelude::*;
use std::time::Instant;

fn solve_dfs(terms: &[usize], prev: usize, res: usize) -> bool {
    if terms.len() == 0 || prev > res {
        return prev == res;
    }

    solve_dfs(&terms[1..], terms[0] * prev, res)
        || solve_dfs(&terms[1..], terms[0] + prev, res)
        || solve_dfs(
            &terms[1..],
            prev * 10usize.pow(terms[0].to_string().len() as u32),
            res,
        )
}

fn main() {
    let start = Instant::now();

    let input = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .par_bridge()
        .map(|l| {
            let column_pos = l.iter().position(|&b| b == b':').unwrap();
            let res = atoi::atoi::<usize>(&l[0..column_pos]).unwrap();
            let terms: Vec<_> = l[column_pos + 2..]
                .split(|&b| b == b' ')
                .map(|n| atoi::atoi::<usize>(n).unwrap())
                .collect();

            match solve_dfs(&terms[1..], terms[0], res) {
                true => res,
                false => 0,
            }
        })
        .sum::<usize>();

    let elapsed = start.elapsed();

    dbg!(elapsed, input);
}
