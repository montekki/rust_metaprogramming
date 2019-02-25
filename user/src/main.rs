extern crate my_macro;
extern crate my_macro_derive;

use my_macro::{HelloMacro, Serialize};
use my_macro_derive::{HelloMacro, Serialize};

#[derive(HelloMacro, Serialize)]
struct Pancakes;

#[derive(HelloMacro, Serialize)]
struct Tuple(u32, u32, String);

fn main() {
    Pancakes::hello_macro();
    let t = Tuple(42, 64, "test string".into());

    dbg!(t.serialize());
}
