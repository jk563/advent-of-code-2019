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
    pub fn from(memory: &Vec<isize>, position: usize) -> Instruction {
        let instruction_code = usize::try_from(memory[position]).unwrap();
        let mut parameter_modes: Vec<ParameterMode> = Vec::new();

        let opcode = Opcode::from(instruction_code % 100);

        for x in 1..=(opcode.parameter_count()) {
            // Get the digit corrosponding to the parameter mode
            let mode_integer = (instruction_code / 100) / 1.max(10 * (x - 1)) % 10;
            if x == opcode.parameter_count() {
                parameter_modes.push( match opcode {
                        Opcode::JumpTrue | Opcode::JumpFalse => ParameterMode::from(mode_integer),
                        _ => ParameterMode::ImmediateMode,
                    }
                );
            } else {
                parameter_modes.push(ParameterMode::from(mode_integer));
            }
        }
        
        let mut instruction = Instruction {
            opcode: opcode,
            parameters: None,
            parameter_modes: parameter_modes,
        };
        instruction.load_parameters(memory, position);
        instruction
    }

    fn load_parameters(&mut self, memory: &Vec<isize>, position: usize) {
        let param_count = self.opcode.parameter_count();
        let mut parameters: Vec<isize> = Vec::new();

        for param_index in 1..=param_count {
            let param = memory[position + param_index];

            parameters.push(match self.parameter_modes[param_index - 1] {
                    ParameterMode::PositionalMode => memory[usize::try_from(param).unwrap()],
                    ParameterMode::ImmediateMode => param,
            });
        };

        self.parameters = Some(parameters);
    }

    pub fn execute(&self, memory: &mut Vec<isize>, position: &mut usize, input: Option<isize>) -> Option<isize> {
        let parameters = self.parameters.clone().unwrap();
        let mut output: Option<isize> = None;
        match self.opcode {
            Opcode::Add => {
                let result = parameters[0] + parameters[1];

                memory[usize::try_from(parameters[2]).unwrap()] = result;
                *position += 4; 
            },
            Opcode::Multiply => {
                let result = parameters[0] * parameters[1];

                memory[usize::try_from(parameters[2]).unwrap()] = result;
                *position += 4; 
            },
            Opcode::Input => {
                memory[usize::try_from(parameters[0]).unwrap()] = input.unwrap();
                *position += 2;
            },
            Opcode::Output => {
                output = Some(memory[usize::try_from(parameters[0]).unwrap()]);
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
                memory[usize::try_from(parameters[2]).unwrap()] = match parameters[0] < parameters[1] {
                    true => 1,
                    false => 0,
                };
                *position += 4;
            },
            Opcode::Equal => {
                memory[usize::try_from(parameters[2]).unwrap()] = match parameters[0] == parameters[1] {
                    true => 1,
                    false => 0,
                };
                *position += 4;
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
        let instruction = Instruction::from(&memory, 0);

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
        let instruction = Instruction::from(&memory, 0);

        assert_eq!(instruction.opcode, expected_opcode);
        assert_eq!(instruction.parameters.unwrap(), expected_parameters);
        assert_eq!(instruction.parameter_modes, vec![]);
    }

    #[test]
    fn test_execute_add() {
        let expected_memory = vec![1,0,0,2,99];
        let expected_position = 4;

        let mut memory = vec![1,0,0,3,99];
        let instruction = Instruction::from(&memory, 0);
        let mut position = 0;
        instruction.execute(&mut memory, &mut position, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_multiply() {
        let expected_memory = vec![2,0,3,6,99];
        let expected_position = 4;

        let mut memory = vec![2,0,3,3,99];
        let instruction = Instruction::from(&memory, 0);
        let mut position = 0;
        instruction.execute(&mut memory, &mut position, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_input() {
        let expected_memory = vec![3,5,99];
        let expected_position = 2;

        let mut memory = vec![3,1,99];
        let instruction = Instruction::from(&memory, 0);
        let mut position = 0;
        let input: isize = 5;
        instruction.execute(&mut memory, &mut position, Some(input));

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_output() {
        let expected_memory = vec![4,2,99];
        let expected_position = 2;
        let expected_output = Some(99);

        let mut memory = vec![4,2,99];
        let instruction = Instruction::from(&memory, 0);
        let mut position = 0;
        let output = instruction.execute(&mut memory, &mut position, None);

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

        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);
        assert_eq!(position, expected_first_position);

        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);
        assert_eq!(position, expected_second_position);
    }

    #[test]
    fn test_execute_jumpfalse() {
        let expected_first_position = 3;
        let expected_second_position = 5;

        let mut memory = vec![1106,5,99,106,0,1];
        let mut position = 0;

        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);
        assert_eq!(position, expected_first_position);

        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);
        assert_eq!(position, expected_second_position);
    }

    #[test]
    fn test_execute_less() {
        let expected_memory = vec![0,0,1,0,1007,0,1,2,99];
        let expected_position = 8;

        let mut memory = vec![1007,0,99,0,1007,0,1,2,99];
        let mut position = 0;
        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);

        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_execute_equal() {
        let expected_memory = vec![1,0,99,0,08,0,1,1,99];
        let expected_position = 8;

        let mut memory = vec![1108,99,99,0,08,0,1,1,99];
        let mut position = 0;
        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);

        let instruction = Instruction::from(&memory, position);
        instruction.execute(&mut memory, &mut position, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(position, expected_position);
    }

    #[test]
    #[should_panic]
    fn test_execute_halt() {
        let mut memory = vec![2,0,3,3,99];
        let instruction = Instruction::from(&memory, 4);
        let mut position = 4;
        instruction.execute(&mut memory, &mut position, None);
    }
}
