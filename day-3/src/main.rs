use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn priority_of(item: char) -> u32 {
    let val = item as u32;
    match item {
        'a'..='z' => val - 96,
        'A'..='Z' => val - 38,
        _ => 0,
    }
}

fn puzzle_one(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|backpack| {
            let middle = backpack.len() / 2;
            let (first, second) = (&backpack[..middle], &backpack[middle..]);
            priority_of(first.chars().find(|item| second.contains(*item)).unwrap())
        })
        .sum()
}

fn puzzle_two(input: &[&str]) -> u32 {
    input
        .chunks(3)
        .map(|group| {
            let [first, second, third] = group else { panic!("Groups of 3, please!") };
            priority_of(
                first
                    .chars()
                    .find(|item| second.contains(*item) && third.contains(*item))
                    .unwrap(),
            )
        })
        .sum()
}
