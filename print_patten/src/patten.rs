pub fn patten(){
    println!("from patten function");
    for i in 1..11  {
        for j in 1..i {
            print!("{:?}",j);
        }
        print!("\n")
    }
}