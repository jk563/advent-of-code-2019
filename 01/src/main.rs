use std::fs::read_to_string;
use std::iter::successors;

fn main() {
    let masses: Vec<isize> = read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let fuel_for_mass: isize = masses.clone().into_iter().map(calculate_fuel).sum();
    println!("Fuel for mass: {}", fuel_for_mass);

    let total_fuel: isize = masses
        .into_iter()
        .map(calculate_fuel)
        .flat_map(calculate_fuel_for_fuel)
        .sum();
    println!("Total fuel required: {}", total_fuel);
}

fn calculate_fuel(mass: isize) -> isize {
    0.max((mass / 3) - 2)
}

fn calculate_fuel_for_fuel(fuel: isize) -> impl Iterator<Item = isize> {
    successors(Some(fuel), |&fuel| match calculate_fuel(fuel) {
        0 => None,
        _ => Some(calculate_fuel(fuel)),
    })
}
