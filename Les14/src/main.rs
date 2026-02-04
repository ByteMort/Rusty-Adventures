use std::io::{self, Write};

fn main() {
    // User Input and Output (I/O)

    /*
    println!("Enter a value: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong with input.");

    println!("Your value is: {}", input);
    */

    /*
    print!("Enter a value: ");

    let mut input = String::new();

    io::stdout().flush().unwrap();
    
    /*
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong with input.");
    */

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Value: {}", input);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    */

    /*
    print!("Enter your name: ");

    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong.");

    let input = input.trim();

    println!("Your name is : {}", input);
    */


    loop{
        print!("Enter a number: ");    
        let mut input = String::new();

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Someting went wrong.");

        let input = input.trim();

        match input.parse::<i32>() {
            Ok(v) => {
                println!("Your number is : {}", v);
                break;
            },
            Err(_) => {
                println!("Error : You need to enter a number.");
            }
        }
    }
    
}
