use std::fs;

use intcodecomputer::IntcodeComputer;

fn main() {
    let input: Vec<isize> = fs::read_to_string("input.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();

    let mut computer = IntcodeComputer::new();

    computer.load(&input);
    let outputs = computer.run(Some(vec![1]));
    println!("Boost Keycode: {:?}", outputs);

    computer.load(&input);
    let outputs = computer.run(Some(vec![2]));
    println!("Distress Signal: {:?}", outputs);
}
