use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn puzzle_two(input: &str) -> i32 {
    let mut elves = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();
    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum()
}

fn _puzzle_one(input: &[&str]) -> i32 {
    let mut most_calories = 0;

    let mut elf_sum = 0;
    for line in input.iter() {
        if line.is_empty() {
            if elf_sum > most_calories {
                most_calories = elf_sum;
            }
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }

    most_calories
}

fn _puzzle_two(input: &[&str]) -> i32 {
    let mut most_calories = [0; 3];

    let mut elf_sum = 0;
    for line in input.iter() {
        if line.is_empty() {
            most_calories = _add_elf(most_calories, elf_sum);
            most_calories.sort();
            most_calories.reverse();
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }

    most_calories.iter().sum()
}

fn _add_elf(most_calories: [i32; 3], elf: i32) -> [i32; 3] {
    if elf > most_calories[0] {
        [elf, most_calories[0], most_calories[1]]
    } else if elf > most_calories[1] {
        [most_calories[0], elf, most_calories[1]]
    } else if elf > most_calories[2] {
        [most_calories[0], most_calories[1], elf]
    } else {
        most_calories
    }
}
