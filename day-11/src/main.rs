mod monkey;

use monkey::*;
use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\r\n").map(Monkey::from_str).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((target_monkey, item)) = monkeys[i].inspect_item() {
                monkeys[target_monkey].add_item(item);
            }
        }
    }

    monkeys.sort_by_key(|b| cmp::Reverse(b.num_inspections()));
    monkeys[0].num_inspections() * monkeys[1].num_inspections()
}

fn puzzle_two(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\r\n").map(Monkey::from_str).collect();

    let divisor = monkeys.iter().map(|monkey| monkey.divisible_by()).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some((target_monkey, item)) = monkeys[i].inspect_item_calmly(divisor) {
                monkeys[target_monkey].add_item(item);
            }
        }
    }

    monkeys.sort_by_key(|b| cmp::Reverse(b.num_inspections()));
    monkeys[0].num_inspections() * monkeys[1].num_inspections()
}
