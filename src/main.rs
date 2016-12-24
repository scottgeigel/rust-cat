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
