use std::fs;

mod intcodecomputer;
use intcodecomputer::IntcodeComputer;

fn main() {
    let input: Vec<isize> = fs::read_to_string("input_day_9.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();

    let mut computer = IntcodeComputer::new();

    computer.load(&input);
    let outputs = computer.run(Some(vec![1]));
    println!("Outputs: {:?}", outputs);
}
