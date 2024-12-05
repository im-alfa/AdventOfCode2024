use std::time::Instant;

fn main() {
    let start = Instant::now();

    let lines = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let mut c = 0;

    for (nline, line) in lines.clone().into_iter().enumerate() {
        if nline == 0 || nline == lines.len() - 1 {
            continue;
        }

        let a_pos = line
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'A')
            .map(|(i, _)| i);

        for xpos in a_pos {
            if xpos == 0 || xpos == line.len() - 1 {
                continue;
            }

            match (
                // NW
                lines[nline - 1][xpos - 1],
                // SW
                lines[nline + 1][xpos - 1],
                // NE
                lines[nline - 1][xpos + 1],
                // SE
                lines[nline + 1][xpos + 1],
            ) {
                (b'M', b'M', b'S', b'S') => {
                    c += 1;
                }
                (b'S', b'S', b'M', b'M') => {
                    c += 1;
                }
                (b'S', b'M', b'S', b'M') => {
                    c += 1;
                }
                (b'M', b'S', b'M', b'S') => {
                    c += 1;
                }
                _ => {}
            }
        }
    }

    let elapsed = start.elapsed();

    dbg!(elapsed, c);
}
