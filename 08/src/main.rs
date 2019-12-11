use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let input: Vec<u32> = fs::read_to_string("input_day_8.txt").unwrap().trim().chars().map(|code| code.to_digit(10).unwrap()).collect();

    let width = 25;
    let height = 6;

    let mut layers: Vec<Vec<u32>> = Vec::new();
    layers.push(vec![]);
    for digit in input {
        if layers.last().unwrap().len() == (width * height) {
            layers.push(vec![]);
        };
        let num_layers = layers.len();
        layers[num_layers - 1].push(digit);
    };

    let mut least_zeros = layers[0].len();
    let mut layer_number = 0;

    for (i, layer) in layers.iter().enumerate() {
        let zeros = layer.iter().filter(|x| **x == 0).fold(0, |acc, _| acc + 1);
        match zeros < least_zeros {
            true => { 
                least_zeros = zeros;
                layer_number = i;   
            },
            false => (),
        };
    };

    let ones = layers[layer_number].iter().filter(|x| **x == 1).fold(0, |acc, _| acc + 1);
    let twos = layers[layer_number].iter().filter(|x| **x == 2).fold(0, |acc, _| acc + 1);

    println!("Check: {}", ones * twos);


    let mut message: Vec<u32> = Vec::new();

    for pixel in 0..150 {
        let mut colour = 2;
        let mut i = 0;
        while colour == 2 {
            colour = layers[i][pixel];
            if i < 150 {
                i += 1;
                continue;
            } else {
                break;
            }
        };
        message.push(colour)
    };

    let mut stuff_str = String::new();

    for num in message {
        stuff_str.push_str(match num {
            1 => "#",
            _ => " ",
        });
    }

    println!("{}", &stuff_str[0 .. 25]);
    println!("{}", &stuff_str[25 .. 50]);
    println!("{}", &stuff_str[50 .. 75]);
    println!("{}", &stuff_str[75 .. 100]);
    println!("{}", &stuff_str[100 .. 125]);
    println!("{}", &stuff_str[125 .. 150]);
}
