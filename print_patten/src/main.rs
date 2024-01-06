mod function;
mod patten;
use patten::patten;
use function::copy;
fn main() {
//    println!("Hello, world!"); 
   copy("hello".to_string());
   patten()
}


/* If we want to send the data from one function to another function we use reference variable because we don't want to copy the data. */

// fn main() {
// let  s = String::from("hello");  // scop of the s variable is in the main function and not in the change_word function.we pass only the reference of the variable
//     change_word(&s);
//     change_word(&s);
// }
// fn change_word(s: &str)
// {
//     println!("Before Change word:-{}",s);
// }
/*
fn main() {
    let  s = String::from("hello"); // in this case we don't pass the reference of the variable to the change_word function so that after the call of the function scop of the s variable is not in main function
    change_word(s);
    // print!("After Change word:-{}",s);
}
fn change_word(s: String)
{
    println!("Before Change word:-{}",s);
}

 */

