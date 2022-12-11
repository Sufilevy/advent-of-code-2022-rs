use ndarray::Array2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let trees = Trees::parse(input);
    let mut visible_trees = trees.side_len * 2 + (trees.side_len - 2) * 2; // Border

    for x in 1..trees.side_len - 1 {
        for y in 1..trees.side_len - 1 {
            if trees.is_visible(x, y) {
                visible_trees += 1;
            }
        }
    }

    visible_trees as u32
}

fn puzzle_two(input: &[&str]) -> u32 {
    let trees = Trees::parse(input);

    (1..trees.side_len - 1)
        .map(|x| {
            (1..trees.side_len - 1)
                .map(|y| trees.scenic_view_of(x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

struct Trees {
    grid: Array2<u32>,
    side_len: usize,
}

impl Trees {
    fn parse(input: &[&str]) -> Self {
        let side_len = input.len();
        let mut trees = input
            .iter()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()));

        Self {
            grid: Array2::<u32>::from_shape_simple_fn((side_len, side_len), || {
                trees.next().unwrap()
            }),
            side_len,
        }
    }

    fn height_of(&self, x: usize, y: usize) -> u32 {
        *self.grid.get((y, x)).unwrap()
    }

    fn row_column_of(&self, x: usize, y: usize) -> (Vec<u32>, Vec<u32>) {
        (self.grid.row(y).to_vec(), self.grid.column(x).to_vec())
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.height_of(x, y);

        let (row, column) = self.row_column_of(x, y);
        let slices = vec![&row[..x], &row[x + 1..], &column[..y], &column[y + 1..]];

        for slice in slices {
            if tree_height > *slice.iter().max().unwrap() {
                return true;
            }
        }

        false
    }

    fn scenic_view_of(&self, x: usize, y: usize) -> u32 {
        let tree_height = self.height_of(x, y);

        let (row, column) = self.row_column_of(x, y);
        let (row_left, column_up) = (&mut row[..x].to_vec(), &mut column[..y].to_vec());
        row_left.reverse();
        column_up.reverse();

        let slices = vec![
            &row_left[..],
            &row[x + 1..],
            &column_up[..],
            &column[y + 1..],
        ];

        let mut scenic_view = 1;
        for slice in slices {
            let mut current_scenic_view =
                slice.iter().take_while(|&tree| *tree < tree_height).count();
            if current_scenic_view < slice.len() {
                current_scenic_view += 1;
            }
            if current_scenic_view == 0 {
                return 0;
            }
            scenic_view *= current_scenic_view;
        }

        scenic_view as u32
    }
}
