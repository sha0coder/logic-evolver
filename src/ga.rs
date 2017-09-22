// Genetic Algorithm 

use std::vec::Vec;
use cpu::Cpu;

pub struct GA {
    population: Vec<Cpu>,
    mutation_probability: u16
}

impl GA {
    pub fn new() -> GA {
        GA {
            population: Vec::new(),
            mutation_probability: 2
        }
    }

    pub fn init_population(&mut self, population_sz: usize, num_instructions: usize) {
        for _ in 0..population_sz {
            let mut cpu = Cpu::new();
            cpu.randomize(num_instructions);
            self.population.push(cpu);
        }
    }

    pub fn run(&mut self, num_cycles: usize) {
        //let mut cpu = self.population[0].crossover(self.population[1]);
    }


}

