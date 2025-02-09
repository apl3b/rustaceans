use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// impl fmt::Debug for Complex {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imag)
//     }
// }

fn main() {
    let comp = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", comp);
    println!("Debug: {:?}", comp);
}