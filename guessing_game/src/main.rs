use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);    // this is for generating a random number between 1 and 100

    loop {   // this is a loop that will keep running until the user guesses the correct number or we break out of the loop
        println!("Please input your guess with in 0 to 100.");

        let mut guess = String::new();  // declaring a new variable called guess with the type String

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");  // taking user input and storing it in guess variable

             // here we are using shadowing to create a new variable called guess with the type u32 and the value from the user input

        let guess: u32 = match guess.trim().parse() {   
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };  // this is for converting the user input to u32 and storing it in guess variable if num then continue if not then print error and continue

        println!("You guessed: {guess}");

       match guess.cmp(&secret_number) { // this is for checking with the secret number and comparing it with the guess
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal => {
               println!("You win!");
               break;
           }
       } // this is for checking if the guess is less than the secret number, greater than the secret number or equal to the secret number An Ordering is the result of a comparison between two values. It can be one of three values: Less, Greater, or Equal.
    }
}