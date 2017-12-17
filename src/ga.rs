/*
    Genetic Algorithm to evolve code.

*/

use std::collections::HashMap;
use std::process::exit;
use std::vec::Vec;
use cpu::Cpu;
use cpu::Instruction;

pub struct GA {
    population: Vec<Cpu>,
    mutation_probability: u16,
    sz: usize,
    num_instructions: usize
}

impl GA {
    pub fn new() -> GA {
        GA {
            population: Vec::new(),
            mutation_probability: 2,
            sz: 0,
            num_instructions: 10
        }
    }

    pub fn init_population(&mut self, population_sz: usize, num_instructions: usize) {
        self.sz = population_sz;
        for _ in 0..population_sz {
            let mut cpu = Cpu::new();
            cpu.randomize(num_instructions);
            self.num_instructions = num_instructions;
            self.population.push(cpu);
        }
    }

    pub fn sort(&mut self) -> Vec<usize>  {
        //let sorted_population : Vec<Cpu> = Vec::new();
        let i: usize;
        let j: usize;
        let mut tmp: usize;
        let mut sorted: Vec<usize> = Vec::new(); //HashMap<i32,i32> = HashMap::new();
        
        for i in 0..self.population.len() {
            sorted.push(i);
        }

        for i in 0..self.population.len()-1 {
            for j in 1..self.population.len() {
                if self.population[sorted[j]].get_fitness() > self.population[sorted[i]].get_fitness() {
                    tmp = sorted[j];
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
            for i in 0..self.population.len() {
                let cpu: &mut Cpu = &mut self.population[i];
                evaluate(cpu);
            }

            // clasify
            let mut sorted = self.sort();
            println!("\n** Cycle: {} pop: {} max fitness: {} tests passed: {}/5", cycle, self.population.len(), self.population[sorted[0]].get_fitness(), self.population[sorted[0]].get_tv().len());
            self.population[sorted[0]].print();


            let mut ng: Vec<Cpu> = Vec::new();

            // crosoover top 50, looking for the best individual to pair
            for i in 0..49 {
                let father = &self.population[sorted[i]];
                let mut mother: Option<&Cpu> = None;
                let ftv = father.get_tv();

                if (ftv.len() == 5) {
                    println!("5 TESTS PASSED:");
                    father.print();
                    exit(1);
                }

                let mut falta: Vec<u8> = Vec::new();
                let k: u8 = 0;
                
                for k in 0..5 {
                    if (!ftv.contains(&k)) {
                        falta.push(k);
                    }
                }

                if (falta.len()<5) {
                    println!("--- falta: ---");
                    for k in 0..falta.len() {
                        println!("   {}", falta[k]);
                    }
                    println!("--------");
                }
            


                for j in 1..50 {
                    let mut pmother = &self.population[sorted[j]];
                    let tv = pmother.get_tv();

                    if (falta.len() == 1 && tv.contains(&falta[0])) {
                        mother = Some(pmother);
                        println!("ENTRA!!!!!!!!!!!!!1");
                        break;
                    }

                    if (falta.len() == 2 && tv.contains(&falta[0]) && tv.contains(&falta[1])) {
                        mother = Some(pmother);
                        println!("ENTRA!!!!!!!!!!!!!1");
                        break;
                    }

                    if (falta.len() == 2 && (tv.contains(&falta[0]) || tv.contains(&falta[1]))) {
                        mother = Some(pmother);
                        println!("ENTRA!!!!!!!!!!!!!1");
                    }

                    if (falta.len() == 3 && tv.contains(&falta[0]) && tv.contains(&falta[1]) && tv.contains(&falta[2])) {
                        mother = Some(pmother);
                        println!("ENTRA!!!!!!!!!!!!!1");
                        break;
                    }

                    if (falta.len() == 3 && (tv.contains(&falta[0]) || tv.contains(&falta[1]) || tv.contains(&falta[2]))) {
                        mother = Some(pmother);
                        println!("ENTRA!!!!!!!!!!!!!1");
                        break;
                    }

                    if (falta.len() == 4) {
                        mother = Some(pmother);
                        println!("ENTRA!!!!!!!!!!!!!1");
                        break;
                    }
                }

                //if (*mother) {
                if let Some(mum) = mother {
                    println!("ENTRA!!!!!!!!!!!!!2");
                    let childs = father.crossover(mum);
                    ng.push(childs.0);
                    ng.push(childs.1);

                    let mut vchilds = father.crossover2(mum);
                    for i in 0..vchilds.len() {
                        ng.push(vchilds[i].clone());
                    }
                    vchilds = father.crossover2(mum);
                    for i in 0..vchilds.len() {
                        ng.push(vchilds[i].clone());
                    }
                    vchilds = father.crossover2(mum);
                    for i in 0..vchilds.len() {
                        ng.push(vchilds[i].clone());
                    }

                } else {
                    let r = self.population[0].get_rand(50);
                    let mum = &self.population[sorted[r]];
                    let childs = father.crossover(mum);
                    ng.push(childs.0);
                    ng.push(childs.1);
                    
                    let mut vchilds = father.crossover2(mum);
                    for i in 0..vchilds.len() {
                        ng.push(vchilds[i].clone());
                    }

                    let mut vchilds = father.crossover2(mum);
                    for i in 0..vchilds.len() {
                        ng.push(vchilds[i].clone());
                    }
                }
            }



            // trascend top 10
            for i in 0..40 {
                ng.push(self.population[sorted[i]].clone());
            }

            // diversity
            for _ in 0..100 {
                let r = self.population[0].get_rand(self.population.len());
                ng.push(self.population[r].clone());
            }

            // mutation
            for i in 0..ng.len() {
                ng[i].mutate(25);
            }

            // 100 news
            let mut cpu;
            for _ in 0..1000 {
                cpu = Cpu::new();
                cpu.randomize(self.num_instructions);
                ng.push(cpu);
            }


            // 001 v0 = v1
            // 102 v0 += v2
            /*
            let mut i1 = Instruction::new(0,0,1);
            let mut i2 = Instruction::new(1,0,2);
            cpu = Cpu::new();
            cpu.add_instruction(i1);
            cpu.add_instruction(i2);
            ng.push(cpu);*/


            self.population = ng;
        }
    }
}

