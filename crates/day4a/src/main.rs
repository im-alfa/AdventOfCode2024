use std::time::Instant;

const XMAS: &[u8; 4] = b"XMAS";
const SAMX: &[u8; 4] = b"SAMX";

fn check_pattern(
    lines: &Vec<&[u8]>,
    nline: usize,
    x: usize,
    pattern: &[u8],
    horizontal: bool,
    backwards: bool,
) -> bool {
    for (xletter, &letter) in pattern.iter().enumerate() {
        // chk bounds
        if nline + xletter >= lines.len() {
            return false;
        }

        // determine direction
        let x_coord = match (horizontal, backwards) {
            (true, false) => x + xletter,
            (true, true) => x.wrapping_sub(xletter),
            (false, _) => x,
        };

        // bounds checks
        if (horizontal && x_coord >= lines[nline + xletter].len())
            || &lines[nline + xletter][x_coord] != &letter
        {
            return false;
        }

        if xletter == pattern.len() - 1 {
            return true;
        }
    }
    false
}

fn main() {
    let start = Instant::now();

    let lines = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let c1: usize = lines
        .clone()
        .into_iter()
        .map(|l| l.windows(4).filter(|b| b == XMAS || b == SAMX).count())
        .sum();

    let mut c2 = 0;

    for (nline, line) in lines.clone().into_iter().enumerate() {
        let x_pos = line
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'X')
            .map(|(i, _)| i);

        let s_pos = line
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'S')
            .map(|(i, _)| i);

        x_pos.for_each(|x| {
            // Right diagonal (forward)
            if check_pattern(&lines, nline, x, XMAS, true, false) {
                c2 += 1;
            }
            // Left diagonal (backward)
            if check_pattern(&lines, nline, x, XMAS, true, true) {
                c2 += 1;
            }
            // Vertical
            if check_pattern(&lines, nline, x, XMAS, false, false) {
                c2 += 1;
            }
        });

        s_pos.for_each(|x| {
            // Right diagonal (forward)
            if check_pattern(&lines, nline, x, SAMX, true, false) {
                c2 += 1;
            }
            // Left diagonal (backward)
            if check_pattern(&lines, nline, x, SAMX, true, true) {
                c2 += 1;
            }
            // Vertical
            if check_pattern(&lines, nline, x, SAMX, false, false) {
                c2 += 1;
            }
        });
    }

    let fc = c1 + c2;

    let elapsed = start.elapsed();

    dbg!(elapsed, fc);
}
