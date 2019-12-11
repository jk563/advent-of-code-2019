use std::fs;

use intcodecomputer::IntcodeComputer;

fn main() {
    let mut computer = IntcodeComputer::new();
    let day_5_input: Vec<isize> = fs::read_to_string("input.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();
    let inputs: Option<Vec<isize>> = Some(vec![1]);

    computer.load(&day_5_input);
    let outputs = computer.run(inputs);
    println!("Part 1: {}", outputs.last().unwrap());

    let inputs: Option<Vec<isize>> = Some(vec![5]);
    computer.load(&day_5_input);
    let outputs = computer.run(inputs);
    println!("Part 2: {}", outputs.last().unwrap());
}
