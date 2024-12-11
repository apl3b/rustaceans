#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: top_left_local,
        bottom_right: bottom_right_local,
    } = rect;
    let width = (top_left_local.x - bottom_right_local.x).abs();
    let height = (top_left_local.y - bottom_right_local.y).abs();
    width * height
}

fn square(point: Point, d: f32) -> Rectangle {
    let bottom_right = Point {
        x: point.x + d,
        y: point.y + d,
    };
    Rectangle {
        top_left: point,
        bottom_right: bottom_right,
    }
}

fn main() {
    let top_left = Point { x: 0.0, y: 0.0 };
    let bottom_right = Point { x: 5.0, y: 2.0 };
    let rect = Rectangle {
        top_left,
        bottom_right,
    };
    let area = rect_area(&rect);
    println!("Rectangle area is {area}");
    let square = square(Point { x: 0.0, y: 0.0 }, 5.0);
    println!("Square is {square:?}");
}
