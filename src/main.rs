//input output library from std(standard library)
use std::io;
//rand random number generator crate
use rand::Rng;
//another enum with variants, Less, Greater, and Equal
//cmp() = comparison operator
use std::cmp::Ordering;
use colored::*;

//Vars and Refs are immutable by default

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        //create mutable variable "guess" with base of type string
        let mut guess = String::new();

        //io = input output
        //could've also used std::io::Stdin = type that represents a handle ot the standar input for your terminal
        io::stdin()
        //read_line store input into param("guess"), & indicates a reference which allows multiple parts to accesss one piece of data without using memory space
        .read_line(&mut guess)
        //io::results variant "expect()", result can either be OK or ERR
        //must add this in as an error check
        .expect("Failed to read line");

        //shadow's previous value of guess, normally used to convert from type to type
        //define u32 type, trim() gets rid of any whitespace or \n\r, parse() creates a number (defined by u32) from a string or returns a result type "Ok" || "Err"... but is very easy to get errors, so expect() or some error check must be there
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            //"_" is a catch-all, anything else
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        //match expression, takes in any value and compares with it's "arms."
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!!!".green());
                break;
            },
        }        
    }

}
