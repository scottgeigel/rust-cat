use super::CatMethod;

pub struct BasicCat {
}

impl BasicCat {
    pub fn new() -> BasicCat {
        BasicCat {
        }
    }

}

impl CatMethod for BasicCat {
    fn process_buffer(&mut self, input_buffer : &[u8]) {
        use std::io;
        use std::io::Write;
        io::stdout().write(input_buffer);
    }
}
