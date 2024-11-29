use std::io;
use regex::Regex;

fn readline(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

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

fn main() {
    loop {
        let input: String = readline("Calculate:");
        if input.is_empty() {
            return;
        }
        
        let re = Regex::new(r"(\d+|\D)").unwrap();
        let parts: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
        
        // Borrow 'parts' to check if the equation contains invalid characters
        if has_invalid_character(&parts) {
            println!("Invalid characters in equation");
            continue;
        }

        // println!("{:?}", parts);
        
        // ["25", "+", "2", "+", "7", "-", "2"]
        // Initialize the result with the first number
        let mut result: f32 = parts[0].parse().unwrap();

        for i in 1..parts.len() {
            let part = parts[i];

            // Is current part is an operator
            if is_operator(part.chars().next().unwrap()) {    
                let operator = part.chars().next().unwrap();

                // Parse the next part as a number (operand)
                let operand = parts[i + 1].parse::<f32>().unwrap();

                // Perform the operation based on the operator
                match operator {
                    '+' => result += operand,
                    '-' => result -= operand,
                    '*' => result *= operand,
                    '/' => result /= operand,
                    _ => (),
                }
            }
        }

        println!("{}", result);
        

    }

}