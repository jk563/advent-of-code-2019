use std::fs;

fn main() {
    let input = read_file("input.txt");

    let mut part1_input = input.clone();
    part1_input[1] = 12;
    part1_input[2] = 2;
    println!("Part 1: {}", compute(part1_input));
    println!();

    let (noun, verb) = calculate(input);

    println!("Part 2:");
    println!("Noun: {}", noun);
    println!("Verb: {}", verb);
}

fn calculate(input: Vec<u32>) -> (u32, u32) {
    let program: Vec<u32> = input;
    let mut noun_variation: u32 = 0;
    let mut verb_variation: u32 = 0;

    'noun: while noun_variation < 100 {
        'verb: while verb_variation < 100 {
            let mut program_variation = program.clone();

            program_variation[1] = noun_variation;
            program_variation[2] = verb_variation;
        
            if compute(program_variation.clone()) == 19690720 {
                break 'noun;
            }

            verb_variation = verb_variation + 1;
        }

        noun_variation = noun_variation + 1;
        verb_variation = 0
    }

    (noun_variation, verb_variation)
}

fn compute(mut program: Vec<u32>) -> u32 {
    let mut opcode_index = 0;

    while program[opcode_index] != 99 {
        let input1_index_index = opcode_index + 1;
        let input2_index_index = opcode_index + 2;
        let output_index_index = opcode_index + 3;
        
        let output_index = program[output_index_index] as usize;

        let input1_index = program[input1_index_index] as usize;
        let input2_index = program[input2_index_index] as usize;

        let opcode = program[opcode_index];
        let input1 = program[input1_index];
        let input2 = program[input2_index];

        match opcode {
            1 => program[output_index] = input1 + input2,
            2 => program[output_index] = input1 * input2,
            _ => (),
        }
        opcode_index = opcode_index + 4;
    }

    program[0]
}

fn read_file(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("Error reading file");

    let vector: Vec<u32> = contents.trim().split(",").map(|code| code.parse::<u32>().unwrap()).collect();

    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_1: Vec<u32> = vec![1,0,0,0,99];
        assert_eq!(compute(input_1), 2);
    }

    #[test]
    fn test_2() {
        let input_2: Vec<u32> = vec![2,3,0,3,99];
        assert_eq!(compute(input_2), 2);
    }
    
    #[test]
    fn test_3() {
        let input_3: Vec<u32> = vec![2,4,4,5,99,0];
        assert_eq!(compute(input_3), 2);
    }

    #[test]
    fn test_4() {
        let input_4: Vec<u32> = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(compute(input_4), 30);
    }
}
