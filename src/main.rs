extern crate getopts;
use getopts::Options;
use std::env;

mod cat;

const PROGRAM_NAME : &'static str = env!("CARGO_PKG_NAME");
const PROGRAM_VERSION : &'static str = env!("CARGO_PKG_VERSION");
const BRIEF : &'static str = "Usage [OPTION]... [FILE]...";

struct Settings {
    display_help : bool,
    display_version : bool,
    file_list : Vec<String>,
    cat_settings : cat::Settings,
}

fn print_usage(opts : Options) {
    println!("{}", opts.usage(BRIEF));
}

fn print_version() {
    println!("{}: {}", PROGRAM_NAME, PROGRAM_VERSION);
}

fn process_arguments(opts : &Options) -> Settings {
    let args : Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(s) => {
                println!("{}\nTry '{} --help' for more information", s, PROGRAM_NAME);
                std::process::exit(1);
            },
    };

    let mut result = Settings {
        display_help : matches.opt_present("help"),
        display_version : matches.opt_present("version"),
        cat_settings : cat::Settings {
            number_only_nonblank : false,
            show_newlines : false,
            number_lines : false,
            squeeze_blanks : false,
            show_tabs : false,
            show_nonprinting : false,
        },
        file_list : vec![String::from("-")],
    };

    if matches.opt_present("A") {
        result.cat_settings.show_nonprinting = true;
        result.cat_settings.show_newlines = true;
        result.cat_settings.show_tabs = true;
    }

    if matches.opt_present("b") {
        result.cat_settings.number_lines = true;
        result.cat_settings.number_only_nonblank = true;
    }

    if matches.opt_present("e") {
        result.cat_settings.show_nonprinting = true;
        result.cat_settings.show_newlines = true;
    }

    if matches.opt_present("E") {
        result.cat_settings.show_newlines = true;
    }

    if matches.opt_present("n") {
        result.cat_settings.number_lines = true;
    }

    if matches.opt_present("s") {
        result.cat_settings.squeeze_blanks = true;
    }

    if matches.opt_present("t") {
        result.cat_settings.show_nonprinting = true;
        result.cat_settings.show_tabs = true;
    }

    if matches.opt_present("T") {
        result.cat_settings.show_tabs = true;
    }

    if matches.opt_present("v") {
        result.cat_settings.show_nonprinting = true;
    }

    if matches.free.len() > 0 {
        result.file_list = matches.free;
    }

    return result;
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("A", "show-all", "equivalent to -vET");
    opts.optflag("b", "number-nonblank", "number nonempty output lines, overrides -n");
    opts.optflag("e", "", "equivalent to -vE");
    opts.optflag("E", "show-ends", "displays $ at the end of each line");
    opts.optflag("n", "number", "number all output lines");
    opts.optflag("s", "squeeze-blank", "suppress repeated empty output lines");
    opts.optflag("t", "", "equivalent to -vT");
    opts.optflag("T", "show-tabs", "display TAB characters as ^I");
    opts.optflag("u", "", "(ignored)");
    opts.optflag("v", "show-nonprinting", "use ^ and M- notation, except for LFD and TAB");
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");

    let settings = process_arguments(&opts);

    if settings.display_help {
        print_usage(opts);
    } else if settings.display_version {
        print_version();
    } else {
        let mut cat = cat::Cat::new(settings.cat_settings);
        cat.cat_files(settings.file_list);
    }
}

/*
fn cat(settings : Settings) {
    use std::fs::File;
    use std::io;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::io::Read;
    let mut line_count :usize = 0;
    let mut last_line_blank = false;
    let mut buffer : [u8; 512] = [0;512];
    let scan_chars = settings.cat_settings.show_nonprinting || settings.cat_settings.show_tabs;
    let mut bytes_read : usize;
    //main file loop
    'next_file: for file_name in settings.file_list {
        let mut input : Box<Read> = {
            if file_name == "-" {
                Box::new(std::io::stdin())
            }
            else if let Ok(file) = File::open(&file_name) {
                Box::new(file)
            } else {
                println!("{}: {}: No such file or directory", PROGRAM_NAME, file_name);
                //if there was no file of that name, continue on to the next item in the list
                continue 'next_file;
            }
        };

        while let Ok(bytes_read) = input.read(&mut buffer) {
            if bytes_read == 0usize {
                break;
            }

            let line : Vec<u8> = buffer.to_vec();

            //pre-print rules
            if last_line_blank && line.len() > 0 {
                continue;
            }

            if (settings.cat_settings.number_lines && !settings.number_only_nonblank)
            || (settings.number_only_nonblank && line.len() > 0) {
                line_count += 1;
                print!("    {}\t", line_count);
            }
            //print rules
            if scan_chars {
                for character in line.chars() {
                    const TAB_U8 : u8 = '\t' as u8;
                    match character as u8 {
                        TAB_U8 => {
                            if settings.cat_settings.show_tabs {
                                print!("^I");
                            } else {
                                print!("\t");
                            }
                        },
                        0...8 | 10...31 => print!("^{}", (character as u8 + 64u8) as char),
                        127 | 255 => print!("^?"),
                        128...159 => print!("M-^{}", (character as u8 - 64u8) as char),
                        160...254 => print!("M-{}", (character as u8 - 128u8) as char),
                        _ => print!("{}", character),
                    }
                }
            } else {
                print!("{}", line);
            }

            if settings.cat_settings.show_newlines {
                println!("$");
            } else {
                println!("");
            }
            //post-print rules
            last_line_blank = line.len() == 0;
        }
    }
}
*/
