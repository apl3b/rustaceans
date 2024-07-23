use std::fmt;

#[derive(Debug)]
struct RGB(u8,u8,u8);

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:0>2X}{:0>2X}{:0>2X}", self.0, self.1, self.2)
    }
}

fn main() {
    for rgb in [
        RGB(128, 255, 90),
        RGB(0, 3, 254),
        RGB(0, 0, 0),
    ] {
        println!("{}", rgb);
    }
}