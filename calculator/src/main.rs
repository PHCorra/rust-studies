use std::io::{self, Write};

fn main() {
    println!("Welcome to calculator!\n");

    println!("1. Sum");
    println!("2. Minus");
    println!("3. Multiplication");
    println!("4. Divison\n");

    print!("Please choose an operation: ");

    let user_operation_choice = loop {
        match handle_input() {
            Ok(num) => break num,
            Err(err) => println!("Error: {}. Try Again.", err),
        }
    };

    match  user_operation_choice {
        1 => {
            print!("Type the first operator: ");
            let user_first_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            print!("Type the second operator: ");
            let user_second_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            let result = user_first_operator + user_second_operator;

            return println!("The result is: {}", result);
        }
        2 => {
            print!("Type the first operator: ");
            let user_first_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            print!("Type the second operator: ");
            let user_second_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            let result = user_first_operator - user_second_operator;

            return println!("The result is: {}", result);
        }
        3 => {
            print!("Type the first operator: ");
            let user_first_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            print!("Type the second operator: ");
            let user_second_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            let result = user_first_operator * user_second_operator;

            return println!("The result is: {}", result);
        }
        4 => {
            print!("Type the first operator: ");
            let user_first_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            print!("Type the second operator: ");
            let user_second_operator = loop {
                match handle_input() {
                    Ok(num) => break num,
                    Err(err) => println!("Error: {}. Try Again.", err),
                }
            };

            let result = user_first_operator / user_second_operator;

            return println!("The result is: {}", result);
        }
        _ => {
            println!("Invalid Operation!");
            return;
        }
    }
}

fn handle_input() -> Result<i32, String>{
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let trimmed = input.trim();

    match trimmed.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input: Expected a number.".to_string()),
    }

}
