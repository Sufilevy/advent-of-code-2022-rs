mod monkey;

use monkey::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u32 {
    let mut monkeys: Vec<Monkey> = input.split("\n\r\n").map(Monkey::from_str).collect();

    for _ in 0..20 {
        for monkey in &monkeys {
            // ...
        }
    }

    monkeys.sort_by_key(|b| std::cmp::Reverse(b.num_inspections()));
    monkeys[0].num_inspections() * monkeys[1].num_inspections()
}

fn puzzle_two(_input: &str) -> u32 {
    0
}
