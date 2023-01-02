mod simulation;
use simulation::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut simulation = P1Simulation::new();
    input
        .iter()
        .map(Motion::from_str)
        .for_each(|motion| simulation.apply_motion(motion));
    simulation.count_visited_positions()
}

fn puzzle_two(input: &[&str]) -> u32 {
    let mut simulation = P2Simulation::new();
    input
        .iter()
        .map(Motion::from_str)
        .for_each(|motion| simulation.apply_motion(motion));
    simulation.count_visited_positions()
}
