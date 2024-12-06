use std::time::Instant;

const ROWS: usize = 130;
const COLS: usize = 130;
const GRID_SIZE: usize = ROWS * COLS;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_u8(s: &u8) -> Self {
        match s {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => unreachable!("error parsing"),
        }
    }

    fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn to_bytes(self) -> u8 {
        match self {
            Direction::Up => b'^',
            Direction::Down => b'v',
            Direction::Left => b'<',
            Direction::Right => b'>',
        }
    }
}

fn main() {
    let start = Instant::now();

    let mut visited = [false; GRID_SIZE];

    let mut input = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|a| Vec::from(a))
        .collect::<Vec<_>>();

    let mut current_pos = input
        .iter()
        .enumerate()
        .find_map(|(posy, &ref b)| match b.iter().position(|&b| b == b'^') {
            Some(posx) => Some((posy, posx)),
            None => None,
        })
        .unwrap();

    visited[current_pos.0 * COLS + current_pos.1] = true;

    loop {
        let guard = input[current_pos.0][current_pos.1];
        let direction = Direction::from_u8(&guard);

        let new_pos = match direction {
            Direction::Up => (current_pos.0 - 1, current_pos.1),
            Direction::Down => (current_pos.0 + 1, current_pos.1),
            Direction::Left => (current_pos.0, current_pos.1 - 1),
            Direction::Right => (current_pos.0, current_pos.1 + 1),
        };

        let cur = input[new_pos.0][new_pos.1];

        if (new_pos.0 == ROWS - 1 || new_pos.0 == 0 || new_pos.1 == COLS - 1 || new_pos.1 == 0)
            && cur == b'.'
        {
            visited[new_pos.0 * COLS + new_pos.1] = true;
            break;
        }

        match cur {
            b'#' => {
                input[current_pos.0][current_pos.1] =
                    Direction::rotate(Direction::from_u8(&guard)).to_bytes();
            }
            b'.' => {
                input[current_pos.0][current_pos.1] = b'.';
                current_pos = new_pos;
                input[current_pos.0][current_pos.1] = guard;
            }
            _ => unreachable!("not reachable"),
        }

        visited[current_pos.0 * COLS + current_pos.1] = true;
    }

    let ans = visited.iter().filter(|&b| *b == true).count();
    let elapsed = start.elapsed();

    dbg!(ans, elapsed);
}
