pub trait HelloMacro {
    fn hello_macro();
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let mut res = (self.len() as u32).serialize();
        res.extend_from_slice(self.as_bytes());
        res
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Vec<u8> {
        use std::mem::transmute;
        let bytes: [u8; 4] = unsafe { transmute(self.to_be()) };
        bytes.to_vec()
    }
}
