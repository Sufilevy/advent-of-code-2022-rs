#[derive(Debug)]
pub struct Supplies {
    stacks: Vec<Vec<char>>,
}

impl Supplies {
    pub fn from(input: &str) -> Self {
        let mut lines = input.lines().collect::<Vec<_>>();
        let num_stacks = lines.pop().unwrap().split_whitespace().count();
        let mut stacks = vec![Vec::new(); num_stacks];

        lines.iter().rev().for_each(|line| {
            format!("{} ", line)
                .chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .enumerate()
                .for_each(|(index, e)| match e {
                    [' ', ' ', ' ', ' '] => {}
                    ['[', c, ']', ' '] => stacks[index].push(*c),
                    _ => {}
                })
        });

        Self { stacks }
    }

    pub fn execute_instructions_9000(&mut self, instructions: &str) {
        instructions.lines().for_each(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let ["move", amount, "from", from, "to", to] = parts[..] else { panic!("Well") };

            let (amount, from, to) = (
                amount.parse::<usize>().unwrap(),
                from.parse::<usize>().unwrap() - 1,
                to.parse::<usize>().unwrap() - 1,
            );

            for _ in 0..amount {
                let e = self.stacks[from].pop().unwrap();
                self.stacks[to].push(e);
            }
        });
    }

    pub fn execute_instructions_9001(&mut self, instructions: &str) {
        instructions.lines().for_each(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let ["move", amount, "from", from, "to", to] = parts[..] else { panic!("Huh...") };

            let (amount, from, to) = (
                amount.parse::<usize>().unwrap(),
                from.parse::<usize>().unwrap() - 1,
                to.parse::<usize>().unwrap() - 1,
            );

            let crates = self.stacks[from]
                .iter()
                .rev()
                .take(amount)
                .copied()
                .rev()
                .collect::<Vec<_>>();
            self.stacks[to].extend(crates);

            let len = self.stacks[from].len();
            self.stacks[from].truncate(len - amount);
        });
    }

    pub fn get_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }
}
