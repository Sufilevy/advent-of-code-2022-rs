use bitvec::prelude::*;

type CaveArray = BitArr!(for SandSimulation::ARRAY_LENGTH);
type Point = (usize, usize);

pub struct SandSimulation {
    cave: CaveArray,
}

impl SandSimulation {
    const WIDTH: usize = 680;
    const HEIGHT: usize = 160;
    const ARRAY_LENGTH: usize = Self::WIDTH * Self::HEIGHT;

    const SAND_ORIGIN: Point = (500, 0);
    const VOID_HEIGHT: usize = 158;
    const FLOOR_HEIGHT: usize = 158;

    fn new() -> Self {
        let cave = bitarr![0; SandSimulation::ARRAY_LENGTH];
        Self { cave }
    }

    fn get(&self, pos: Point) -> bool {
        assert!(pos.0 < Self::WIDTH);
        assert!(pos.1 < Self::HEIGHT);

        let index = pos.1 * Self::WIDTH + pos.0;
        *self.cave
            .get(index)
            .unwrap_or_else(|| panic!("failed to access bit array at ({}, {})", pos.0, pos.1))
    }

    fn set(&mut self, pos: Point, value: bool) {
        assert!(pos.0 < Self::WIDTH);
        assert!(pos.1 < Self::HEIGHT);

        let index = pos.1 * Self::WIDTH + pos.0;
        self.cave.set(index, value);
    }

    pub fn from_lines(lines: &[&str]) -> Self {
        let mut cave = Self::new();

        lines
            .iter()
            .map(|&line| Self::parse_line(line))
            .for_each(|path| cave.add_rock_path(&path));

        cave
    }

    fn parse_line(line: &str) -> Vec<Point> {
        line.split(" -> ").map(Self::parse_point).collect()
    }

    fn parse_point(point: &str) -> Point {
        let mut parts = point.split(',');
        (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
    }

    fn add_rock_path(&mut self, path: &[Point]) {
        for i in 0..path.len() - 1 {
            let (mut first, second) = (path[i], path[i + 1]);

            // Move horizontally
            while first.0 != second.0 {
                self.set(first, true);
                if first.0 < second.0 {
                    first.0 += 1;
                } else {
                    first.0 -= 1;
                }
            }

            // Move vertically
            while first.1 != second.1 {
                self.set(first, true);
                if first.1 < second.1 {
                    first.1 += 1;
                } else {
                    first.1 -= 1;
                }
            }
        }
        self.set(*path.last().unwrap(), true);
    }

    pub fn simulate_sand_until_void(&mut self) -> u32 {
        let mut num_particles = 0;
        loop {
            let mut sand = Self::SAND_ORIGIN;

            // Simulate falling
            loop {
                if sand.1 >= Self::VOID_HEIGHT {
                    return num_particles;
                }

                if !self.get((sand.0, sand.1 + 1)) {
                    // Go down
                    sand.1 += 1;
                } else if !self.get((sand.0 - 1, sand.1 + 1)) {
                    // Go down left
                    sand.0 -= 1;
                    sand.1 += 1;
                } else if !self.get((sand.0 + 1, sand.1 + 1)) {
                    // Go down right
                    sand.0 += 1;
                    sand.1 += 1;
                } else {
                    // Rest
                    self.set(sand, true);
                    break;
                }
            }

            num_particles += 1;
        }
    }

    pub fn set_floor(&mut self) {
        for x in 0..Self::WIDTH {
            self.set((x, Self::FLOOR_HEIGHT), true);
        }
    }

    pub fn simulate_sand_until_safe(&mut self) -> u32 {
        let mut num_particles = 0;
        loop {
            let mut sand = Self::SAND_ORIGIN;

            // Simulate falling
            loop {
                if !self.get((sand.0, sand.1 + 1)) {
                    // Go down
                    sand.1 += 1;
                } else if !self.get((sand.0 - 1, sand.1 + 1)) {
                    // Go down left
                    sand.0 -= 1;
                    sand.1 += 1;
                } else if !self.get((sand.0 + 1, sand.1 + 1)) {
                    // Go down right
                    sand.0 += 1;
                    sand.1 += 1;
                } else {
                    // Rest
                    if sand == Self::SAND_ORIGIN {
                        return num_particles + 1;
                    }

                    self.set(sand, true);
                    break;
                }
            }

            num_particles += 1;
        }
    }
}
