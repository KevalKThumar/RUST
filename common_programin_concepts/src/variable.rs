pub fn variable() {
    /* let
    *
        // let x = 5; //cannot assign twice to immutable variable `x`
        let mut x = 5;
        println!("The value of x is: {}", x);
        x=6;
        println!("The Value after change is:: {}",x)
    */

    /*!const
    *
        // const MAX_POINTS: u32 = 4294967296; //error due to overflow range of u32 type is 0 to 4294967295, full name is unsigned 32 bit integer
         const MAX_POINTS: u32 = 4294967295; // it can not change after decreting it is by default immuttable
        println!("The value of MAX_POINTS is: {}", MAX_POINTS);
     */

    // Shadowing

    // let mut x = 5;
    // x = x + 1;
    // println!("The value of x before shadowing is: {}", x);
    // {
    //     // if  we declear here new variable then it scop in over afer the "}" and we can also use out of the scope variable.
    //      x = x * 2;
    //     println!("The value of x in shadowing is: {}", x);
    // }
    // println!("The value of x after shadowing is: {}", x)

    let spaces = "    ";
    let spaces = spaces.len();  // it gives an error because len() it is use in String type variable and we can not use it in integer
    println!("The value of spaces is: {}", spaces);

    // SO NOW WE NEED TO UNDERSTAND DATATYPE 
}
