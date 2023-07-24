mod climbing;

use climbing::*;
use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let height_map = HeightMap::from_str(input);
    let mut paths = height_map.generate_paths();
    paths.par_sort_by_key(|path| path.len());
    paths.first().unwrap().len() as u32
}

fn puzzle_two(_input: &[&str]) -> u32 {
    0
}
