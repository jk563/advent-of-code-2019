use std::fs;
use std::convert::TryFrom;

fn main() {
    let mut program = read_input();
    let mut instruction_pointer = 0;
    let (new_program, next_instruction_pointer, stop) = perform_instruction(program, instruction_pointer, Some(5));
        let mut halt = stop;
        program = new_program;
        instruction_pointer = next_instruction_pointer;
    while !halt {
        let (new_program, next_instruction_pointer, stop) = perform_instruction(program.clone(), instruction_pointer, None);
        halt = stop;
        program = new_program;
        instruction_pointer = next_instruction_pointer;
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl Default for ParameterMode {
    fn default() -> Self { ParameterMode::PositionMode }
}

impl ParameterMode {
    fn from(mode: &str) -> ParameterMode {
        match mode {
            "1" => return ParameterMode::ImmediateMode,
            "0" | _ => return ParameterMode::PositionMode,
        };
    }
}

fn get_parameter(program: &Vec<String>, argument_index: usize, parameter_mode: ParameterMode) -> isize {
    let argument = program[argument_index].parse::<isize>().unwrap();
    match parameter_mode {
        ParameterMode::ImmediateMode => argument,
        ParameterMode::PositionMode => program[usize::try_from(argument).unwrap()].parse::<isize>().unwrap(),
    }
}

fn perform_instruction(mut program: Vec<String>, instruction_pointer: usize, input: Option<isize>) -> (Vec<String>, usize, bool) {
    let instruction = Instruction::from(&program[instruction_pointer]);
    let mut instruction_changed = false;
    let mut next_instruction_pointer = instruction_pointer;
    let parameter_1 = match instruction.opcode {
        Opcode::Halt => isize::try_from(instruction_pointer).unwrap(),
        _ => get_parameter(&program, instruction_pointer + 1, ParameterMode::ImmediateMode),
    };

    match instruction.opcode {
        Opcode::Input => {
            if let Some(input_integer) = input {
                program[usize::try_from(parameter_1).unwrap()] = input_integer.to_string();
            }
        },
        Opcode::Output => println!("Output: {}", program[usize::try_from(parameter_1).unwrap()]),
        Opcode::Add | Opcode::Multiply => {
            let parameter_1 = get_parameter(&program, instruction_pointer + 1, instruction.first_parameter_mode);
            let parameter_2 = get_parameter(&program, instruction_pointer + 2, instruction.second_parameter_mode);
            let parameter_3 = get_parameter(&program, instruction_pointer + 3, ParameterMode::ImmediateMode);

            let result = match instruction.opcode {
                Opcode::Add => parameter_1 + parameter_2,
                Opcode::Multiply => parameter_1 * parameter_2,
                _ => 323232323233,
            };

            program[usize::try_from(parameter_3).unwrap()] = result.to_string();
        },
        Opcode::JumpTrue => {
            let parameter_1 = get_parameter(&program, instruction_pointer + 1, instruction.first_parameter_mode);
            let parameter_2 = get_parameter(&program, instruction_pointer + 2, instruction.second_parameter_mode);

            if parameter_1 > 0 {
                instruction_changed = true;
                next_instruction_pointer = usize::try_from(parameter_2).unwrap();
            }
        },
        Opcode::JumpFalse => {
            let parameter_1 = get_parameter(&program, instruction_pointer + 1, instruction.first_parameter_mode);
            let parameter_2 = get_parameter(&program, instruction_pointer + 2, instruction.second_parameter_mode);

            if parameter_1 == 0 {
                instruction_changed = true;
                next_instruction_pointer = usize::try_from(parameter_2).unwrap();
            }
        },
        Opcode::Less => {
            let parameter_1 = get_parameter(&program, instruction_pointer + 1, instruction.first_parameter_mode);
            let parameter_2 = get_parameter(&program, instruction_pointer + 2, instruction.second_parameter_mode);
            let parameter_3 = get_parameter(&program, instruction_pointer + 3, ParameterMode::ImmediateMode);
            
            let result = match parameter_1 < parameter_2 {
                true => 1,
                false => 0,
            };

            program[usize::try_from(parameter_3).unwrap()] = result.to_string();
        },
        Opcode::Equal => {
            let parameter_1 = get_parameter(&program, instruction_pointer + 1, instruction.first_parameter_mode);
            let parameter_2 = get_parameter(&program, instruction_pointer + 2, instruction.second_parameter_mode);
            let parameter_3 = get_parameter(&program, instruction_pointer + 3, ParameterMode::ImmediateMode);
            
            let result = match parameter_1 == parameter_2 {
                true => 1,
                false => 0,
            };

            program[usize::try_from(parameter_3).unwrap()] = result.to_string();
        },
        _ => (),
    }

    next_instruction_pointer = match instruction.opcode {
        Opcode::Input | Opcode::Output => instruction_pointer + 2,
        Opcode::Add | Opcode::Multiply | Opcode::Less | Opcode::Equal => instruction_pointer + 4,
        Opcode::JumpTrue | Opcode::JumpFalse => {
            match instruction_changed {
                true => next_instruction_pointer,
                false => instruction_pointer + 3,
            }
        }
        _ => instruction_pointer,
    };
    
    (program, next_instruction_pointer, instruction.opcode == Opcode::Halt)
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
    Halt,
    Unset,
}

impl Default for Opcode {
    fn default() -> Self { Opcode::Unset }
}

impl Opcode {
    fn from(code: &str) -> Opcode {
        match code {
            "01" | "1" => return Opcode::Add,
            "02" | "2" => return Opcode::Multiply,
            "03" | "3" => return Opcode::Input,
            "04" | "4" => return Opcode::Output,
            "05" | "5" => return Opcode::JumpTrue,
            "06" | "6" => return Opcode::JumpFalse,
            "07" | "7" => return Opcode::Less,
            "08" | "8" => return Opcode::Equal,
            "99" => return Opcode::Halt,
            _ => panic!(),
        };
    }
}

#[derive(Default)]
#[derive(Debug)]
struct Instruction {
    third_parameter_mode: ParameterMode,
    second_parameter_mode: ParameterMode,
    first_parameter_mode: ParameterMode,
    opcode: Opcode,
}

impl Instruction {
    fn from(mut instruction_string: &str) -> Instruction {
        let mut instruction = Instruction{ ..Default::default() };
        while instruction_string.len() != 0 {
            match instruction_string.len() {
                1 | 2 => {
                    instruction.opcode = Opcode::from(instruction_string);
                    if instruction_string.len() == 2 { instruction_string = &instruction_string[1..]; }
                },
                3 => {
                    instruction.first_parameter_mode = ParameterMode::from(&instruction_string[0..1]); 
                },
                4 => {
                    instruction.second_parameter_mode = ParameterMode::from(&instruction_string[0..1]); 
                },
                5 => {
                    instruction.third_parameter_mode = ParameterMode::from(&instruction_string[0..1]); 
                },
                _ => (),
            }
            instruction_string = &instruction_string[1..];
        }
        instruction
    }
}

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    let vector: Vec<String> = contents.trim().split(",").map(|code| code.parse::<String>().unwrap()).collect();

    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_only() {
        let instruction = Instruction::from("02");
        assert_eq!(instruction.opcode, Opcode::Multiply);
        assert_eq!(instruction.first_parameter_mode, ParameterMode::PositionMode);
        assert_eq!(instruction.second_parameter_mode, ParameterMode::PositionMode);
        assert_eq!(instruction.third_parameter_mode, ParameterMode::PositionMode);
    }

    #[test]
    fn test_parse_opcode_one_extra() {
        let instruction = Instruction::from("103");
        assert_eq!(instruction.opcode, Opcode::Input);
        assert_eq!(instruction.first_parameter_mode, ParameterMode::ImmediateMode);
        assert_eq!(instruction.second_parameter_mode, ParameterMode::PositionMode);
        assert_eq!(instruction.third_parameter_mode, ParameterMode::PositionMode);
    }

    #[test]
    fn test_parse_opcode_two_extra() {
        let instruction = Instruction::from("1004");
        assert_eq!(instruction.opcode, Opcode::Output);
        assert_eq!(instruction.first_parameter_mode, ParameterMode::PositionMode);
        assert_eq!(instruction.second_parameter_mode, ParameterMode::ImmediateMode);
        assert_eq!(instruction.third_parameter_mode, ParameterMode::PositionMode);
    }

    #[test]
    fn test_parse_opcode_three_extra() {
        let instruction = Instruction::from("11101");
        assert_eq!(instruction.opcode, Opcode::Add);
        assert_eq!(instruction.first_parameter_mode, ParameterMode::ImmediateMode);
        assert_eq!(instruction.second_parameter_mode, ParameterMode::ImmediateMode);
        assert_eq!(instruction.third_parameter_mode, ParameterMode::ImmediateMode);
    }

    #[test]
    fn test_perform_instruction() {
        let input = "1,0,0,3,99";
        let program: Vec<String> = input.trim().split(",").map(|code| code.parse::<String>().unwrap()).collect();
        let (program, _next_instruction, _halt) = perform_instruction(program, 0, None);
        assert_eq!(program, "1,0,0,2,99".to_string().trim().split(",").map(|code| code.parse::<String>().unwrap()).collect::<Vec<String>>());
    }

    #[test]
    fn test_perform_multiply() {
        let input = "2,3,0,3,99";
        let program: Vec<String> = input.trim().split(",").map(|code| code.parse::<String>().unwrap()).collect();
        let (program, _next_instruction, _halt) = perform_instruction(program, 0, None);
        assert_eq!(program, "2,3,0,6,99".to_string().trim().split(",").map(|code| code.parse::<String>().unwrap()).collect::<Vec<String>>());
    }
}
