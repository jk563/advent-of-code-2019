use std::fs;

use intcodecomputer::IntcodeComputer;

fn main() {
    let input: Vec<isize> = fs::read_to_string("input.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();

    let input: Vec<isize> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let mut computer = IntcodeComputer::new();

    computer.load(&input);
    let outputs = computer.run(Some(vec![1]));
    println!("Outputs: {:?}", outputs);
}
