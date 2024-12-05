use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

type PageRules = HashMap<usize, HashSet<usize>>;

fn parse_rules(rule_section: &[u8]) -> PageRules {
    rule_section
        .split(|&b| b == b'\n')
        .fold(PageRules::default(), |mut rules, rule| {
            let mut page_numbers = rule
                .split(|&b| b == b'|')
                .map(|b| atoi::atoi::<usize>(b).unwrap());

            let left_page = page_numbers.next().unwrap();
            let right_page = page_numbers.next().unwrap();

            rules
                .entry(right_page)
                .or_insert_with(HashSet::new)
                .insert(left_page);

            rules.entry(left_page).or_insert_with(HashSet::new);

            rules
        })
}

fn process_entry(entry: &[usize], rules: &PageRules) -> usize {
    let entry_set: HashSet<usize> = entry.iter().copied().collect();

    let sorted_pages: Vec<usize> = rules
        .iter()
        .filter_map(|(page, left_pages)| {
            if entry_set.contains(page) {
                let left_page_count = left_pages.intersection(&entry_set).count();
                Some((*page, left_page_count))
            } else {
                None
            }
        })
        .sorted_by_key(|&(_, left_page_count)| left_page_count)
        .map(|(page, _)| page)
        .collect();

    if sorted_pages != entry {
        sorted_pages[sorted_pages.len() / 2]
    } else {
        0
    }
}

fn main() {
    let start = Instant::now();
    let input = include_bytes!("input.txt");
    let mid_pos = input.windows(2).position(|b| b == b"\n\n").unwrap();
    let rules = parse_rules(&input[0..mid_pos]);

    let entries: usize = input[mid_pos + 2..]
        .split(|&b| b == b'\n')
        .map(|entry_line| {
            entry_line
                .split(|&b| b == b',')
                .map(|n| atoi::atoi(n).unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|entry| process_entry(&entry, &rules))
        .sum();

    let elapsed = start.elapsed();
    dbg!(entries, elapsed);
}
