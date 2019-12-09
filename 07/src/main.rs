use std::fs;

mod intcodecomputer;
use intcodecomputer::IntcodeComputer;

fn main() {
    let input: Vec<isize> = fs::read_to_string("input_day_7.txt").unwrap().trim().split(",").map(|code| code.parse::<isize>().unwrap()).collect();

    let mut amp1 = IntcodeComputer::new();
    let mut amp2 = IntcodeComputer::new();
    let mut amp3 = IntcodeComputer::new();
    let mut amp4 = IntcodeComputer::new();
    let mut amp5 = IntcodeComputer::new();

    let mut highest_thrust = 0;
    let mut amp_number = 0;
    let mut amps = vec![amp1, amp2, amp3, amp4, amp5];
    for combination in generate_combinations() {
        amps[0].load(&input);
        amps[1].load(&input);
        amps[2].load(&input);
        amps[3].load(&input);
        amps[4].load(&input);

        let mut output = amps[0].run(Some(vec![combination[0], 0]));
        output = amps[1].run(Some(vec![combination[1], output[0]]));
        output = amps[2].run(Some(vec![combination[2], output[0]]));
        output = amps[3].run(Some(vec![combination[3], output[0]]));
        output = amps[4].run(Some(vec![combination[4], output[0]]));
        let mut amp_number = 0;
        let mut last_output = 0;
        while output.len() != 0 {
            last_output = output[0];
            output = amps[amp_number].run(Some(vec![output[0]]));
            amp_number = match amp_number {
                0 | 1 | 2 | 3 => amp_number + 1,
                _ => 0,
            };
        }

        highest_thrust = highest_thrust.max(last_output);
    }

    println!("Highest Thrust: {}", highest_thrust);
}



fn generate_combinations() -> Vec<Vec<isize>> {
    let mut combinations: Vec<Vec<isize>> = Vec::new();

    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    for e in 5..=9 {
                        let potential_combination = vec![a, b, c, d, e];
                        if potential_combination.contains(&5) &&
                           potential_combination.contains(&6) &&
                           potential_combination.contains(&7) &&
                           potential_combination.contains(&8) &&
                           potential_combination.contains(&9) {
                            combinations.push(potential_combination);
                        };
                    };
                };
            };
        };
    };

    combinations
}
