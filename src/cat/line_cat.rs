use super::CatMethod;
use super::CatFilter;

pub struct LineCat {
    //configuration variables
    number_blanks : bool,
    squeeze_blanks : bool,
    //state variables
    in_line : bool,
    last_line_blank : bool,
    line_number : usize,
}

impl LineCat {
    pub fn new(number_blanks : bool, squeeze_blanks : bool) -> LineCat {
        LineCat {
            number_blanks : number_blanks,
            squeeze_blanks : squeeze_blanks,
            in_line : false,
            last_line_blank : false,
            line_number : 1,
        }
    }
}

impl CatMethod for LineCat {
    fn process_buffer(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>) {
        let mut start : usize = 0;
        let mut end : usize = 0;

        while end < input_buffer.len() {
            if !self.in_line {
                print!("{:6}\t", self.line_number);
                self.in_line = true;
            }
            if input_buffer[end] == b'\n' {
                end += 1;

                let is_blank : bool = (end - start) == 1;

                if (self.number_blanks || !is_blank) && !(self.squeeze_blanks && is_blank)  {
                    self.in_line = false;
                    self.line_number += 1;
                    cat.filter_output(&input_buffer[start..end]);
                }

                start = end;
                self.last_line_blank = is_blank;
            } else {
                end += 1;
            }
        }
        //clear out the leftovers in the buffer
        cat.filter_output(&input_buffer[start..]);
    }
}
