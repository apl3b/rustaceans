use std::fmt;

#[derive(Debug)]
struct RGB(u8,u8,u8);

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}", red=self.0, green=self.1, blue=self.2)
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