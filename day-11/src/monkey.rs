#[derive(Debug)]
enum Target {
    Old,
    Num(u32),
}

impl Target {
    pub fn from_str(input: &str) -> Self {
        if input == "old" {
            Self::Old
        } else {
            Self::Num(input.parse::<u32>().unwrap())
        }
    }
}
#[derive(Debug)]
enum Operation {
    Multiply(Target),
    Add(Target),
}

impl Operation {
    pub fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let (operator, target) = (parts[1], parts[2]);

        if operator == "+" {
            Self::Add(Target::from_str(target))
        } else {
            Self::Multiply(Target::from_str(target))
        }
    }

    pub fn apply_to(&self, item: &mut u32) {
        let old = *item;

        let new = match self {
            Operation::Multiply(target) => match target {
                Target::Old => old * old,
                Target::Num(num) => old * num,
            },
            Operation::Add(target) => match target {
                Target::Old => old + old,
                Target::Num(num) => old + num,
            },
        };

        *item = new;
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisible_by: u32,
    monkey_a: u32,
    monkey_b: u32,
    num_inspections: u32,
}

impl Monkey {
    fn parse_items(line: &str) -> Vec<u32> {
        line.split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }

    fn parse_operation(line: &str) -> Operation {
        Operation::from_str(line.split_once("= ").unwrap().1)
    }

    fn parse_test(line: &str) -> u32 {
        line.split_once("by ").unwrap().1.parse::<u32>().unwrap()
    }

    fn parse_target(line: &str) -> u32 {
        line.split_once("key ").unwrap().1.parse::<u32>().unwrap()
    }

    pub fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        Self {
            items: Self::parse_items(lines[1]),
            operation: Self::parse_operation(lines[2]),
            divisible_by: Self::parse_test(lines[3]),
            monkey_a: Self::parse_target(lines[4]),
            monkey_b: Self::parse_target(lines[5]),
            num_inspections: 0,
        }
    }

    /// Returns the target monkey's index and the thrown item's worry level.
    pub fn inspect_item(&mut self) -> Option<(usize, u32)> {
        if self.items.is_empty() {
            return None;
        }
        self.num_inspections += 1;

        let mut item = self.items.remove(0);

        self.operation.apply_to(&mut item); // Monkey inspects the item

        item /= 3; // Monkey gets bored

        if item % self.divisible_by == 0 {
            Some((self.monkey_a as usize, item))
        } else {
            Some((self.monkey_b as usize, item))
        }
    }

    pub fn add_item(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn num_inspections(&self) -> u32 {
        self.num_inspections
    }
}
