fn main() {
    let mut id: usize = 0;

    let data =
        include_bytes!("input.txt")
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, l)| {
                let num = atoi::ascii_to_digit::<usize>(*l).unwrap();
                match i % 2 {
                    0 => {
                        let vec = vec![Some(id); num];
                        id += 1;
                        acc.extend(vec);
                        acc
                    }
                    _ => {
                        acc.extend(vec![None; num]);
                        acc
                    }
                }
            });

    let mut rev_blocks = data
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, num)| num.is_some());

    let mut max_block = data.len();

    let sorted_blocks = data
        .iter()
        .enumerate()
        .filter_map(|(i, block)| {
            if i >= max_block {
                return None;
            }

            Some(match block {
                Some(b) => *b,
                None => {
                    let (index, block) = rev_blocks.next().unwrap();
                    let block = block.unwrap();
                    max_block = index;

                    block
                }
            })
        })
        .collect::<Vec<_>>();

    let ans = sorted_blocks
        .iter()
        .enumerate()
        .map(|(i, b)| i * b)
        .sum::<usize>();

    dbg!(ans);
}
