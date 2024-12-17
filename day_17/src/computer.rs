#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Computer {
    instruction_pointer: i32,
    pub a_register: u32,
    pub b_register: u32,
    pub c_register: u32,
    program: Vec<u32>,
}

impl Computer {
    pub fn new(a_register: u32, b_register: u32, c_register: u32, program: Vec<u32>) -> Self {
        Computer {
            instruction_pointer: 0,
            a_register,
            b_register,
            c_register,
            program,
        }
    }

    pub fn run(&mut self) -> Vec<u32> {
        let mut result = vec![];
        while self.instruction_pointer as usize + 1 < self.program.len() {
            let pointer = self.instruction_pointer as usize;
            let instruction = self.program[pointer];
            let operand = self.program[pointer + 1];
            if let Some(output) = self.run_instruction(instruction, operand) {
                result.push(output);
            }
            self.instruction_pointer += 2;
        }
        result
    }

    fn combo_operand(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4 => self.a_register,
            5 => self.b_register,
            6 => self.c_register,
            7 => panic!("Combo operand 7 is reserved and will not appear in valid programs."),
            _ => panic!("Combo operand ({operand}) must be in range 0..=7."),
        }
    }

    fn run_instruction(&mut self, instruction: u32, operand: u32) -> Option<u32> {
        match instruction {
            0 => self.a_register = self.a_register / 2_u32.pow(self.combo_operand(operand)),
            1 => self.b_register ^= operand,
            2 => self.b_register = self.combo_operand(operand) % 8,
            3 => {
                if self.a_register != 0 {
                    self.instruction_pointer = operand as i32 - 2;
                }
            }
            4 => self.b_register ^= self.c_register,
            5 => {
                return Some(self.combo_operand(operand) % 8);
            }
            6 => self.b_register = self.a_register / 2_u32.pow(self.combo_operand(operand)),
            7 => self.c_register = self.a_register / 2_u32.pow(self.combo_operand(operand)),
            _ => panic!("Instruction ({instruction}) must be in range 0..=7."),
        }
        None
    }
}
