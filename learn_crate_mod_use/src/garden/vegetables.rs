pub mod vegetables{
    #[derive(Debug)]
    pub struct Vegetable {
        pub name: String,
        pub quantity: i32,
    }

    impl Vegetable {
        pub fn new(name: &str, quantity: i32) -> Vegetable {
            Vegetable {
                name: name.to_string(),
                quantity,
            }
        }
    }
}