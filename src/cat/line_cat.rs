use super::CatMethod;
use super::CatFilter;

pub struct LineCat {
    //configuration variables
    number_blanks : bool,
    squeeze_blanks : bool,
    //state variables
    last_line_blank : bool,
    line_buffer : Vec<u8>,
    line_number : usize,
    char_pos : usize,
}

impl LineCat {
    pub fn new(number_blanks : bool, squeeze_blanks : bool) -> LineCat {
        LineCat {
            number_blanks : number_blanks,
            last_line_blank : false,
            squeeze_blanks : squeeze_blanks,
            line_buffer : Vec::new(),
            line_number : 0,
            char_pos : 0,
        }
    }
}

impl CatMethod for LineCat {
    fn process_buffer(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>) {
        self.line_buffer.extend_from_slice(input_buffer);
        while self.char_pos < self.line_buffer.len() {
            if self.line_buffer[self.char_pos] == b'\n' {
                self.char_pos += 1;

                let is_blank : bool = self.char_pos == 1;

                if (self.number_blanks || !is_blank) && !(self.squeeze_blanks && is_blank)  {
                    self.line_number += 1;
                    let (line, _) = self.line_buffer.split_at(self.char_pos);
                    print!("    {}\t", self.line_number);
                    cat.filter_output(line);
                }

                self.line_buffer = self.line_buffer.split_off(self.char_pos);
                self.char_pos = 0;
                self.last_line_blank = is_blank;
            } else {
                self.char_pos += 1;
            }
        }
    }
}
