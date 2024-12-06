use std::io;
use regex::Regex;

// Read a line from the console
// Copied over from the CLI notes app
fn readline(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

// Check if character is one of 4 operators
// Possibly add parantheses support in the future
fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

// Check if equation contains invalid characters
fn has_invalid_character(equation_parts: &Vec<&str>) -> bool {
    for part in equation_parts {
        if part.parse::<f32>().is_err() {
            if !is_operator(part.chars().next().unwrap()) {
                return true;
            }
        }
    }
    return false;
}

/*  Calculate the equation
    First we handle multiplication and division
    Then we handle addition and subtraction
    This is done to follow the order of operations
*/
fn calculate_equation(parts: &Vec<&str>) -> f32 {
    // Step 1: Handle multiplication and division first
    let mut intermediate_parts = Vec::new();
    let mut i = 0;

    while i < parts.len() {
        let part = parts[i];

        if is_operator(part.chars().next().unwrap()) && (part == "*" || part == "/") {

            // Removes the last element from a vector and returns it
            let prev: String = intermediate_parts.pop().unwrap();

            // Parse the operands and perform the operation
            let operand1 = prev.parse::<f32>().unwrap();
            let operand2 = parts[i + 1].parse::<f32>().unwrap();

            // Perform the operation
            let result = match part {
                "*" => operand1 * operand2,
                "/" => operand1 / operand2,
                _ => panic!("Unexpected operator"),
            };

            // Push result back into the list
            intermediate_parts.push(result.to_string());
            i += 2; // Skip the operator and next operand
        } else {
            intermediate_parts.push(part.to_string());
            i += 1;
        }
    }

    // Step 2: Handle addition and subtraction
    let mut result = intermediate_parts[0].parse::<f32>().unwrap();
    for i in (1..intermediate_parts.len()).step_by(2) {
        let operator = intermediate_parts[i].chars().next().unwrap();
        let operand = intermediate_parts[i + 1].parse::<f32>().unwrap();

        match operator {
            '+' => result += operand,
            '-' => result -= operand,
            _ => panic!("Unexpected operator"),
        }
    }

    result
}


fn main() {
    let re = Regex::new(r"\d+(\.\d+)?|[+\-*/]").unwrap();


    loop {
        let input: String = readline("Calculate:");
        if input.is_empty() {
            return;
        }
        
        let parts: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
        
        // Borrow 'parts' to check if equation contains invalid characters
        if has_invalid_character(&parts) {
            println!("Invalid characters in equation");
            continue;
        }

        let result = calculate_equation(&parts);
        println!("{}", result);
    }
}

// Tests used for mostly for 'calculate_equation' but also for 'has_invalid_character'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let parts = vec!["2", "+", "3"];
        assert_eq!(calculate_equation(&parts), 5.0);
    }

    #[test]
    fn test_subtraction() {
        let parts = vec!["5", "-", "3"];
        assert_eq!(calculate_equation(&parts), 2.0);
    }

    #[test]
    fn test_multiplication() {
        let parts = vec!["4", "*", "2"];
        assert_eq!(calculate_equation(&parts), 8.0);
    }

    #[test]
    fn test_division() {
        let parts = vec!["8", "/", "2"];
        assert_eq!(calculate_equation(&parts), 4.0);
    }

    #[test]
    fn test_combined_operations() {
        let parts = vec!["2", "*", "3", "+", "4"];
        assert_eq!(calculate_equation(&parts), 10.0);
    }

    #[test]
    fn test_invalid_character() {
        let parts = vec!["2", "+", "if this went through, it would be invalid unless I messed soemthing up"];
        assert!(has_invalid_character(&parts));
    }

    #[test]
    fn test_valid_character() {
        let parts = vec!["2", "+", "3"];
        assert!(!has_invalid_character(&parts));
    }
}

