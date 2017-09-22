mod cpu;
mod ga;

use cpu::Cpu;
use ga::GA;
use cpu::Instruction;
use std::vec::Vec;

fn test_cpu() {
    let mut cpu = Cpu::new();

    cpu.init_params(vec![1,2]);
    cpu.init_vars(3);
    cpu.add(5,0,0);
    cpu.add(5,0,0);
    cpu.add(5,0,0);
    cpu.randomize(1000000);
    cpu.run();
    //cpu.print();

    println!("ret: {}", cpu.result());

}

fn main() {
    let mut ga = GA::new();
    ga.init_population(100, 10);

    let mut cpu1 = Cpu::new();
    cpu1.init_vars(3);
    let mut cpu2 = cpu1.clone();
    cpu1.randomize(4);
    cpu2.randomize(4);

    let (cpu3, cpu4) = cpu1.crossover(&cpu2);

    cpu1.print();
    println!("---");
    cpu2.print();
    println!("---");
    cpu3.print();
    println!("---");
    cpu4.print();
    println!("---");
}


