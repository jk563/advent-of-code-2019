use std::fs;

fn main() {
    let mut input = read_input();
    input = perform_instruction(input, 0);
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
        println!("{}", mode);
        match mode {
            "1" => return ParameterMode::ImmediateMode,
            "0" | _ => return ParameterMode::PositionMode,
        };
    }
}

fn perform_instruction(mut program: Vec<String>, instruction_pointer: usize) -> Vec<String> {
    let instruction = Instruction::from(&program[instruction_pointer]);
    
    if instruction.opcode == Opcode::Halt {()}

    let arguments: Vec<String> = match instruction.opcode {
        Opcode::Add | Opcode::Multiply => vec![program[instruction_pointer + 1].to_string(), program[instruction_pointer + 2].to_string(), program[instruction_pointer + 3].to_string()],
        Opcode::Input | Opcode::Output => vec![program[instruction_pointer + 1].to_string()],
        _ => Vec::new(),
    };

    let parameter_1 = match instruction.first_parameter_mode {
        ParameterMode::ImmediateMode => 
    }

    match instruction.opcode {
        Opcode::Add => {
            program[0] = "0".to_string();
        },
        _ => (),
    };

    program
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
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
            "99" => return Opcode::Halt,
            _ => return Opcode::Unset,
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
        let mut program: Vec<String> = input.trim().split(",").map(|code| code.parse::<String>().unwrap()).collect();
        let arguments = vec!["0".to_string(), "0".to_string(), "3".to_string()];
        let instruction = Instruction::from("1");
        println!("Result: {:?}", program);
        program = perform_instruction(program, 0);
        println!("Result: {:?}", program);
//        assert_eq!(program, "1,0,0,2,99");
        assert!(false);
    }
}
