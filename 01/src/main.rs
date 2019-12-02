use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input_file_name: &str = "input.txt";

    let mut masses: Vec<i32> = Vec::new();
    for input in read_file(input_file_name).iter() {
        masses.push(input.parse().unwrap())
    }

    let mut fuel_total: i32 = 0;

    for mass in masses.iter() {
        let mut fuel_required = fuel_for_mass(&mass);
        fuel_total = fuel_total + fuel_required;
        while fuel_for_mass(&fuel_required) > 0 {
            fuel_required = fuel_for_mass(&fuel_required);
            fuel_total = fuel_total + fuel_required;
        }
    }

    println!("Total fuel required: {}", fuel_total);
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No such file");

    let buffer = BufReader::new(file);

    let coll: Vec<String> = buffer.lines().collect::<Result<_,_>>().unwrap();
    coll
}

fn fuel_for_mass(mass: &i32) -> i32 {
    (mass / 3) - 2
}
