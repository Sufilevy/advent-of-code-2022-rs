mod cpu;

use cpu::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}\n", puzzle_one(&input));
    puzzle_two(&input);
}

fn puzzle_one(input: &[&str]) -> i64 {
    let mut cpu = P1Cpu::new();
    for line in input {
        cpu.execute_instruction(line);
    }
    cpu.get_sum()
}

fn puzzle_two(input: &[&str]) {
    let mut cpu = P2Cpu::new();
    for line in input {
        cpu.execute_instruction(line);
    }
}
