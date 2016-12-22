use self::basic_cat::BasicCat;
use self::numbered_cat::NumberedCat;

mod basic_cat;
mod numbered_cat;

use std::io::Read;

pub struct Settings {
    pub number_only_nonblank : bool,
    pub number_lines : bool,
    pub show_newlines : bool,
    pub squeeze_blanks : bool,
    pub show_tabs : bool,
    pub show_nonprinting : bool,
}

pub struct Cat {
    test : Box<CatMethod>,
}

impl Cat {
    pub fn new(settings : Settings) -> Cat {
        Cat {
            test : {
                if settings.number_lines {
                    Box::new(NumberedCat::new())
                } else {
                    Box::new(BasicCat::new())
                }
            }
        }
    }
    pub fn cat_files(&mut self, file_list : Vec<String>) {
        use std::io;
        use std::fs::File;

        let mut buffer : [u8;512] = [0;512];

        'next_file: for file_name in file_list {
            let mut input : Box<Read> = {
                if file_name == "-" {
                    Box::new(io::stdin())
                }
                else if let Ok(file) = File::open(&file_name) {
                    Box::new(file)
                } else {
                    println!("{}: {}: No such file or directory", super::PROGRAM_NAME, file_name);
                    //if there was no file of that name, continue on to the next item in the list
                    continue 'next_file;
                }
            };
            while let Ok(bytes_read) = input.read(&mut buffer) {
                if bytes_read == 0usize {
                    break;
                }

                self.test.process_buffer(&buffer[0..bytes_read]);
            }
        }
    }
}

trait CatMethod {
    fn process_buffer(&mut self, input_buffer : &[u8]);
}
