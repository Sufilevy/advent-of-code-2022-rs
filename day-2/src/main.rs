use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split('\n').collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));

    Ok(())
}

fn puzzle_one(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|round| puzzle_one_score_round(round.get(..=0).unwrap(), round.get(2..).unwrap()))
        .sum()
}

fn puzzle_one_score_round(elf: &str, player: &str) -> i32 {
    match (elf, player) {
        ("A", "X") => 4, // Rock, Rock
        ("A", "Y") => 8, // Rock, Paper
        ("A", "Z") => 3, // Rock, Scissors
        ("B", "X") => 1, // Paper, Rock
        ("B", "Y") => 5, // Paper, Paper
        ("B", "Z") => 9, // Paper, Scissors
        ("C", "X") => 7, // Scissors, Rock
        ("C", "Y") => 2, // Scissors, Paper
        ("C", "Z") => 6, // Scissors, Scissors
        (_, _) => 0,
    }
}

fn puzzle_two(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|round| puzzle_two_score_round(round.get(..=0).unwrap(), round.get(2..).unwrap()))
        .sum()
}

fn puzzle_two_score_round(elf: &str, player: &str) -> i32 {
    match (elf, player) {
        ("A", "X") => 3, // Rock, Lose
        ("A", "Y") => 4, // Rock, Draw
        ("A", "Z") => 8, // Rock, Win
        ("B", "X") => 1, // Paper, Lose
        ("B", "Y") => 5, // Paper, Draw
        ("B", "Z") => 9, // Paper, Win
        ("C", "X") => 2, // Scissors, Lose
        ("C", "Y") => 6, // Scissors, Draw
        ("C", "Z") => 7, // Scissors, Win
        (_, _) => 0,
    }
}
