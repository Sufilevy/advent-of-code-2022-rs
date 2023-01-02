pub struct P1Cpu {
    cycle: i32,
    x: i64,
    sum: i64,
}

impl P1Cpu {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            x: 1,
            sum: 0,
        }
    }

    fn add_signal_strength(&mut self) {
        if (self.cycle - 20) % 40 == 0 {
            self.sum += self.cycle as i64 * self.x;
        }
    }

    pub fn execute_instruction(&mut self, instruction: &str) {
        if instruction.starts_with("noop") {
            self.cycle += 1;
            self.add_signal_strength();
        } else {
            self.cycle += 1;
            self.add_signal_strength();
            self.cycle += 1;
            self.add_signal_strength();

            let parts = instruction.split_whitespace().collect::<Vec<_>>();
            self.x += parts[1].parse::<i32>().unwrap() as i64;
        }
    }

    pub fn get_sum(&self) -> i64 {
        self.sum
    }
}

pub struct P2Cpu {
    cycle: i32,
    x: i64,
}

impl P2Cpu {
    pub fn new() -> Self {
        Self { cycle: 0, x: 1 }
    }

    fn draw_pixel(&mut self) {
        let cursor_x = (self.cycle as i64 - 1) % 40;
        if cursor_x == self.x - 1 || cursor_x == self.x || cursor_x == self.x + 1 {
            print!("#");
        } else {
            print!(".")
        }
        if self.cycle % 40 == 0 {
            println!();
        }
    }

    pub fn execute_instruction(&mut self, instruction: &str) {
        if instruction.starts_with("noop") {
            self.cycle += 1;
            self.draw_pixel();
        } else {
            self.cycle += 1;
            self.draw_pixel();

            self.cycle += 1;
            self.draw_pixel();

            let parts = instruction.split_whitespace().collect::<Vec<_>>();
            self.x += parts[1].parse::<i32>().unwrap() as i64;
        }
    }
}
