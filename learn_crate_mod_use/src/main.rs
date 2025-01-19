/**
 * What is a crate?
 * A crate is a package of Rust code. It can be a library or an executable.
 * A crate can be used in multiple projects. It can be published to crates.io.
 * A crate can contain multiple modules.
 *
 * What is a module?
 * A module is a namespace that contains definitions of functions, structs, traits, etc.
 * Modules can be nested. They can be used to organize code.
 * To use a module we need to set that in to orgainze way.
 * A module can be defined in a file or in a block and it can be public or private.
 *
 * What is a path?
 * A path is a way to refer to an item in a module. It can be absolute or relative.Paths are used to refer to functions, structs, traits, etc.
 * Absolute paths start from the crate root.
 * Relative paths start from the current module.
 *
 * What is a use statement?
 * A use statement is used to bring a path into scope. It can be used to refer to an item without using its full path. It can be used to rename an item.
 *
 * What is package in rust?
 * A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file.
 * A package can contain as many binary crates as you like, but at most only one library crate.
 */
extern crate learn_crate_mod_use as ls;
use ls::{eat_at_restaurant, deliver_order,back_house};


fn main() {
    println!("Hello, world!");
    eat_at_restaurant();
    deliver_order();
    back_house();
}

mod garden;


/* There are three way to use a module
    1. Inline, within curly brackets that replace the semicolon following mod garden
    2. In the file src/garden.rs
    3. In the file src/garden/mod.rs
*/




