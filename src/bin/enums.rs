use std::io;

fn main() {
    // Prompt the user to input the first number
    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");

    // Parse the input into a floating-point number
    let first_number: f64 = match first_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Prompt the user to input the operation
    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    // Prompt the user to input the second number
    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");

    // Parse the input into a floating-point number
    let second_number: f64 = match second_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Perform the specified operation and print the result
    let result = match operation.trim() {
        "+" => calculate(Operation::Add { x: first_number, y: second_number }),
        "-" => calculate(Operation::Subtract { x: first_number, y: second_number }),
        "*" => calculate(Operation::Multiply { x: first_number, y: second_number }),
        "/" => calculate(Operation::Divide { x: first_number, y: second_number }),
        _ => {
            println!("Invalid operation. Please enter a valid operation (+, -, *, /).");
            return;
        }
    };

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => panic!("{:?}", err),
    }
    
}

// Create Operation Enum
enum Operation {
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 },
}

#[derive(Debug)]
pub enum MathError {
    DivisionByZero
}

/**
 * calculate is used to operate each given operation
 * @param operation: Operation enum to specified which opetaion to be used for calculate
 * @return result: f64 value
 */
fn calculate(opr: Operation) -> Result<f64, MathError> {
    match opr {
        Operation::Add {x,y} => {
            Ok(x + y)
        },
        Operation::Subtract {x,y} => {
            Ok(x - y)
        },
        Operation::Multiply {x,y} => {
            Ok(x * y)
        },
        Operation::Divide {x,y} => {
            if y == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(x / y)
            }
        }
    }
}