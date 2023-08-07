type Position = (i32, i32);

pub struct Sensor {
    position: Position,
    closest_beacon_distance: u32,
}

impl Sensor {
    pub fn from_string(str: &str) -> (Self, Position) {
        let mut parts = str.strip_prefix("Sensor at ").unwrap().split(": closest beacon is at ");
        let sensor = parse_position(parts.next().unwrap());
        let beacon = parse_position(parts.next().unwrap());

        (
            Self {
                position: sensor,
                closest_beacon_distance: manhattan_distance(sensor, beacon),
            },
            beacon,
        )
    }

    pub fn distance_to(&self, position: Position) -> u32 {
        manhattan_distance(self.position, position)
    }

    pub fn position_empty(&self, position: Position) -> bool {
        self.distance_to(position) <= self.closest_beacon_distance
    }

    pub fn find_in_outline<F>(&self, max_xy: i32, predicate: F) -> Option<Position>
        where F: Fn(Position) -> bool
    {
        let dist = self.closest_beacon_distance as i32;
        let (x, y) = self.position;

        // Top side of the rhombus
        let pos = (x, y - dist - 1);
        if position_in_range(pos, 0, max_xy) && predicate(pos) {
            return Some(pos);
        }

        // Bottom side of the rhombus
        let pos = (x, y + dist + 1);
        if position_in_range(pos, 0, max_xy) && predicate(pos) {
            return Some(pos);
        }

        for y_dist in -dist..dist {
            let x_dist = dist + 1 - y_dist.abs();

            // Right side of the rhombus
            let pos = (x + x_dist, y + y_dist);
            if position_in_range(pos, 0, max_xy) && predicate(pos) {
                return Some(pos);
            }

            // Left side of the rhombus
            let pos = (x - x_dist, y + y_dist);
            if position_in_range(pos, 0, max_xy) && predicate(pos) {
                return Some(pos);
            }
        }
        None
    }
}

fn manhattan_distance(pos1: Position, pos2: Position) -> u32 {
    (pos1.0 - pos2.0).unsigned_abs() + (pos1.1 - pos2.1).unsigned_abs()
}

pub fn parse_sensors_beacons(lines: &[&str]) -> (Vec<Sensor>, Vec<Position>) {
    lines
        .iter()
        .map(|&line| Sensor::from_string(line))
        .unzip()
}

fn parse_position(input: &str) -> Position {
    let mut parts = input.split(", ");
    let x = parts.next().unwrap().strip_prefix("x=").unwrap().parse().unwrap();
    let y = parts.next().unwrap().strip_prefix("y=").unwrap().parse().unwrap();
    (x, y)
}

fn position_in_range(pos: Position, min_xy: i32, max_xy: i32) -> bool {
    pos.0 > min_xy && pos.0 < max_xy && pos.1 > min_xy && pos.1 < max_xy
}
