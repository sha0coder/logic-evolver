use std::vec::Vec;

pub struct Instruction {
    op: u8,
    a: usize,
    b: usize
}

impl Instruction {
    pub fn new(opcode:u8, operandA:usize, operandB:usize) -> Instruction {
        Instruction {
            op: opcode,
            a: operandA,
            b: operandB
        }
    }
}


pub struct Cpu {
    params: Vec<i32>,
    vars: Vec<i32>,
    code: Vec<Instruction>
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            params: Vec::new(),
            vars: Vec::new(),
            code: Vec::new(),
        }
    }

    pub fn params(&mut self, p: Vec<i32>) {
        self.params = p;
    }

    pub fn vars(&mut self, num: u32) {
        let mut i: u32 = 0;
        loop {
            self.vars.push(0);
            i += 1;
            if i>num {
                break;
            }
        }
    }

    pub fn result(&self) -> i32 {

        return self.vars[0];
    }

    pub fn add(&mut self, op:u8, a:usize, b:usize) {
        self.code.push(Instruction::new(op,a,b));
    }

    pub fn load(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    pub fn run(&mut self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        loop {

            if line >= len {
                break;
            }

            let ins:&Instruction = &self.code[line];

            println!("ins:{}", ins.op);

            match ins.op {
                0 => { self.vars[ins.a] = self.vars[ins.b] }

                1 => { self.vars[ins.a] += self.vars[ins.b] }
                2 => { self.vars[ins.a] -= self.vars[ins.b] }
                3 => { self.vars[ins.a] *= self.vars[ins.b] }
                4 => { if self.vars[ins.b] != 0 {self.vars[ins.a] /= self.vars[ins.b] }} 

                5 => { self.vars[ins.a] += 1 }
                6 => { self.vars[ins.a] -= 1 }
                7 => { self.vars[ins.a] = 0 }

                8 => { if self.vars[ins.a] == self.vars[ins.b] { line+=1 } }
                9 => { if self.vars[ins.a] != self.vars[ins.b] { line+=1 } }
                10 => { if self.vars[ins.a] < self.vars[ins.b] { line+=1 } }
                11 => { if self.vars[ins.a] <= self.vars[ins.b] { line+=1 } }
                12 => { if self.vars[ins.a] > self.vars[ins.b] { line+=1 } }
                13 => { if self.vars[ins.a] >= self.vars[ins.b] { line+=1 } }

                _ => {;}
            }

            line += 1;
       }
    }

}