use std::io;

fn readline(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}



fn main() {
    loop {
        let input1: String = readline("First value: ");
        let input2: String = readline("Second value: ");

        if input1.parse::<f32>().is_ok() && input2.parse::<f32>().is_ok() {
            let value: f32 = input1.parse().unwrap();
            let value2: f32 = input2.parse().unwrap();

            let result: f32 = value + value2;

            println!("Result is: {}", result)

        } else {
            println!("Invalid characters in use!")
        }

    }

}