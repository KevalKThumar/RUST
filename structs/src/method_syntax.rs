// there is difference between method and function in rust.
/**
 * Unlike functions, methods are defined within a struct and are associated with a specific type.
 * Method are difine with in the context of struct or enum or trait Object
 * and their first Parameter is always self
 */

#[derive(Debug)]
struct Rectangle {
    length: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.height
    }
    // here self is the reference to the struct like "this" in other programing language

    // associated function
    fn square(size: u32) -> Self {
        Self {
            length: size,
            height: size,
        }
    }
}

pub fn method_syntax() {
    let rect = Rectangle {
        length: 10,
        height: 20,
    };
    println!("The area of Rectangle {:#?} is {}.", rect, rect.area());
    
    let ract2 = Rectangle {
        length: 20,
        height: 30,
    };
    println!("The area of Rectangle {:#?} is {}.", ract2, ract2.area());


    // to call associated function we don't need to create instance of struct we can call it directly with struct name like below it is like static method in other programming language
    let squre = Rectangle::square(10);
    println!("The area of squre {:#?} is {}.", squre, squre.area());
}
