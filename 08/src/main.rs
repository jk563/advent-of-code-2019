use std::fs;

fn main() {
    let input: Vec<u32> = fs::read_to_string("input_day_8.txt").unwrap().trim().chars().map(|code| code.to_digit(10).unwrap()).collect();

    let width = 3;
    let height = 2;

    let mut layers: Vec<Vec<u32>> = Vec::new();
    layers.push(vec![]);
    for digit in input {
        let num_layers = layers.len();
        if layers.last().unwrap().len() == width * height {
            layers.push(vec![]);
        };
        layers[num_layers - 1].push(digit);
    };

    let mut most_zeros = 0;

    for (layer, i) in layers.enumerate() {
        let zeros = layer.iter().fold(0, |acc, x| acc++);
        match zeros > layers[most_zeros] {
            true => most_zeros = i,
            false => (),
        };
    };

    println!("{}", most_zeros);
}
