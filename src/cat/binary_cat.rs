use super::CatMethod;

pub struct BinaryCat {
}

impl BinaryCat {
    pub fn new() -> BinaryCat {
        BinaryCat {
        }
    }

}

impl CatMethod for BinaryCat {
    fn process_buffer(&mut self, input_buffer : &[u8]) {
        println!("Hello World");
    }
}
