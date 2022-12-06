use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u32 {
    for i in 3..input.len() {
        let chars = &input[i - 3..=i];
        if !chars.chars().any(|c| chars.matches(c).count() > 1) {
            return i as u32 + 1;
        }
    }
    0
}

fn puzzle_two(input: &str) -> u32 {
    for i in 13..input.len() {
        let chars = &input[i - 13..=i];
        if !chars.chars().any(|c| chars.matches(c).count() > 1) {
            return i as u32 + 1;
        }
    }
    0
}
