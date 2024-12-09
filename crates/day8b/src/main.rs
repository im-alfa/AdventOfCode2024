use std::collections::HashMap;

const COLS: usize = 50;
const ROWS: usize = 50;

// const COLS: usize = 12;
// const ROWS: usize = 12;
const TOTAL: usize = COLS * ROWS;

fn main() {
    let mut counted_cells = [false; TOTAL];

    include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .enumerate()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<&u8, Vec<_>>, (posy, b)| {
                b.iter().enumerate().for_each(|(posx, b)| {
                    if *b != b'.' {
                        acc.entry(b).or_default().push((posy, posx));
                    }
                });
                acc
            },
        )
        .into_iter()
        .for_each(|chunk| {
            let cases = chunk.1;

            for (i, case) in cases.iter().enumerate() {
                let to_check = cases[0..i].iter().chain(&cases[i + 1..]);
                let y = case.0 as i32;
                let x = case.1 as i32;

                for check in to_check {
                    let dx = x - check.1 as i32;
                    let dy = y - check.0 as i32;

                    for gamma in 0..200 {
                        let dx = gamma * dx;
                        let dy = gamma * dy;

                        if dx + x < 0
                            || dy + y < 0
                            || dx + x >= COLS as i32
                            || dy + y >= ROWS as i32
                        {
                            break;
                        }

                        let idx = (dx + x) as usize + (dy + y) as usize * COLS;
                        if !counted_cells[idx] {
                            counted_cells[idx] = true;
                        }

                        for gamma in 0..200 {
                            let dx = gamma * dx;
                            let dy = gamma * dy;

                            if dx - x < 0
                                || dy - y < 0
                                || x - dx >= COLS as i32
                                || y - dy >= ROWS as i32
                            {
                                break;
                            }

                            let idx = (dx - x) as usize + (dy - y) as usize * COLS;
                            if !counted_cells[idx] {
                                counted_cells[idx] = true;
                            }
                        }
                    }
                }
            }
        });

    dbg!(counted_cells.iter().filter(|b| **b == true).count());
}
