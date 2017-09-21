mod cpu;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();



    cpu.params(vec![1,2]);
    cpu.vars(3);
    cpu.add(5,0,0);
    cpu.add(5,0,0);
    cpu.add(5,0,0);
    cpu.run();

    println!("ret: {}", cpu.result());

}
