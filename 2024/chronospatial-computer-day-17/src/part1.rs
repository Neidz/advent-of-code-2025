enum Instruction {
    Adv,
    Blx,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
}

impl Computer {
    pub fn execute_program(&mut self, program: Vec<usize>) {
        program.chunks(2).for_each(|chunk| {
            let instruction_opcode = chunk[0];
            let operand = chunk[1];

            let instruction = match instruction_opcode {
                0 => Instruction::Adv,
                1 => Instruction::Blx,
                2 => Instruction::Bst,
                3 => Instruction::Jnz,
                4 => Instruction::Bxc,
                5 => Instruction::Out,
                6 => Instruction::Bdv,
                7 => Instruction::Cdv,
                _ => panic!("Invalid instruction opcode"),
            };

            let value = match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                _ => panic!("Invalid operand"),
            };

            match instruction {
                Instruction::Adv => self.adv(value),
                _ => {}
            }
        });
    }

    fn adv(&mut self, value: usize) {
        self.register_a = self.register_a / (2usize.pow(value as u32));
    }

    fn bxl(&mut self, value: usize) {
        self.register_b = self.register_b ^ value;
    }

    fn bst(&mut self, value: usize) {}

    fn jnz(&mut self, value: usize) {}

    fn bxc(&mut self, value: usize) {}

    fn out(&mut self, value: usize) {}

    fn bdv(&mut self, value: usize) {
        self.register_b = self.register_a / (2 ^ value);
    }

    fn cdv(&mut self, value: usize) {
        self.register_c = self.register_a / (2 ^ value);
    }
}

pub fn part1() {
    let mut computer = Computer {
        register_a: 729,
        register_b: 0,
        register_c: 0,
    };
    computer.execute_program(vec![0, 1, 5, 4, 3, 0]);
}
