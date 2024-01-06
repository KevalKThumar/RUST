pub fn patten(){
    println!("from patten function");
    for i in 1..10  {
        for j in 1..i {
            print!("{:?}",j);
        }
        print!("\n")
    }
}