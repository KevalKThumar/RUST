fn main() {
    let mul: i32 = multiply(3, 2);
    println!("{}", mul);
    let div: i32 = divide(3, 2);
    println!("{}", div);
    let sub: i32 = subtract(3, 2);
    println!("{}", sub);
    let add: i32 = addnumbers(3, 2);
    println!("{}", add);
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn addnumbers(a: i32, b: i32) -> i32 {
    a + b
}
