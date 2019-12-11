use std::fs;

use intcodecomputer::IntcodeComputer;

fn main() {
    let input: Vec<isize> = fs::read_to_string("input.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();
    
    let mut computer = IntcodeComputer::new();

    let mut part1_input = input.clone();
    part1_input[1] = 12;
    part1_input[2] = 2;
    computer.load(&part1_input);
    computer.run(None);
    println!("Part 1: {}", computer.memory[0]);
    println!();

    println!("Part 2:");

    let mut noun = 0;
    let mut verb = 0;

    'noun: for n in 0..100 { 
        for v in 0..100 {
            let mut program = input.clone();
            program[1] = n;
            program[2] = v;
            computer.load(&program);
            computer.run(None);
            if computer.memory[0] == 19690720 {
                noun = n;
                verb = v;
                break 'noun;  
            };
        }
    }

    println!("{}{}", noun, verb);
}
