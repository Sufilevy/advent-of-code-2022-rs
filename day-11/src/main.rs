mod monkey;

use monkey::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u32 {
    let monkeys: Vec<Monkey> = input.split("\n\r\n").map(Monkey::from_str).collect();
    0
}

fn puzzle_two(_input: &str) -> u32 {
    0
}
