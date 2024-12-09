fn add(x: i32, y: i32) -> i32 {
    // 1st way to define the function
    // x + y    if you don't use semicolon then it will return the value and it's called expression in rust
    // 2nd way to define the function
    let sum = x + y;
    sum
    // only expression can be return the value not the statement
}

// function call is an expression so it can return the value
// function definition is a statement so it can not return the value
// when i want to early return the value then i can use return keyword

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn div(x: f32, y: f32) -> f32 {
    x / y
}

fn rem(x: i32, y: i32) -> i32 {
    x % y
}
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

pub fn calclulator(x: f32, y: f32) {
    println!("Is {} even? {}", x as i32, is_even(x as i32));
    println!("Is {} even? {}", y as i32, is_even(y as i32));
    println!(
        "Addition of {} and {} is: {}",
        x,
        y,
        add(x as i32, y as i32)
    );
    println!(
        "Multiplication of {} and {} is: {}",
        x,
        y,
        mul(x as i32, y as i32)
    );
    println!(
        "Subtraction of {} and {} is: {}",
        x,
        y,
        sub(x as i32, y as i32)
    );
    println!("Division of {} and {} is: {}", x, y, div(x, y));
    println!(
        "Remainder of {} and {} is: {}",
        x,
        y,
        rem(x as i32, y as i32)
    );
}
