extern crate hexdump;
extern crate my_macro;
extern crate my_macro_derive;

use my_macro::{HelloMacro, Serialize};
use my_macro_derive::{HelloMacro, Serialize};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(Serialize)]
struct Tuple2(u32);

#[derive(HelloMacro, Serialize)]
struct Tuple(u32, u32, String, Tuple2);

fn main() {
    Pancakes::hello_macro();
    let t = Tuple(42, 64, "test string".into(), Tuple2(23));

    hexdump::hexdump(&t.serialize());
}
