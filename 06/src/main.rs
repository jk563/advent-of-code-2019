use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    let inputs = create_input_hashmap(contents);

    let orbit_totals = create_orbits(inputs.clone());

    let checksum: usize = orbit_totals.values().sum();

    println!("Checksum: {}", checksum);
    
    println!("Jumps: {}", jumps_to_san(inputs));
}

fn jumps_to_san(inputs: HashMap<String, String>) -> usize {
    let san_to_com = route_to_com(inputs.clone(), String::from("SAN"));
    let you_to_com = route_to_com(inputs.clone(), String::from("YOU"));
    
    let san_to_com_set: HashSet<String> = HashSet::from_iter(san_to_com.iter().cloned());
    let you_to_com_set: HashSet<String> = HashSet::from_iter(you_to_com.iter().cloned());

    let you_to_san_set: Vec<&String> = you_to_com_set.symmetric_difference(&san_to_com_set).collect::<Vec<&String>>();

    // Remove SAN and YOU. Doesn't include the joining node, but nodes-1 is the answer so ¯\_(ツ)_/¯
    you_to_san_set.len() - 2
}

fn route_to_com(inputs: HashMap<String, String>, start_orbit: String) -> Vec<String> {
    let mut route_to_com: Vec<String> = vec![start_orbit.clone()];
    let mut current_orbit = start_orbit;

    while current_orbit != "COM" {
        current_orbit = inputs.get(&current_orbit).unwrap().to_string();
        route_to_com.push(current_orbit.clone());
    }

    route_to_com
}

fn create_input_hashmap(input: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for relationship in input.trim().split_whitespace().collect::<Vec<&str>>() {
        let object_names = relationship.split(")").collect::<Vec<&str>>();
        map.insert(object_names[1].to_string(), object_names[0].to_string());
    }
    map
}

fn create_orbits(input: HashMap<String, String>) -> HashMap<String, usize> {
    let mut orbit_totals: HashMap<String, usize> = HashMap::new();
    orbit_totals.insert(String::from("COM"), 0);

    while orbit_totals.len() < input.len() + 1 {
        for key in input.keys() {
            match orbit_totals.get(input.get(key).unwrap()).cloned() {
                Some(orbits) => {
                    orbit_totals.insert(key.to_string(), orbits + 1);
                }
                None => (),
            }
        }
    }

    orbit_totals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_hashmap() {
        let input = String::from("BBB)AAA\nAAA)CCC\nCOM)BBB");
        let hashmap: HashMap<String, String> = create_input_hashmap(input);
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert(String::from("AAA"), String::from("BBB"));
        expected_hashmap.insert(String::from("BBB"), String::from("COM"));
        expected_hashmap.insert(String::from("CCC"), String::from("AAA"));
        assert_eq!(expected_hashmap, hashmap);
    }

    #[test]
    fn test_create_orbital_objects() {
        let mut input_hashmap = HashMap::new();
        input_hashmap.insert(String::from("AAA"), String::from("BBB"));
        input_hashmap.insert(String::from("BBB"), String::from("COM"));
        input_hashmap.insert(String::from("CCC"), String::from("AAA"));
        let orbits_hashmap = create_orbits(input_hashmap);

        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert(String::from("AAA"), 2);
        expected_hashmap.insert(String::from("BBB"), 1);
        expected_hashmap.insert(String::from("CCC"), 3);
        expected_hashmap.insert(String::from("COM"), 0);

        assert_eq!(expected_hashmap, orbits_hashmap);
    }
}
