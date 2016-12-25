use super::CatMethod;
use super::CatFilter;

#[derive(PartialEq)]
enum NumberingMode {
    None,
    All,
    NonBlank,
}

pub struct LineCat {
    //configuration variables
    mode : NumberingMode,
    squeeze_blanks : bool,
    //state variables
    in_line : bool,
    last_line_blank : bool,
    line_number : usize,
    line_size : usize,
}

impl LineCat {
    pub fn new(number_lines : bool, number_blanks : bool, squeeze_blanks : bool) -> LineCat {
        LineCat {
            mode : {
                if number_lines {
                    if number_blanks {
                        NumberingMode::All
                    } else {
                        NumberingMode::NonBlank
                    }
                } else {
                    NumberingMode::None
                }
            },
            squeeze_blanks : squeeze_blanks,
            in_line : false,
            last_line_blank : false,
            line_number : 1,
            line_size : 0,
        }
    }

    //function assumes that the buffer is either empty or ends in a new line
    fn emit_line(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>) {
        self.emit_partial_line(input_buffer, cat);
        self.in_line = false;
    }

    //function assumes there are 0 or 1 new line characters in the buffer
    fn emit_partial_line(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>) {
        if input_buffer.len() > 0 {
            //if the only character in this buffer is a new line and there hasn't
            //been any previous characters before this
            let is_blank = input_buffer[0] == b'\n' && self.line_size == 0;
            //print line number
            if !self.in_line {
                match self.mode {
                    NumberingMode::None => {},
                    NumberingMode::NonBlank => {
                        if !is_blank {
                            print!("{:6}\t", self.line_number);
                            self.line_number += 1;
                        }
                    },
                    NumberingMode::All => {
                        print!("{:6}\t", self.line_number);
                        self.line_number += 1;
                    },
                }
            }
            //print the line
            if !(self.squeeze_blanks && is_blank && self.last_line_blank) {
                cat.filter_output(input_buffer);
            }
            self.last_line_blank = is_blank;
            self.in_line = true;
        }
    }
}

impl CatMethod for LineCat {
    fn process_buffer(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>) {
        let mut start : usize = 0;
        let mut end : usize = 0;

        while end < input_buffer.len() {

            if input_buffer[end] == b'\n' {
                end += 1;
                self.emit_line(&input_buffer[start..end], cat);
                start = end;
                self.line_size = 0;
            } else {
                end += 1;
                self.line_size += 1;
            }
        }
        //clear out the leftovers in the buffer
        self.emit_partial_line(&input_buffer[start..], cat);
    }
}
