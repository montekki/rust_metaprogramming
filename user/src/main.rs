extern crate my_macro;
extern crate my_macro_derive;

use my_macro::HelloMacro;
use my_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
