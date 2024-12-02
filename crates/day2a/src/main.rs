use std::time::Instant;

fn is_valid_trend(nums: &[isize]) -> bool {
    nums.windows(2)
        .try_fold(0, |dir, window| {
            let (a, b) = (window[0], window[1]);

            if dir >= 0 && (1..=3).contains(&(b - a)) {
                Ok(1)
            } else if dir <= 0 && (1..=3).contains(&(a - b)) {
                Ok(-1)
            } else {
                Err(())
            }
        })
        .is_ok()
}

fn main() {
    let now = Instant::now();

    let ans = include_bytes!("input.txt")
        .split(|&c| c == b'\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            Some(
                line.split(|&c| c == b' ')
                    .map(|num| atoi::atoi::<isize>(num).unwrap())
                    .collect::<Vec<isize>>(),
            )
        })
        .filter(|nums| is_valid_trend(nums))
        .count();

    let elapsed = now.elapsed();

    println!("{:?}", ans);
    println!("{:?}", elapsed);
}
