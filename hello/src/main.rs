

fn main() {
    /*!SECTION
     * 
     println!("Hello, world!");  
     // how to use String in Rust
     let mut s = String::from("hello");
     println!("{s}");
     let s1 = String::from("world!!");
     println!("{s1}");
     s.push_str(&s1);
     println!("with concetnation: {s}");
     println!("with push_str:{s}");
     s= s+&s1;
     */
     
    // String slice in rust
    /*!SECTION
     * 
     let s = String::from("hello world");
     let hello = &s[0..1];  //0 is inclusive and 1 is exclusive
    let world = &s[6..11];
    println!("{hello} {world}");
 */

 let s = String::from("hello");
 
    // for loop
    for c in s.chars(){
        println!("{c}");
    }
    for b in s.bytes(){
        println!("{b}");
    }
    

    // what is u8 in rust ? :-> unsigned 8 bit integer it give ASCII value of the character
    // let x = b'1';
    // println!("{x}");

}
