// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// mod example_struct;
// use example_struct::main_example;

mod method_syntax;
use method_syntax::method_syntax;
// struct Color(i32, i32, i32);
//     struct Location(i32, i32, i32);
fn main() {
    // main_example();
    method_syntax();
    // let user1 = User {
    //     email: String::from("thumarkeval12@gmail.com"),
    //     username: String::from("keval"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    // println!(
    //     "User1 email: {} \nUser1 username: {}",
    //     user1.username, user1.email
    // );

    // let email = user1.email;
    // println!("User1 email: {}", user1.email); // This will give error because email is moved to email variable

    // let updated_user = User {
    //     email: String::from("thumarkeval2003@gmail.com"),
    //     username: String::from("keval2003"),
    //     ..user1
    // };

    // println!(
    //     "updated_user email: {} \nupdated_user username: {}",
    //     updated_user.username, updated_user.email
    // );

    // let user2 = build_user(
    //     String::from("thumarkeval2003@gmail.com"),
    //     String::from("keval2003"),
    // );

    // println!(
    //     "User2 email: {} \nUser2 username: {}",
    //     user2.username, user2.email
    // );

    // // struct tuple
    // /*
    //    let red = (255, 0, 0);
    //    let current_location = (0, 0, 0);
    //    // this is right
    //    set_color(red);
    //    set_location(current_location);
    //    // but what if we pass wrong value accroding to rust it is right but it is wrong as per usecase
    //    set_color(current_location);
    //    set_location(red);
    // */

    // // to solve this problem we use struct tuple
    // let red = Color(255, 0, 0);
    // let current_location = Location(0, 0, 0);

    // set_color(red);
    // set_location(current_location);

    // we can not make mistake in this case

    // set_location(red);
    
}
/*
fn set_color(color: Color) {
    println!("red: {}, green: {}, blue: {}", color.0, color.1, color.2);
}

fn set_location(location: Location) {
    println!("x: {}, y: {}, z: {}", location.0, location.1, location.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
 */