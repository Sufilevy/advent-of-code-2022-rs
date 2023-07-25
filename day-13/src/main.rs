#![allow(unused)]

use std::{ fs, cmp::Ordering };

use packets::*;

mod packets;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> usize {
    parse_packets(input)
        .into_iter()
        .enumerate()
        .map(|(i, p)| if compare_packet_pair(&p) == Ordering::Greater { 0 } else { i + 1 })
        .sum()
}

fn puzzle_two(input: &[&str]) -> usize {
    0
}
