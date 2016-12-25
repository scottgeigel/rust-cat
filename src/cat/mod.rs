use self::basic_cat::BasicCat;
use self::line_cat::LineCat;
use self::filters::NoFilter;
use self::filters::NonPrintFilter;

mod basic_cat;
mod line_cat;
mod filters;

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
                    Box::new(NonPrintFilter{show_tabs : settings.show_tabs, show_newlines : settings.show_newlines, show_other : settings.show_nonprinting})
                } else {
                    Box::new(NoFilter{})
                }
            }
        }
    }

    pub fn cat_files(&mut self, file_list : Vec<String>) {
        use std::io;
        use std::io::BufRead;
        use std::io::BufReader;
        use std::fs::File;

        'next_file: for file_name in file_list {
            let mut input : Box<BufRead> = {
                if file_name == "-" {
                    Box::new(BufReader::new(io::stdin()))
                }
                else if let Ok(file) = File::open(&file_name) {
                    Box::new(BufReader::new(file))
                } else {
                    println!("{}: {}: No such file or directory", super::PROGRAM_NAME, file_name);
                    //if there was no file of that name, continue on to the next item in the list
                    continue 'next_file;
                }
            };
            loop {
                let bytes_read = {
                    let buffer = input.fill_buf().unwrap();
                    if buffer.len() == 0usize {
                        break;
                    }
                    self.test.process_buffer(buffer, &self.filter);
                    buffer.len()
                };

                input.consume(bytes_read);
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
