// Rust is a statically typed language

pub fn data_type() {
    // let mut value = String::new();

    // io::stdin()
    //     .read_line(&mut value)
    //     .expect("Failed to read line");

    // let abc: u32 = value.trim().parse().expect("Please type a number!");

    // println!("You typed: {abc}");

    // Array

    // let array = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Please type a number!");

    // let element = array[index -1];

    // println!("The value of array at index {index} is {}", element);

    // Tuple

    let tuple :(&str, f64, i32) =("Hello", 5.5, 6);

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);
}
