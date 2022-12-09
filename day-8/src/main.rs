use ndarray::Array2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let (trees, side_len) = parse_trees(input);
    let mut visible_trees = side_len * 2 + (side_len - 2) * 2;

    for x in 1..side_len - 1 {
        'y: for y in 1..side_len - 1 {
            if is_tree_visible(&trees, x, y) {
                visible_trees += 1;
                continue 'y;
            }
        }
    }

    visible_trees as u32
}

fn parse_trees(input: &[&str]) -> (Array2<u32>, usize) {
    let side_len = input.len();
    let mut trees = input
        .iter()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()));
    (
        Array2::<u32>::from_shape_simple_fn((side_len, side_len), || trees.next().unwrap()),
        side_len,
    )
}

fn is_tree_visible(trees: &Array2<u32>, x: usize, y: usize) -> bool {
    let tree_height = *trees.get((y, x)).unwrap();

    let (row, column) = (trees.row(y).to_vec(), trees.column(x).to_vec());
    let slices = vec![&row[..x], &row[x + 1..], &column[..y], &column[y + 1..]];

    for slice in slices {
        if tree_height > *slice.iter().max().unwrap() {
            return true;
        }
    }

    false
}

fn puzzle_two(input: &[&str]) -> u32 {
    0
}
