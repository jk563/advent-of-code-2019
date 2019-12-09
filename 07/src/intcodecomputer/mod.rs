mod instruction;
use instruction::Instruction;
use instruction::opcode::Opcode;

use std::collections::VecDeque;

pub struct IntcodeComputer {
    pub memory: Vec<isize>,
    position: usize,
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            memory: Vec::new(),
            position: 0,
        }
    }

    pub fn load(&mut self, program: &Vec<isize>) {
        self.memory = program.clone();
        self.position = 0;
    }

    pub fn run(&mut self, input_option: Option<Vec<isize>>) -> Vec<isize> {
        let mut instruction = self.load_instruction();
        let mut inputs: VecDeque<isize> = match input_option {
            Some(input_options) => VecDeque::from(input_options),
            None => VecDeque::new(),
        };
        let mut outputs: Vec<isize> = vec![];

        while instruction.opcode != Opcode::Halt {
            if ((instruction.opcode == Opcode::Input ) && inputs.len() == 0) {
                break;
            }
            let input = match instruction.opcode == Opcode::Input {
                true => Some(inputs.pop_front().unwrap()),
                false => None,
            };
            match instruction.execute(&mut self.memory, &mut self.position, input) {
                Some(output) => outputs.push(output),
                None => (),
            };
            instruction = self.load_instruction();
        }

        outputs
    }

    fn load_instruction(&self) -> Instruction {
        Instruction::from(&self.memory, self.position)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_initialisation() {
        let computer = IntcodeComputer::new();
        let expected_memory = Vec::new();
        assert_eq!(computer.memory, expected_memory);
        assert_eq!(computer.position, 0);
    }

    #[test]
    fn test_program_load() {
        let mut computer = IntcodeComputer::new();
        let program = vec![2, 1, 3];

        computer.load(&program);

        assert_eq!(computer.memory, program);
    }

    #[test]
    fn test_position_reset_on_load() {
        let mut computer = IntcodeComputer::new();
        let program = vec![2, 1, 3];

        computer.position = 50;
        computer.load(&program);

        assert_eq!(computer.memory, program);
        assert_eq!(computer.position, 0);
    }

    #[test]
    fn test_program_with_input() {
        let mut computer = IntcodeComputer::new();
        let program = vec![3,1,99];
        let inputs: Option<Vec<isize>> = Some(vec![7]); 

        computer.load(&program);
        computer.run(inputs);

        let expected_memory = vec![3,7,99];

        assert_eq!(computer.memory, expected_memory);
        assert_eq!(computer.position, 2);
    }

    #[test]
    fn test_exit_if_not_enough_inputs() {
        let expected_memory = vec![3,1,99];
        let mut computer = IntcodeComputer::new();
        let program = vec![3,1,99];

        computer.load(&program);
        computer.run(None);

        assert_eq!(computer.memory, expected_memory);
        assert_eq!(computer.position, 0);
    }

    #[test]
    fn test_program_with_output() {
        let mut computer = IntcodeComputer::new();
        let program = vec![4,2,99];

        computer.load(&program);
        let outputs = computer.run(None);

        let expected_memory = vec![4,2,99];
        let expected_outputs = vec![99];

        assert_eq!(computer.memory, expected_memory);
        assert_eq!(computer.position, 2);
        assert_eq!(expected_outputs, outputs);
    }

    #[test]
    fn test_basic_program_execution() {
        let mut computer = IntcodeComputer::new();
        let program = vec![1,0,0,3,99];

        computer.load(&program);
        computer.run(None);

        let expected_memory = vec![1,0,0,2,99];

        assert_eq!(computer.memory, expected_memory);
    }

    #[test]
    fn test_program_with_multiple_instructions() {
        let expected_memory: Vec<isize> = vec![30,1,1,4,2,5,6,0,99];

        let mut computer = IntcodeComputer::new();
        let program = vec![1,1,1,4,99,5,6,0,99];

        computer.load(&program);
        computer.run(None);

        assert_eq!(computer.memory, expected_memory);
    }

    #[test]
    fn test_input_0() {
        let expected_output = 0;

        let mut computer = IntcodeComputer::new();
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        computer.load(&program);
        let inputs = Some(vec![0]);
        let outputs = computer.run(inputs);

        assert_eq!(expected_output, outputs[0]);
    }

    #[test]
    fn test_day_2_part_1() {
        let mut computer = IntcodeComputer::new();

        let mut day_2_input: Vec<isize> = fs::read_to_string("input_day_2.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();

        day_2_input[1] = 12;
        day_2_input[2] = 2;

        computer.load(&day_2_input);
        computer.run(None);

        assert_eq!(5434663, computer.memory[0]);
    }

    #[test]
    fn test_day_2_part_2() {
        let mut computer = IntcodeComputer::new();

        let day_2_input: Vec<isize> = fs::read_to_string("input_day_2.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();

        let mut noun_variation = 0;
        let mut verb_variation = 0;

        'noun: while noun_variation < 100 {
            'verb: while verb_variation < 100 {
                let mut program_variation = day_2_input.clone();

                program_variation[1] = noun_variation;
                program_variation[2] = verb_variation;
            
                computer.load(&program_variation);
                computer.run(None);

                if computer.memory[0] == 19690720 {
                    break 'noun;
                }

                verb_variation = verb_variation + 1;
            }

            noun_variation = noun_variation + 1;
            verb_variation = 0
        }

        assert_eq!(noun_variation, 45);
        assert_eq!(verb_variation, 59);
    }

    #[test]
    fn test_day_5_part_1() {
        let mut computer = IntcodeComputer::new();
        let expected_final_output: isize = 6745903;
        let day_5_input: Vec<isize> = fs::read_to_string("input_day_5.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();
        let inputs: Option<Vec<isize>> = Some(vec![1]);

        computer.load(&day_5_input);
        let outputs = computer.run(inputs);

        assert_eq!(expected_final_output, *outputs.last().unwrap());
    }

    #[test]
    fn test_day_5_part_2() {
        let mut computer = IntcodeComputer::new();
        let expected_final_output: isize = 9168267;
        let day_5_input: Vec<isize> = fs::read_to_string("input_day_5.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();
        let inputs: Option<Vec<isize>> = Some(vec![5]);

        computer.load(&day_5_input);
        let outputs = computer.run(inputs);

        assert_eq!(expected_final_output, *outputs.last().unwrap());
    }
}
