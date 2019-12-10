pub mod opcode;
pub mod parametermode;
use opcode::Opcode;
use parametermode::ParameterMode;

use std::convert::TryFrom;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    parameters: Option<Vec<isize>>,
    parameter_modes: Vec<ParameterMode>,
}

impl Instruction {
    pub fn from(mut memory: &Vec<isize>, position: usize, relative_base: isize) -> Instruction {
        let instruction_code = usize::try_from(memory[position]).unwrap();
        let mut parameter_modes: Vec<ParameterMode> = Vec::new();

        let opcode = Opcode::from(instruction_code % 100);

        for x in 1..=(opcode.parameter_count()) {
            // Get the digit corrosponding to the parameter mode
            let mode_integer = (instruction_code / 100) / 1.max(10 * (x - 1)) % 10;
            println!("MI: {:?}", mode_integer);
            if x == opcode.parameter_count() {
                parameter_modes.push( match opcode {
                       Opcode::Input | Opcode::Output | Opcode::JumpTrue | Opcode::JumpFalse => ParameterMode::from(mode_integer),
                        _ => ParameterMode::ImmediateMode,
                    }
                );
            } else {
                parameter_modes.push(ParameterMode::from(mode_integer));
            }
        }
            println!("MI: {:?}", parameter_modes);
        
        let mut instruction = Instruction {
            opcode: opcode,
            parameters: None,
            parameter_modes: parameter_modes,
        };
        instruction.load_parameters(&mut memory.clone(), position, relative_base);
        instruction
    }

    fn load_parameters(&mut self, mut memory: &mut Vec<isize>, position: usize, relative_base: isize) {
        let param_count = self.opcode.parameter_count();
        let mut parameters: Vec<isize> = Vec::new();

        for param_index in 1..=param_count {
            let param = self.read_from(&mut memory, position + param_index);

            parameters.push(match self.parameter_modes[param_index - 1] {
                    ParameterMode::PositionalMode => self.read_from(&mut memory, usize::try_from(param).unwrap()),
                    ParameterMode::RelativeMode => self.read_from(&mut memory, usize::try_from(param + relative_base).unwrap()),
                    ParameterMode::ImmediateMode => param,
            });
        };

        self.parameters = Some(parameters);
        println!("{:?}", self);
    }

    fn read_from(&self, memory: &mut Vec<isize>, position: usize) -> isize {
        if memory.len() <= position {
                return 0;
        }
        memory[position]
    }

    fn write_to(&self, memory: &mut Vec<isize>, position: usize, input: isize) {
        if memory.len() <= position {
                memory.resize(position + 1, 0);
        }
        memory[position] = input;
    }

    pub fn execute(&self, mut memory: &mut Vec<isize>, position: &mut usize, relative_base: &mut isize, input: Option<isize>) -> Option<isize> {
        let parameters = self.parameters.clone().unwrap();
        let mut output: Option<isize> = None;
        match self.opcode {
            Opcode::Add => {
                let result = parameters[0] + parameters[1];
                let write_location = usize::try_from(parameters[2]).unwrap();
                self.write_to(&mut memory, write_location, result);
                *position += 4; 
            },
            Opcode::Multiply => {
                let result = parameters[0] * parameters[1];
                let write_location = usize::try_from(parameters[2]).unwrap();
                self.write_to(&mut memory, write_location, result);

                *position += 4; 
            },
            Opcode::Input => {
                let write_location = usize::try_from(parameters[0]).unwrap();
                self.write_to(&mut memory, write_location, input.unwrap());
                *position += 2;
            },
            Opcode::Output => {
                output = Some(parameters[0]);
                *position += 2;
            },
            Opcode::JumpTrue => {
                *position = match parameters[0] != 0 {
                    true => usize::try_from(parameters[1]).unwrap(),
                    false => *position + 3, 
                };
            },
            Opcode::JumpFalse => {
                *position = match parameters[0] == 0 {
                    true => usize::try_from(parameters[1]).unwrap(),
                    false => *position + 3, 
                };
            },
            Opcode::Less => {
                let write_location = usize::try_from(parameters[2]).unwrap();

                self.write_to(&mut memory, write_location, match parameters[0] < parameters[1] {
                    true => 1,
                    false => 0,
                });
                *position += 4;
            },
            Opcode::Equal => {
                let write_location = usize::try_from(parameters[2]).unwrap();

                self.write_to(&mut memory, write_location, match parameters[0] == parameters[1] {
                    true => 1,
                    false => 0,
                });
                *position += 4;
            },
            Opcode::RelativeBase => {
                *relative_base += parameters[0];
                *position +=2;
            },
            _ => panic!("Execute not implemented for opcode: {:?}", self.opcode),
        };
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let expected_opcode: Opcode = Opcode::Add;
        let expected_parameters: Vec<isize> = vec![1,1,3];

        let memory  = vec![101,1,1,3,99];
        let instruction = Instruction::from(&memory, 0, 0);

        assert_eq!(instruction.opcode, expected_opcode);
        assert_eq!(instruction.parameters.unwrap(), expected_parameters);
        assert_eq!(instruction.parameter_modes, 
                vec![
                    ParameterMode::ImmediateMode,
                    ParameterMode::PositionalMode, 
                    ParameterMode::ImmediateMode
                ]
        );
    }

    #[test]
    fn test_from_halt_only() {
        let expected_opcode: Opcode = Opcode::Halt;
        let expected_parameters: Vec<isize> = vec![];

        let memory  = vec![99,0,0,3,99];
        let instruction = Instruction::from(&memory, 0, 0);

        assert_eq!(instruction.opcode, expected_opcode);
        assert_eq!(instruction.parameters.unwrap(), expected_parameters);
        assert_eq!(instruction.parameter_modes, vec![]);
    }

    #[test]
    fn test_execute_add() {
        let expected_memory = vec![1,0,0,2,99];
        let expected_position = 4;

        let mut memory = vec![1,0,0,3,99];
        let instruction = Instruction::from(&memory, 0, 0);
        let mut position = 0;
        let mut relative_base = 0;
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_multiply() {
        let expected_memory = vec![2,0,3,6,99];
        let expected_position = 4;

        let mut memory = vec![2,0,3,3,99];
        let instruction = Instruction::from(&memory, 0, 0);
        let mut position = 0;
        let mut relative_base = 0;
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_input() {
        let expected_memory = vec![3,5,99];
        let expected_position = 2;

        let mut memory = vec![3,1,99];
        let instruction = Instruction::from(&memory, 0, 0);
        let mut position = 0;
        let mut relative_base = 0;
        let input: isize = 5;
        instruction.execute(&mut memory, &mut position, &mut relative_base, Some(input));

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_output() {
        let expected_memory = vec![4,2,99];
        let expected_position = 2;
        let expected_output = Some(99);

        let mut memory = vec![4,2,99];
        let mut position = 0;
        let mut relative_base = 0;
        let instruction = Instruction::from(&memory, position, relative_base);
        let output = instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_execute_jumptrue() {
        let expected_first_position = 3;
        let expected_second_position = 0;

        let mut memory = vec![105,0,0,105,99,2];
        let mut position = 0;
        let mut relative_base = 0;

        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);
        assert_eq!(position, expected_first_position);

        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);
        assert_eq!(position, expected_second_position);
    }

    #[test]
    fn test_execute_jumpfalse() {
        let expected_first_position = 3;
        let expected_second_position = 5;

        let mut memory = vec![1106,5,99,106,0,1];
        let mut position = 0;
        let mut relative_base = 0;

        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);
        assert_eq!(position, expected_first_position);

        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);
        assert_eq!(position, expected_second_position);
    }

    #[test]
    fn test_execute_less() {
        let expected_memory = vec![0,0,1,0,1007,0,1,2,99];
        let expected_position = 8;

        let mut memory = vec![1007,0,99,0,1007,0,1,2,99];
        let mut position = 0;
        let mut relative_base = 0;
        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_equal() {
        let expected_memory = vec![1,0,99,0,08,0,1,1,99];
        let expected_position = 8;

        let mut memory = vec![1108,99,99,0,08,0,1,1,99];
        let mut position = 0;
        let mut relative_base = 0;
        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_read_from_outside_memory() {
        let expected_memory = vec![1,5,1,4,5];
        let expected_position = 4;

        let mut memory = vec![1,5,1,4,99];
        let mut position = 0;
        let mut relative_base = 0;
        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_read_relative_input() {
        let expected_memory = vec![203,5,1,4,99,0,3];

        let mut memory = vec![203,5,1,4,99];
        let mut position = 0;
        let mut relative_base = 1;
        let instruction = Instruction::from(&memory, position, relative_base);
        instruction.execute(&mut memory, &mut position, &mut relative_base, Some(3));

        assert_eq!(memory, expected_memory);
    }


    #[test]
    #[should_panic]
    fn test_execute_halt() {
        let mut memory = vec![2,0,3,3,99];
        let instruction = Instruction::from(&memory, 4, 0);
        let mut position = 4;
        let mut relative_base = 0;
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);
    }
}
