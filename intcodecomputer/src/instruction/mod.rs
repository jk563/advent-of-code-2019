pub mod opcode;
pub mod parametermode;
use opcode::Opcode;
use parametermode::ParameterMode;

use std::convert::TryFrom;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    parameter_positions: Option<Vec<usize>>,
    parameter_modes: Vec<ParameterMode>,
}

impl Instruction {
    pub fn from(memory: &Vec<isize>, position: usize, relative_base: usize) -> Instruction {
        let instruction_code = usize::try_from(memory[position]).unwrap();
        let mut parameter_modes: Vec<ParameterMode> = Vec::new();

        let opcode = Opcode::from(instruction_code % 100);

        for x in 1..=(opcode.parameter_count()) {
            // Get the digit corrosponding to the parameter mode
            let mode_integer = (instruction_code / 10) / (10_usize.pow(u32::try_from(x).unwrap())) % 10;
            parameter_modes.push(ParameterMode::from(mode_integer));
        }
        
        let mut instruction = Instruction {
            opcode: opcode,
            parameter_positions: None,
            parameter_modes: parameter_modes,
        };
        instruction.load_parameter_positions(memory, position, relative_base);
        instruction
    }

    fn load_parameter_positions(&mut self, memory: &Vec<isize>, position: usize, relative_base: usize) {
        let param_count = self.opcode.parameter_count();
        let mut parameter_positions: Vec<usize> = Vec::new();

        for param_index in 1..=param_count {
            parameter_positions.push(match self.parameter_modes[param_index - 1] {
                    ParameterMode::PositionalMode => match position + param_index < memory.len() {
                        true => usize::try_from(memory[position + param_index]).unwrap(),
                        false => 0,
                    },
                    ParameterMode::ImmediateMode => position + param_index,
                    ParameterMode::RelativeMode => match position + param_index < memory.len() {
                        true => usize::try_from(isize::try_from(relative_base).unwrap() + memory[position + param_index]).unwrap(),
                        false => usize::try_from(isize::try_from(relative_base).unwrap()).unwrap() + 0,
                    },
            });
        };

        self.parameter_positions = Some(parameter_positions);
    }

    pub fn execute(&self, memory: &mut Vec<isize>, position: &mut usize, relative_base: &mut usize, input: Option<isize>) -> Option<isize> {
        let parameter_positions = self.parameter_positions.clone().unwrap();
        let mut output: Option<isize> = None;
        let mut required_memory_size = memory.len();
        for parameter_position in parameter_positions.clone() {
            if parameter_position >= required_memory_size {
                required_memory_size = parameter_position + 1;
            };
        }
        memory.resize(required_memory_size, 0);
        match self.opcode {
            Opcode::Add => {
                let result = memory[parameter_positions[0]] + memory[parameter_positions[1]];

                memory[parameter_positions[2]] = result;
                *position += 4; 
            },
            Opcode::Multiply => {
                let result = memory[parameter_positions[0]] * memory[parameter_positions[1]];

                memory[parameter_positions[2]] = result;
                *position += 4; 
            },
            Opcode::Input => {
                memory[parameter_positions[0]] = input.unwrap();
                *position += 2;
            },
            Opcode::Output => {
                output = Some(memory[parameter_positions[0]]);
                *position += 2;
            },
            Opcode::JumpTrue => {
                *position = match memory[parameter_positions[0]] != 0 {
                    true => usize::try_from(memory[parameter_positions[1]]).unwrap(),
                    false => *position + 3, 
                };
            },
            Opcode::JumpFalse => {
                *position = match memory[parameter_positions[0]] == 0 {
                    true => usize::try_from(memory[parameter_positions[1]]).unwrap(),
                    false => *position + 3, 
                };
            },
            Opcode::Less => {
                memory[parameter_positions[2]] = match memory[parameter_positions[0]] < memory[parameter_positions[1]] {
                    true => 1,
                    false => 0,
                };
                *position += 4;
            },
            Opcode::Equal => {
                memory[parameter_positions[2]] = match memory[parameter_positions[0]] == memory[parameter_positions[1]] {
                    true => 1,
                    false => 0,
                };
                *position += 4;
            },
            Opcode::RelativeBase => {
                *relative_base = usize::try_from(isize::try_from(*relative_base).unwrap() + memory[parameter_positions[0]]).unwrap();
                *position += 2
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
        let expected_parameters: Vec<usize> = vec![1,1,3];

        let memory  = vec![101,1,1,3,99];
        let instruction = Instruction::from(&memory, 0, 0);

        assert_eq!(instruction.opcode, expected_opcode);
        assert_eq!(instruction.parameter_positions.unwrap(), expected_parameters);
        assert_eq!(instruction.parameter_modes, 
                vec![
                    ParameterMode::ImmediateMode,
                    ParameterMode::PositionalMode, 
                    ParameterMode::PositionalMode, 
                ]
        );
    }

    #[test]
    fn test_from_halt_only() {
        let expected_opcode: Opcode = Opcode::Halt;
        let expected_parameters: Vec<usize> = vec![];

        let memory  = vec![99,0,0,3,99];
        let instruction = Instruction::from(&memory, 0, 0);

        assert_eq!(instruction.opcode, expected_opcode);
        assert_eq!(instruction.parameter_positions.unwrap(), expected_parameters);
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
        let instruction = Instruction::from(&memory, 0, 0);
        let mut position = 0;
        let mut relative_base = 0;
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
    fn test_execute_relativebase() {
        let expected_memory = vec![109,2,109,5,99];
        let expected_relative_base = 7;

        let mut memory = vec![109,2,109,5,99];
        let mut position = 0;
        let mut relative_base = 0;
        let instruction = Instruction::from(&memory, position, 0);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);
        let instruction = Instruction::from(&memory, position, relative_base);
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

        assert_eq!(memory, expected_memory);
        assert_eq!(relative_base, expected_relative_base);
    }

    #[test]
    fn test_accessing_memory_out_of_range() {
        let expected_memory = vec![1,5,1,7,99,0,0,5];

        let mut memory = vec![1,5,1,7,99];
        let instruction = Instruction::from(&memory, 0, 0);
        let mut position = 0;
        let mut relative_base = 0;
        instruction.execute(&mut memory, &mut position, &mut relative_base, None);

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
