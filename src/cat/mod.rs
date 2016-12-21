// pub mod basic_cat;
// pub use self::basic_cat::*;
use std::io::Read;


pub trait Cat {
    fn process_buffer(&self, input_buffer : &[u8]);

    fn cat_files(&self, file_list : Vec<String>) {
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

                self.process_buffer(&buffer);
            }
        }
    }
}

pub struct BasicCat {
}

impl BasicCat {
    pub fn new() -> BasicCat {
        BasicCat {
        }
    }

}

impl Cat for BasicCat {
    fn process_buffer(&self, input_buffer : &[u8]) {
        println!("Hello World");
    }
}
