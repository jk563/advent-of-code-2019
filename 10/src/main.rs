use std::fs;

// Asteroid = 1
// Space = 0
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut coords:Vec<Coordinate> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, thing) in line.chars().enumerate() {
            match thing {
                '#' => coords.push(Coordinate::from(x, y)),
                _ => (),
            };
        };
    };

}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn from(x: usize, y: usize) -> Coordinate {
        Coordinate{ x, y}
    }
}
