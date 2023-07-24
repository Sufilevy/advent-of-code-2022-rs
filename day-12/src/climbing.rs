use ndarray::Array2;
use rayon::prelude::*;

type Position = (usize, usize);
type Path = Vec<Position>;

enum PossiblePositions {
    End(Position),
    Positions(Vec<Position>),
}

#[derive(Debug)]
pub struct HeightMap {
    map: Array2<u8>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn height_from_char(c: u8) -> u8 {
        match c as char {
            'S' => 0,
            'E' => u8::MAX,
            _ => c - 96,
        }
    }

    pub fn from_str(input: &[&str]) -> Self {
        let (width, height) = (input[0].len(), input.len());
        let map = Array2::from_shape_fn((height, width), |(y, x)| {
            Self::height_from_char(input[y].as_bytes()[x])
        });

        Self { map, width, height }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.map[[y, x]]
    }

    fn get_start(&self) -> Position {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) == 0 {
                    return (x, y);
                }
            }
        }
        (0, 0)
    }

    fn get_possible_positions_for_step(&self, current_position: Position) -> PossiblePositions {
        let mut positions = Vec::new();
        let (x, y) = current_position;
        let current_height = self.get(x, y);

        // Up
        if y > 0 {
            let other = self.get(x, y - 1);

            if current_height == 26 && other == u8::MAX {
                return PossiblePositions::End((x, y - 1));
            } else if other == current_height || other == current_height + 1 {
                positions.push((x, y - 1));
            }
        }

        // Down
        if y < self.height - 1 {
            let other = self.get(x, y + 1);

            if current_height == 26 && other == u8::MAX {
                return PossiblePositions::End((x, y + 1));
            } else if other == current_height || other == current_height + 1 {
                positions.push((x, y + 1));
            }
        }

        // Left
        if x > 0 {
            let other = self.get(x - 1, y);

            if current_height == 26 && other == u8::MAX {
                return PossiblePositions::End((x - 1, y));
            } else if other == current_height || other == current_height + 1 {
                positions.push((x - 1, y));
            }
        }

        // Right
        if x < self.width - 1 {
            let other = self.get(x + 1, y);

            if current_height == 26 && other == u8::MAX {
                return PossiblePositions::End((x + 1, y));
            } else if other == current_height || other == current_height + 1 {
                positions.push((x + 1, y));
            }
        }

        PossiblePositions::Positions(positions)
    }

    fn rec_generate_paths(&self, mut path: Path, current_position: Position) -> Vec<Path> {
        path.push(current_position);

        let positions = self.get_possible_positions_for_step(current_position);

        match positions {
            PossiblePositions::End(end) => {
                path.push(end);
                println!("Reached the end-point!");
                vec![path]
            }
            PossiblePositions::Positions(positions) => {
                let positions: Vec<Position> = positions
                    .into_iter()
                    .filter(|position| !path.par_iter().any(|p| p == position))
                    .collect();

                if positions.is_empty() {
                    println!("Dead-end.");
                    return Vec::new();
                }

                let mut paths = Vec::new();
                for position in positions {
                    paths.extend(self.rec_generate_paths(path.clone(), position).into_iter());
                }

                paths
            }
        }
    }

    pub fn generate_paths(&self) -> Vec<Path> {
        let start = self.get_start();

        self.rec_generate_paths(Vec::new(), start)
            .into_par_iter()
            .map(|path| path[1..].to_vec())
            .collect()
    }
}
