mod beacons;

use std::fs;

use beacons::*;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let (sensors, beacons) = parse_sensors_beacons(input);

    let edge_x = 10_000_000;
    let y = 2_000_000;
    let mut empty_positions = 0;
    for x in -edge_x..edge_x {
        let position = (x, y);
        if
            sensors.iter().any(|sensor| sensor.position_empty(position)) &&
            beacons.iter().all(|&beacon| position != beacon)
        {
            empty_positions += 1;
        }
    }

    empty_positions
}

fn puzzle_two(input: &[&str]) -> u64 {
    let (sensors, _) = parse_sensors_beacons(input);

    for sensor in &sensors {
        let position = sensor.find_in_outline(
            4_000_000,
            |position| !sensors.iter().any(|sensor| sensor.position_empty(position))
        );
        if let Some(position) = position {
            return (position.0 as u64) * 4_000_000u64 + (position.1 as u64);
        }
    }

    0
}
