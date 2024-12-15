use std::time::Instant;

const ROWS: usize = 53;
const COLS: usize = 53;
type Grid = Vec<Vec<u8>>;

fn pathfinder(x: usize, y: usize, grid: &Grid) -> usize {
    let current_number = grid[y][x];

    if current_number == 48 + 9 {
        return 1;
    }

    let mut sum = 0;

    for (nx, ny) in [
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
    ] {
        if (0..ROWS).contains(&ny) && (0..COLS).contains(&nx) {
            let next_number = grid[ny][nx];
            if next_number == current_number + 1 {
                sum += pathfinder(nx, ny, grid);
            }
        }
    }

    sum
}

fn main() {
    let start = Instant::now();

    let data = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|b| Vec::from(b))
        .collect::<Grid>();

    let trailheads = data
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, line)| {
            line.iter().enumerate().for_each(|(x, &byte)| {
                if byte == b'0' {
                    acc.push((x, y));
                }
            });

            acc
        });

    let ans = trailheads
        .iter()
        .map(|trailhead| pathfinder(trailhead.0, trailhead.1, &data))
        .sum::<usize>();

    let elapsed = start.elapsed();

    dbg!(elapsed, ans);
}
