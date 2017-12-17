mod cpu;
mod ga;


use std::vec::Vec;
use cpu::Cpu;
use cpu::Instruction;
use ga::GA;


/*
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

fn test_cpu2() {
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
}*/

fn eval(cpu: &mut Cpu) {
    let mut score: i32 = 0;
    let mut tv: Vec<u8> = Vec::new();


    //cpu.print();
    //cpu.debug();

    // test1
    cpu.init_vars(vec![2,1,2]);
    cpu.run();
    if (cpu.result() == 3) {
        score+=1;
        tv.push(1);
    }

    // test2
    cpu.init_vars(vec![2,2,2]);
    cpu.run();
    if (cpu.result() == 4) {
        score+=1;
        tv.push(2);
    }

    // test3
    cpu.init_vars(vec![3,1,3]);
    cpu.run();
    if (cpu.result() == 3) {
        score+=1;
        tv.push(3);
    }

    // test4
    cpu.init_vars(vec![3,3,3]);
    cpu.run();
    if (cpu.result() == 6) {
        score+=1;
        tv.push(4);
    }

    // test5
    cpu.init_vars(vec![4,4,2]);
    cpu.run();
    if (cpu.result() == 8) {
        score+=1;
        tv.push(5);
    }
    
    cpu.set_fitness(score, tv);
}

fn main() {

    // 001 v0 = v1
    // 102 v0 += v2
    /*
    let mut i1 = Instruction::new(0,0,1);
    let mut i2 = Instruction::new(1,0,2);
    let mut cpu = Cpu::new();
    cpu.add_instruction(i1);
    cpu.add_instruction(i2);
    eval(&mut cpu);
    println!(">>{}",cpu.get_fitness());
*/

    

    let mut ga = GA::new();
    ga.init_population(1000, 5);
    ga.run(500, eval);
  

}


