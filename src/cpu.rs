/*
    Basic & fast cpu emulator
    1.8seconds -> execute 1.000.000 instructions
    
    @sha0coder
*/


extern crate rand;

use std::vec::Vec;
use cpu::rand::Rng;


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

    pub fn clone(&self) -> Instruction {
        Instruction {
            op: self.op,
            a: self.a,
            b: self.b
        }
    }

    pub fn set_a(&mut self, a: usize) {
        self.a = a;
    }

    pub fn set_b(&mut self, b: usize) {
        self.b = b;
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.op = opcode;
    }

    pub fn get_a(&self) -> usize {
        return self.a;
    }

    pub fn get_b(&self) -> usize {
        return self.b;
    }

    pub fn get_opcode(&self) -> u8 {
        return self.op;
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

    pub fn init_params(&mut self, p: Vec<i32>) {
        self.params = p;
    }

    pub fn init_vars(&mut self, num: usize) {
        for _ in 0..num {
            self.vars.push(0);
        }
    }

    pub fn result(&self) -> i32 {
        return self.vars[0];
    }

    pub fn add(&mut self, op:u8, a:usize, b:usize) {
        // add code instruction
        self.code.push(Instruction::new(op,a,b));
    }

    pub fn load(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    pub fn randomize(&mut self, n:usize) {
        // generate a random code of n lines
        let mut rng = rand::thread_rng();
        println!("creating");
        let mut i:usize = 0;
        loop {
            if i >= n {
                break;
            }
            self.add(rng.gen_range::<u8>(0,14), rng.gen_range::<usize>(0,3), rng.gen_range::<usize>(0,3));
            i+=1;
        }
        println!("created");
    }

    pub fn init(&mut self) {
        for i in 0..self.vars.len() {
            self.vars[i] = 0;
        }
    }

    pub fn run(&mut self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        self.init();

        loop {

            if line >= len {
                break;
            }

            let ins:&Instruction = &self.code[line];

            //println!("ins:{}", ins.op);

            match ins.op {
                0 => { self.vars[ins.a] = self.vars[ins.b] }

                1 => { self.vars[ins.a] += self.vars[ins.b] }
                2 => { self.vars[ins.a] -= self.vars[ins.b] }
                3 => { if self.vars[ins.a]<99 && self.vars[ins.b]<99 && self.vars[ins.a]>-99 && self.vars[ins.b]>-99 { self.vars[ins.a] *= self.vars[ins.b] } }
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

    pub fn print(&self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        loop {

            if line >= len {
                break;
            }

            let ins:&Instruction = &self.code[line];

            match ins.op {
                0 => { println!("{}: v{} = v{}", line, ins.a, ins.b) }

                1 => { println!("{}: v{} += v{}", line, ins.a, ins.b) }
                2 => { println!("{}: v{} -= v{}", line, ins.a, ins.b) }
                3 => { println!("{}: v{} *= v{} if not overflow", line, ins.a, ins.b) }
                4 => { println!("{}: v{} /= v{} if divisible", line, ins.a, ins.b)} 

                5 => { println!("{}: v{} ++", line, ins.a) }
                6 => { println!("{}: v{} --", line, ins.a) }
                7 => { println!("{}: v{} = 0", line, ins.a) }

                8 => { println!("{}: if v{} != v{}", line, ins.a, ins.b) } //if self.vars[ins.a] == self.vars[ins.b] { line+=1 } }
                9 => { println!("{}: if v{} == v{}", line, ins.a, ins.b) } //if self.vars[ins.a] != self.vars[ins.b] { line+=1 } }
                10 => { println!("{}: if v{} >= v{}", line, ins.a, ins.b) } //if self.vars[ins.a] < self.vars[ins.b] { line+=1 } }
                11 => { println!("{}: if v{} > v{}", line, ins.a, ins.b) } //if self.vars[ins.a] <= self.vars[ins.b] { line+=1 } }
                12 => { println!("{}: if v{} <= v{}", line, ins.a, ins.b) }   //if self.vars[ins.a] > self.vars[ins.b] { line+=1 } }
                13 => { println!("{}: if v{} < v{}", line, ins.a, ins.b) } //if self.vars[ins.a] >= self.vars[ins.b] { line+=1 } }

                _ => { println!("{}: nop", line) }
            }

            line += 1;
       }
    }

       
    pub fn clone(&self) -> Cpu {
        let mut new_cpu: Cpu;

        new_cpu = Cpu::new();
    
        new_cpu.init_vars(self.vars.len());
        for instr in self.code.iter() {
            new_cpu.code.push(instr.clone());
        }

        return new_cpu;
    }

    pub fn crossover(&self, cpu_mother: Cpu) -> (Cpu,Cpu) {
        let mut child1 = Cpu::new();
        let mut child2 = Cpu::new();
        let mut half_father = self.code.len()/2;
        let mut half_mother = cpu_mother.code.len()/2;

        // optimize this:
        for i in 0..half_father {
            child1.code.push(self.code[i].clone());
        }
        for i in 0..half_mother {
            child2.code.push(cpu_mother.code[i].clone());
        }
        for i in half_father..self.code.len() {
            child2.code.push(self.code[i].clone());
        }
        for i in half_mother..self.code.len() {
            child1.code.push(cpu_mother.code[i].clone());
        }

        child1.init_vars(self.vars.len());
        child2.init_vars(self.vars.len());

        return (child1,child2);
    }

}


