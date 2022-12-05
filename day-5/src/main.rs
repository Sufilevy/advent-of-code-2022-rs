mod supplies;

use std::fs;
use supplies::*;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> String {
    let [stacks, instructions] = input.split("\n\n").collect::<Vec<_>>()[..] else { panic!("What...") };
    let mut supplies = Supplies::from(stacks);
    supplies.execute_instructions_9000(instructions);
    supplies.get_top()
}

fn puzzle_two(input: &str) -> String {
    let [stacks, instructions] = input.split("\n\n").collect::<Vec<_>>()[..] else { panic!("Hmph.") };
    let mut supplies = Supplies::from(stacks);
    supplies.execute_instructions_9001(instructions);
    supplies.get_top()
}
