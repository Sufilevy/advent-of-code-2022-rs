mod filesystem;

use std::fs;

use filesystem::Terminal;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u64 {
    Terminal::new().interpret(input).sum_small_directories()
}

fn puzzle_two(input: &str) -> u64 {
    Terminal::new().interpret(input).find_sufficient_directory()
}
