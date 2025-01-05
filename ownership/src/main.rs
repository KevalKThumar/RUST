/**
 *  In this example, we will see how to use ownership in Rust.
 *
 *  what is ownership?
 *  - Rust's memory safety is achieved through a system of ownership with a set of rules that the compiler checks at compile time.
 *  - Ownership is a feature unique to Rust. It allows Rust to make memory safety guarantees without needing a garbage collector.
 *  - Ownership is a central feature of Rust, and it is a feature that has a big impact on how you write Rust code.
 *
 *  - Three types of memory management:
 *    - Garbage collection --> Java, Python, Ruby, JavaScript
 *    - Explicitly allocating and freeing memory --> C, C++
 *    - Ownership -> Rust
 *
 *  - In the ownership system we will focus on a very common data structure is String.
 *
 *  How we can know that data will store in stack or heap?
 *  - If we know the size of data at compile time, then it will store in stack.
 *  - If we don't know the size of data at compile time, then it will store in heap.
 *  - In Rust, we can't know the size of data at compile time, so it will store in heap.
 *
 *  Accessing data in stack is faster than accessing data in heap. Because in stack it will store in sequence and we can access it easily. but in heap it will store in random memory location. So, it will take time to access data. and it follow the pointer to access the data which is stored in stack.
 *
 * Ownership Rules:
 * - Each value in Rust has a variable that’s called its owner.
 * - There can only be one owner at a time.
 * - When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    // data_intrecting_with_change();
    // data_intrecting_with_move();
    // data_intrecting_with_passing_function();
    //---------------------------------------------------------------- borrow
    // let s = String::from("Keval K Thumer");
    // let length = calculate_length_with_borrow(&s);
    // println!("The length of '{}' is {}.", s, length);
    // --------------------------------------------------------------- ownership
    // let s = String::from("Keval K Thumer");
    // let (s, length) = calculate_length(s);
    // println!("The length of '{}' is {}.", s, length);
    // --------------------------------------------------------------- mutable reference
    let mut s = String::from("Keval K Thumer");
    s.push_str(", How are you?");
    let s1 = change(&mut s);
    println!("{}", s1);
    /*    TODO: Aim::write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    */

    // --------------------------------------------------------------- normal
    let mut s = String::from("Keval K Thumar");
    let res_noraml = first_word_normal(&s);
    // in this is bymistake devloper cleare the value of s. So, it will give wrong result.
    s.clear();
    println!("The length of first word of {s} is: {}", res_noraml);

    // --------------------------------------------------------------- slice
    let res = first_word_slice(&s);
    println!(
        "The first word of {s} is: {} And length is: {}",
        res,
        res.len()
    );
    // it is very Tidious to work to clone the value of s to s1. So, we can use reference to solve this problem.
}

// first_word function we can do this by using slice or normal way

// slice way
// you can also user &str instead of &String to make it more general so that it can accept both heap and stack string.
// fn first_word_slice(s: &String) -> &str {
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn change(s: &mut String) -> &mut String {
    s.push_str(", world");
    s
}
// normal way
fn first_word_normal(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

/*
 *  In other Programming language who has garbage collector that will automatically free memory when it is no longer needed And in other end, if there is no garbage collector then we have to free Memory manually. In the rust programming language There is no garbage collector allthough we don't have to free memory manually because rust will automatically free memory when variable goes out of scope. That's known as ownership
 */

/*
fn data_intrecting_with_change() {
    // string variable in stack
    let s = "Hello, World!"; // string literal  --> it will store in stack we can't change the value of s. or we can't modify the value of s.
    println!("Come from Stack memory::-- {}", s);
    // string variable in heap
    let mut s = String::from("Hello, World!"); // string object --> it will store in heap. we can change the value of s. or we can modify the value of s.
    s.push_str(" How are you?"); // push_str() method appends a literal to a String
    println!("Come from Heap memory::-- {}", s);
}

fn data_intrecting_with_move() {
    // let variable in stack
    let mut x = 5;
    let y = x; // copy the value of x to y  --> because integer is stored in stack
    x = 10;
    println!("x: {}, y: {}", x, y);

    // String variable in heap
    let s1 = String::from("Hello");
    let s2 = s1; // move the value of s1 to s2
                 // println!("s1: {}", s1); // it will give error because s1 is moved to s2 so s1 is no longer valid.
    println!("s2: {}", s2);

    // As per rule of ownership that one variable can have only one owner at a time. So, if we want to copy the value of s1 to s2 then we have to clone the value of s1 to s2.

    // if we want do this then we have to use clone() method.
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // explicitly clone the value of s1 to s2 that is expensive operation
    println!("s1: {}, s2: {}", s1, s2);

    /* in heap it make move opretion and in stack it make copy*/
}

fn data_intrecting_with_passing_function() {
    let s = String::from("Hello");
    takes_ownership(s); // s value will move to function
    // println!("s: {}", s); // it will give error because s is moved to function
    let x = 5;
    makes_copy(x); // x value will copy to function
    println!("x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

*/
// give the ownership of s to function
/*
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/

// example of borrowing
/*
fn calculate_length_with_borrow(s: &String) -> usize {
    s.len()
}

// example of mutable reference
fn change(s: &mut String) -> &mut String {
    s.push_str(", world");
    s
}
 */
