use std::io; //to obtain user input and print result
use rand::Rng; //io and Rng are 'traits' in Rust parlance
use std::cmp::Ordering; //Ordering type is an enum with variants Less, Greater, and Equal


//traits define methods that std and rand implement.

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    
    loop {
        println!("Please input your guess.");
        
        //new() is an associated function of the String type
        let mut guess = String::new();
    
        //read_line returns a Result (enum) value. If Result
        //is Err, the message provided to expect() will print.
        //You can skip .expect() but you'll get a warning
        io::stdin()
            .read_line(&mut guess) //references are immutable by default and &mut guess makes it mutable 
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        
        //convert guess from a string to a u32 so it can be compared to a number later
        //this is an example of shadowing and is often used in this situation
        //using ':' tells Rust that a data type is next
        //parse() returns a Result type which is an enum with
        //variants Ok and Err. Use match to distinguish btw them.
        //If parse is able to successfully turn the string into 
        //a number, it will return an Ok value that contains the
        //resultant number.
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        //a match expression is made up of 'arms' 
        //strings (guess) can't be compared to numbers (secret_number, i32 by default)
        match guess.cmp(&secret_number) { //compare guess and secret_number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
    println!("The secret number is: {secret_number}");
}
