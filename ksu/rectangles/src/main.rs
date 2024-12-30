#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 15, 
        height: 20
    };

    dbg!("rect1 is {:#?}", &rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("The rectangle height is bigger than 0? {}", rect1.width());

    if rect1.can_hold(&rect2) {
        println!("First rectangle can hold the second.");
    } else {
        println!("First rectangle cannot hold the second.");
    }

    let square = Rectangle::square(4);
    println!("Square height is bigger thant 0? {}", square.height());
}
