use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn range_from(input: &str) -> (u32, u32) {
    let mut parts = input.split('-');
    (
        parts.next().unwrap().parse::<u32>().unwrap(),
        parts.next().unwrap().parse::<u32>().unwrap(),
    )
}

fn puzzle_one(input: &[&str]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let (a_start, a_end) = range_from(parts.next().unwrap());
            let (b_start, b_end) = range_from(parts.next().unwrap());
            if a_start <= b_start && a_end >= b_end || b_start <= a_start && b_end >= a_end {
                Some(())
            } else {
                None
            }
        })
        .count() as u32
}

fn puzzle_two(input: &[&str]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let (a_start, a_end) = range_from(parts.next().unwrap());
            let (b_start, b_end) = range_from(parts.next().unwrap());
            if a_end < b_start || a_start > b_end {
                None
            } else {
                Some(())
            }
        })
        .count() as u32
}
