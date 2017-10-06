/*
    Genetic Algorithm to evolve code.

*/

use std::vec::Vec;
use cpu::Cpu;

pub struct GA {
    population: Vec<Cpu>,
    mutation_probability: u16,
    sz: usize
}

impl GA {
    pub fn new() -> GA {
        GA {
            population: Vec::new(),
            mutation_probability: 2,
            sz: 0
        }
    }

    pub fn init_population(&mut self, population_sz: usize, num_instructions: usize) {
        self.sz = population_sz;
        for _ in 0..population_sz {
            let mut cpu = Cpu::new();
            cpu.randomize(num_instructions);
            self.population.push(cpu);
        }
    }

    pub fn sort(&mut self) {
        let tmp: Cpu;

    }

    pub fn run(&mut self, num_cycles: usize, evaluate: fn(&mut Cpu)) {
        //let mut cpu = self.population[0].crossover(self.population[1]);

        for cycle in 1..num_cycles+1 {

            // evaluate
            for i in 0..self.sz {
                let cpu: &mut Cpu = &mut self.population[i];
                evaluate(cpu);
            }

            // clasify
            self.sort();

            println!("Cycle {}.", cycle) // score

            // crossover

            // mutation

        }
    }


}

