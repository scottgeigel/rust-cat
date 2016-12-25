use self::basic_cat::BasicCat;
use self::line_cat::LineCat;

mod basic_cat;
mod line_cat;

use std::io::Read;

pub struct Settings {
    //line numbering flags
    pub number_only_nonblank : bool,
    pub number_lines : bool,
    //newline action flags
    pub squeeze_blanks : bool,
    pub show_newlines : bool,
    //nonprintable flags
    pub show_tabs : bool,
    pub show_nonprinting : bool,
}

struct NoFilter {
}

impl CatFilter for NoFilter {
    fn filter_output(&self, input : &[u8]) {
        use std::io;
        use std::io::Write;
        io::stdout().write(input).unwrap();
    }
}

struct TestFilter {
    show_tabs : bool,
    show_newlines : bool,
    show_other : bool,
}

impl CatFilter for TestFilter {
    fn filter_output(&self, input : &[u8]) {
        use std::io;
        use std::io::Write;

        for i in 0..input.len() {
            let byte = input[i];
            if byte == b'\t'{
                if self.show_tabs {
                    io::stdout().write(b"^I").unwrap();
                } else {
                    io::stdout().write(b"\t").unwrap();
                }
            } else if byte == b'\n' {
                if self.show_newlines {
                    io::stdout().write(b"$\n").unwrap();
                } else {
                    io::stdout().write(b"\n").unwrap();
                }
            } else if self.show_other {
                match byte {
                    0...8 | 10...31 => {
                        io::stdout().write(&[b'^', byte + 64]).unwrap();
                    },
                    32...126 => {
                        io::stdout().write(&[byte]).unwrap();
                    },
                    127 => {
                        io::stdout().write(b"^?").unwrap();
                    },
                    128...159 => {
                        io::stdout().write(&[b'M', b'-', b'^', byte - 64]).unwrap();
                    },
                    160...254 => {
                        io::stdout().write(&[b'M', b'-', byte - 128]).unwrap();
                    },
                    _ => {
                        io::stdout().write(b"M-^?").unwrap();
                    },
                }
            } else {
                io::stdout().write(&[byte]).unwrap();
            }
        }
    }
}

pub struct Cat {
    test : Box<CatMethod>,
    filter : Box<CatFilter>,
}

impl Cat {
    pub fn new(settings : Settings) -> Cat {
        Cat {
            test : {
                if settings.number_lines || settings.squeeze_blanks {
                    Box::new(LineCat::new(settings.number_lines, !settings.number_only_nonblank, settings.squeeze_blanks))
                } else {
                    Box::new(BasicCat::new())
                }
            },
            filter : {
                if settings.show_tabs || settings.show_newlines || settings.show_nonprinting {
                    Box::new(TestFilter{show_tabs : settings.show_tabs, show_newlines : settings.show_newlines, show_other : settings.show_nonprinting})
                } else {
                    Box::new(NoFilter{})
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

                self.test.process_buffer(&buffer[0..bytes_read], &self.filter);
            }
        }
    }
}

trait CatFilter {
    fn filter_output(&self, input : &[u8]);
}

trait CatMethod {
    fn process_buffer(&mut self, input_buffer : &[u8], cat : &Box<CatFilter>);
}
