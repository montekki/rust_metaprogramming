pub trait HelloMacro {
    fn hello_macro();
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}
