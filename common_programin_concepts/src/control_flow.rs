
fn if_else_control_flow() {
    let number = 3;
    // 1st way
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // 2nd way
    let x = if number < 5 { 5 } else { 6 };  // if Condition TruePart else FalsePart
    println!("The value of x is: {}", x);

    // if is a expression so it can return the value and it can be use in the let

}

fn loop_control_flow() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // loop is a expression so it can return the value and it can be use in the let
    // here two main keyword is used break and continue, break is used to break the loop and continue is used to skip the current iteration and go to the next iteration
}

fn loop_label_control_flow() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
    // here we are using the label to break the outer loop from the inner loop
}

fn while_control_flow() {
    // if we know the condition before the loop then we can use the while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    // while is a expression so it can return the value and it can be use in the let
}

fn looping_through_collection_control_flow_with_while() {
    // if we want to loop through the collection then we can use the while loop with the index of the collection.
    // problem of this approach is that it is not safe because if the index is out of the range of the collection then it will give the error of the index out of range.
    // so we can use the for loop to iterate through the collection.

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
    // while is a expression so it can return the value and it can be use in the let
}

fn lopping_through_collection_control_flow_with_for() {
    // if we want to loop through the collection then we can use the for loop to iterate through the collection.
    // for loop is safe because it is not allow to access the index out of the range of the collection.
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    // for is a expression so it can return the value and it can be use in the let
}



pub fn control_flow() {
    if_else_control_flow();
    loop_control_flow();
    loop_label_control_flow();
    while_control_flow();
    looping_through_collection_control_flow_with_while();
    lopping_through_collection_control_flow_with_for();
}