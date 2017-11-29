/*
    Genetic Algorithm to evolve code.

*/

use std::collections::HashMap;
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

    pub fn sort(&mut self) -> Vec<usize>  {
        //let sorted_population : Vec<Cpu> = Vec::new();
        let i: usize;
        let j: usize;
        let mut tmp: usize;
        let mut sorted: Vec<usize> = Vec::new(); //HashMap<i32,i32> = HashMap::new();
        
        for i in 0..self.sz {
            sorted.push(i)
        }

        for i in 0..self.sz {
            for j in 1..self.sz-1 {
                if self.population[sorted[j]].get_fitness() > self.population[sorted[i]].get_fitness() {
                    tmp = sorted[i];
                    sorted[j] = sorted[i];
                    sorted[i] = tmp;
                }
            }
        }

        return sorted;
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
            let mut sorted = self.sort();
            println!("Cycle: {} max fitness: {}", cycle, self.population[sorted[0]].get_fitness())

            // crossover
            for i in 1..10 {
                
            }

            // mutation

        }
    }


}

