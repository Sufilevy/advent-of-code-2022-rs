mod sand_simulation;

use std::fs;

use sand_simulation::SandSimulation;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut sim = SandSimulation::from_lines(input);
    sim.simulate_sand_until_void()
}

fn puzzle_two(input: &[&str]) -> u32 {
    let mut sim = SandSimulation::from_lines(input);
    sim.set_floor();
    sim.simulate_sand_until_safe()
}
