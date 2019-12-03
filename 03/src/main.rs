use std::fmt;
use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    println!("Closest: {}", solve1(&contents));
    println!("Least Wire: {}", solve2(&contents));
}

#[derive(Clone)]
#[derive(Eq)]
#[derive(Hash)]
struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {
    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn string_to_vectors(input_string: &str) -> (Vec<String>, Vec<String>) {
    let lines: Vec<&str> = input_string.split_whitespace().collect();
    let wire_1: Vec<String> = lines[0].split(",").map(|x| x.to_string()).collect();
    let wire_2: Vec<String> = lines[1].split(",").map(|x| x.to_string()).collect();
    (wire_1, wire_2)
}

fn path_to_coords(path: &Vec<String>) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = Vec::new();

    let mut last_coordinate = Coordinate { x: 0, y: 0 };

    coordinates.push(last_coordinate.clone());

    for instruction in path {
        let direction = &instruction[0..1];
        let magnitude: &isize = &instruction[1..].parse::<isize>().unwrap();
        match direction {
            "R" => {
                for x in 1..=*magnitude {
                    coordinates.push(Coordinate{ x: last_coordinate.x + x, y: last_coordinate.y });
                }
            },
            "L" => {
                for x in 1..=*magnitude {
                    coordinates.push(Coordinate{ x: last_coordinate.x - x, y: last_coordinate.y });
                }
            },
            "U" => {
                for y in 1..=*magnitude {
                    coordinates.push(Coordinate{ x: last_coordinate.x, y: last_coordinate.y + y });
                }
            },
            "D" => { 
                for y in 1..=*magnitude {
                    coordinates.push(Coordinate{ x: last_coordinate.x, y: last_coordinate.y - y });
                }
            },
            _ => (),
        }

        last_coordinate = coordinates.last().unwrap().clone();
    }

    coordinates
}

fn wire_intersections(wire_1: &Vec<Coordinate>, wire_2: &Vec<Coordinate>) -> Vec<Coordinate> {
    let hash_1: HashSet<Coordinate> = HashSet::from_iter(wire_1.iter().cloned());
    let hash_2: HashSet<Coordinate> = HashSet::from_iter(wire_2.iter().cloned());
    hash_1.intersection(&hash_2).cloned().collect::<Vec<Coordinate>>()
}

fn closest(intersections: &Vec<Coordinate>) -> isize {
    let mut closest = std::isize::MAX;
    for point in intersections {
        let distance = point.manhattan_distance();
        if distance < closest && distance != 0 {
            closest = distance;
        }
    }
    closest
}

fn least_distance(wire_1: &Vec<Coordinate>, wire_2: &Vec<Coordinate>) -> isize {
    let intersections = wire_intersections(&wire_1, &wire_2);
    let mut distance = std::isize::MAX;

    for intersection in intersections {
        let mut intersection_distance: isize= 0;
        for (i, coord) in wire_1.iter().enumerate() {
            if coord == &intersection {
                intersection_distance = i as isize;
                break;
            }
        }
        for (i, coord) in wire_2.iter().enumerate() {
            if coord == &intersection {
                intersection_distance = intersection_distance + i as isize;
                break;
            }
        }
        if intersection_distance < distance && intersection_distance != 0 {
            distance = intersection_distance;
        }
    }
    
    distance
}

fn solve1(input: &str) -> isize {
    let (path_1, path_2) = string_to_vectors(input);
    let wire_1: Vec<Coordinate> = path_to_coords(&path_1);
    let wire_2: Vec<Coordinate> = path_to_coords(&path_2);
    closest(&wire_intersections(&wire_1, &wire_2))
}

fn solve2(input: &str) -> isize {
    let (path_1, path_2) = string_to_vectors(input);
    let wire_1: Vec<Coordinate> = path_to_coords(&path_1);
    let wire_2: Vec<Coordinate> = path_to_coords(&path_2);
    least_distance(&wire_1, &wire_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(Coordinate{ x: 5, y: 5 }.manhattan_distance(), 10);
        assert_eq!(Coordinate{ x: 0, y: 0 }.manhattan_distance(), 0);
        assert_eq!(Coordinate{ x: -5, y: 15 }.manhattan_distance(), 20);
        assert_eq!(Coordinate{ x: -3, y: -17 }.manhattan_distance(), 20);
    }
    
    #[test]
    fn test_vector_split() {
        let input_string = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let (output_1, output_2) = string_to_vectors(input_string);
        assert_eq!(output_1, vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"]);
        assert_eq!(output_2, vec!["U62","R66","U55","R34","D71","R55","D58","R83"]);
    }
    
    #[test]
    fn test_path_to_coords() {
        let path: Vec<String> = vec!["R2".to_string(), "U3".to_string()];
        assert_eq!(path_to_coords(&path), vec![Coordinate{ x: 0, y: 0 }, Coordinate{ x: 1, y: 0 }, Coordinate{ x: 2, y: 0 }, Coordinate{ x: 2, y: 1}, Coordinate{ x: 2, y: 2 }, Coordinate{ x: 2, y: 3 }]);
    }

    #[test]
    fn test_1() {
        let input_string = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve1(input_string), 159);
    }

    #[test]
    fn test_2() {
        let input_string = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve1(input_string), 135);
    }

    #[test]
    fn test_1_part2() {
        let input_string = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve2(input_string), 610);
    }

    #[test]
    fn test_2_part2() {
        let input_string = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve2(input_string), 410);
    }
}
