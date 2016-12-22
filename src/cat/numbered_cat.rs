use super::CatMethod;

pub struct NumberedCat {
    line_number : usize,
}

impl NumberedCat {
    pub fn new() -> NumberedCat {
        NumberedCat {
            line_number : 0,
        }
    }
}

impl CatMethod for NumberedCat {
    fn process_buffer(&mut self, input_buffer : &[u8]) {
        self.line_number += 1;
        println!("{}", self.line_number);
    }
}
