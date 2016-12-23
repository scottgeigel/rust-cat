use super::CatMethod;
use super::CatFilter;
use super:: Cat;

pub struct BasicCat {
}

impl BasicCat {
    pub fn new() -> BasicCat {
        BasicCat {
        }
    }

}

impl CatMethod for BasicCat {
    fn process_buffer(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>) {
        use std::io;
        use std::io::Write;
        cat.filter_output(input_buffer);
    }
}
