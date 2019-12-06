use std::collections::HashMap;

fn main() {
}

#[derive(Default, Debug, PartialEq)]
struct OrbitalObject {
    name: String,
    orbit: Option<Box<OrbitalObject>>,
}

fn create_input_hashmap(input: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for relationship in input.trim().split_whitespace().collect::<Vec<&str>>() {
        let object_names = relationship.split(")").collect::<Vec<&str>>();
        map.insert(object_names[1].to_string(), object_names[0].to_string());
    }
    map
}

fn create_orbital_objects(input: HashMap<String, String>) -> Vec<OrbitalObject> {
    vec![]
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
        let input = String::from("BBB)AAA\nAAA)CCC\nCOM)BBB");
        let hashmap: HashMap<String, String> = create_input_hashmap(input);

        let com: OrbitalObject = OrbitalObject{ name: String::from("COM"), orbit: None };
        let b: OrbitalObject = OrbitalObject{ name: String::from("BBB"), orbit: Some(Box::new(&com)) };
        let a: OrbitalObject = OrbitalObject{ name: String::from("AAA"), orbit: Some(Box::new(&b))};
        let c: OrbitalObject = OrbitalObject{ name: String::from("CCC"), orbit: Some(Box::new(&a))};
        
        let orbital_objects: Vec<OrbitalObject> = create_orbital_objects(hashmap);
        let expected_orbital_objects: Vec<OrbitalObject> = vec![a,c,b,com];
        assert_eq!(expected_orbital_objects, orbital_objects);
    }
}
