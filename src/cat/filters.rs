use super::CatFilter;
use super::Settings;

pub struct NoFilter {
}

impl CatFilter for NoFilter {
    fn filter_output(&self, input : &[u8]) {
        use std::io;
        use std::io::Write;
        io::stdout().write(input).unwrap();
    }
}

pub struct TestFilter {
    pub show_tabs : bool,
    pub show_newlines : bool,
    pub show_other : bool,
}

impl CatFilter for TestFilter {
    fn filter_output(&self, input : &[u8]) {
        use std::io;
        use std::io::Write;
        use std::io::BufWriter;

        let mut output = BufWriter::new(io::stdout());

        for i in 0..input.len() {
            let byte = input[i];
            if byte == b'\t'{
                if self.show_tabs {
                    output.write(b"^I").unwrap();
                } else {
                    output.write(b"\t").unwrap();
                }
            } else if byte == b'\n' {
                if self.show_newlines {
                    output.write(b"$\n").unwrap();
                } else {
                    output.write(b"\n").unwrap();
                }
            } else if self.show_other {
                match byte {
                    0...8 | 10...31 => {
                        output.write(&[b'^', byte + 64]).unwrap();
                    },
                    32...126 => {
                        output.write(&[byte]).unwrap();
                    },
                    127 => {
                        output.write(b"^?").unwrap();
                    },
                    128...159 => {
                        output.write(&[b'M', b'-', b'^', byte - 64]).unwrap();
                    },
                    160...254 => {
                        output.write(&[b'M', b'-', byte - 128]).unwrap();
                    },
                    _ => {
                        output.write(b"M-^?").unwrap();
                    },
                }
            } else {
                output.write(&[byte]).unwrap();
            }
        }
    }
}
