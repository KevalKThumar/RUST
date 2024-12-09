use std::io;
// rand = "0.8.5" -> this is for the random number generation in rust
// version system is MAJOR.MINOR.PATCH
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter the Guess Number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // here meaning of the &mut meance & is for refrence and mut is for the mutable and both combine is refrence of the mutable referance.It meance that It has permission of changing and appendade guess variable.
        // Here result is which io return it is type of enum which has two value, ok and error If there is ok, then it's right. If there is an error part, then it goes into the expect part and print the message.
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };
        // here we are using shadowing to create a new variable called guess with the type u32 and the value from the user input
        // trim() is important because, when the user types in a number and then presses Enter, they might have added an extra newline character that we need to get rid of.
        // parse() is a method that can be used to parse a string into some kind of number. We’re using it here to parse the string into a number. But we need to tell Rust what kind of number we expect, so we add the annotation : u32 to the end of the parse call. This tells Rust that we want parse to convert the string into a number. If parse can’t convert the string into a number, it will panic!.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        // match handales all the possible value of the enum and it is the best way to handle the enum value.
    }
}
