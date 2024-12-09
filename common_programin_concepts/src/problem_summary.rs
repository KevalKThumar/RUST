fn convert_teprature_to_fahrenheit(celcius: f32) -> f32 {
    (celcius * 9.0 / 5.0) + 32.0
}
fn convert_teprature_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn generate_nth_fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return generate_nth_fibonacci(n - 1) + generate_nth_fibonacci(n - 2);
    }
}

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        // 0 and 1 are not prime numbers so we start from 2
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn fectorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    } else {
        return n * fectorial(n - 1);
    }
}

pub fn problem_summary() {
    let celcius = 36.0;
    let fahrenheit = convert_teprature_to_fahrenheit(celcius);
    println!(
        "The temperature of {} celcius is equal to {} fahrenheit",
        celcius, fahrenheit
    );

    let fahrenheit = 98.6;
    let celcius = convert_teprature_to_celcius(fahrenheit);
    println!(
        "The temperature of {} fahrenheit is equal to {} celcius",
        fahrenheit, celcius
    );

    let n = 10;
    let fibonacci = generate_nth_fibonacci(n);
    println!("The {}th fibonacci number is: {}", n, fibonacci);

    let n = 7;
    let is_prime = is_prime(n);
    println!("Is {} prime? {}", n, is_prime);

    let n = 5;
    let fectorial = fectorial(n);
    println!("The fectorial of {} is: {}", n, fectorial);
}
