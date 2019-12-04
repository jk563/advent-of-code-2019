use std::fs;

fn main() {
    let (start_of_range, end_of_range) = read_file();
    println!("Start of range: {}", start_of_range);
    println!("End of range: {}", end_of_range);

    let possible_passwords = possible_passwords(start_of_range, end_of_range);
    println!("Number of possible passwords: {}", possible_passwords.len());

    let passwords_passing_enhanced_check = extra_check(&possible_passwords);
    println!("Number of passwords passing extra check: {}", passwords_passing_enhanced_check.len())
}

fn extra_check(passwords: &Vec<String>) -> Vec<String> {
    let mut passwords_passing_extra_check: Vec<String> = Vec::new();

    for password in passwords {
        if meets_extra_check(&password) {
            passwords_passing_extra_check.push(String::from(password));
        }
    }

    passwords_passing_extra_check
}

fn meets_extra_check(password: &str) -> bool {
    let mut pair = false;
    let mut false_pair = false;
    let mut previous_digit = 0;

    for character in password.chars() {
        let new_digit = character.to_digit(10).unwrap();
        if previous_digit != new_digit {
            if pair { break; }
            false_pair = false;
        } else if previous_digit == new_digit {
            if false_pair { continue; }
            if pair {
                false_pair = true;
                pair = false;
            } else {
                pair = true;
            }
        }
        previous_digit = new_digit;
    }

    return pair
}

fn possible_passwords(start_of_range: u32, end_of_range: u32) -> Vec<String> {
    let mut potential_passwords: Vec<String> = Vec::new();

    for potential_password in start_of_range..=end_of_range {
        let password_str = potential_password.to_string();
        if is_valid_password(&password_str) {
            potential_passwords.push(password_str);
        }
    }

    potential_passwords
}

fn is_valid_password(password: &str) -> bool {
    let mut duplicated_digit = false;

    let mut previous_digit = 0;

    for character in password.chars() {
        let digit = character.to_digit(10).unwrap();
        if digit < previous_digit {
            return false
        }
        if digit == previous_digit {
            duplicated_digit = true;
        }
        previous_digit = digit;
    }

    return duplicated_digit
}

fn read_file() -> (u32, u32) {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    let range_vector: Vec<u32> = contents.trim().split("-").map(|code| code.parse::<u32>().unwrap()).collect();

    (range_vector[0], range_vector[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extra_check() {
        assert!(meets_extra_check("111122"));
        assert!(!meets_extra_check("111111"));
        assert!(meets_extra_check("123668"));
    }

    #[test]
    fn test_valid_passwords() {
        assert!(is_valid_password("134566"));
        assert!(is_valid_password("111111"));
        assert!(is_valid_password("144559"));
        assert!(is_valid_password("125559"));
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(!is_valid_password("123456"));
        assert!(!is_valid_password("223450"));
        assert!(!is_valid_password("123789"));
        assert!(!is_valid_password("125551"));
    }
}
