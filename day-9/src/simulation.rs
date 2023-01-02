use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn from_char(char: &str) -> Self {
        match char {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            _ => Self::Down,
        }
    }
}

pub struct Motion {
    direction: Direction,
    distance: u32,
}

impl Motion {
    pub fn from_str(str: &&str) -> Self {
        let parts = str.split_whitespace().collect::<Vec<_>>();
        Self {
            direction: Direction::from_char(parts[0]),
            distance: parts[1].parse::<u32>().unwrap(),
        }
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn start() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_to_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    pub fn update_knot_horizontally(&mut self, head: Position) {
        // T . . H
        if head.x > self.x + 1 {
            self.x += 1;
        }
        // H . . T
        else if head.x < self.x - 1 {
            self.x -= 1;
        }
    }

    pub fn update_knot_vertically(&mut self, head: Position) {
        // H
        // .
        // .
        // T
        if head.y > self.y + 1 {
            self.y += 1;
        }
        // T
        // .
        // .
        // H
        else if head.y < self.y - 1 {
            self.y -= 1;
        }
    }

    pub fn update_knot_diagonally(&mut self, head: Position) {
        // . H
        // . .
        // T .
        // --or--
        // . . H
        // T . .
        if head.x > self.x && head.y > self.y + 1 || head.x > self.x + 1 && head.y > self.y {
            self.x += 1;
            self.y += 1;
        }
        // H .
        // . .
        // . T
        // --or--
        // H . .
        // . . T
        else if head.x < self.x && head.y > self.y + 1 || head.x < self.x - 1 && head.y > self.y {
            self.x -= 1;
            self.y += 1;
        }
        // T .
        // . .
        // . H
        // --or--
        // T . .
        // . . H
        else if head.x > self.x && head.y < self.y - 1 || head.x > self.x + 1 && head.y < self.y {
            self.x += 1;
            self.y -= 1;
        }
        // . T
        // . .
        // H .
        // --or--
        // . . T
        // H . .
        else if head.x < self.x && head.y < self.y - 1 || head.x < self.x - 1 && head.y < self.y {
            self.x -= 1;
            self.y -= 1;
        }
    }

    pub fn update_knot(&mut self, head: Position) {
        if self.y == head.y {
            self.update_knot_horizontally(head);
        } else if self.x == head.x {
            self.update_knot_vertically(head);
        } else {
            self.update_knot_diagonally(head)
        }
    }
}

pub struct P1Simulation {
    head: Position,
    tail: Position,
    visited_positions: HashSet<Position>,
}

impl P1Simulation {
    pub fn new() -> Self {
        Self {
            head: Position::start(),
            tail: Position::start(),
            visited_positions: HashSet::new(),
        }
    }

    pub fn apply_motion(&mut self, motion: Motion) {
        for _ in 0..motion.distance {
            self.head.move_to_direction(motion.direction);
            self.tail.update_knot(self.head);
            self.visited_positions.insert(self.tail);
        }
    }

    pub fn count_visited_positions(&self) -> u32 {
        self.visited_positions.len() as u32
    }
}

pub struct P2Simulation {
    knots: [Position; 10],
    visited_positions: HashSet<Position>,
}

impl P2Simulation {
    pub fn new() -> Self {
        Self {
            knots: [Position::start(); 10],
            visited_positions: HashSet::new(),
        }
    }

    fn update_knots(&mut self) {
        for knot in 1..10 {
            self.knots[knot].update_knot(self.knots[knot - 1]);
        }
    }

    pub fn apply_motion(&mut self, motion: Motion) {
        for _ in 0..motion.distance {
            self.knots[0].move_to_direction(motion.direction);
            self.update_knots();
            self.visited_positions.insert(self.knots[9]);
        }
    }

    pub fn count_visited_positions(&self) -> u32 {
        self.visited_positions.len() as u32
    }
}
