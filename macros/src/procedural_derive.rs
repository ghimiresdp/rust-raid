use shape::Rect; // this trait is used by the macro Rect
use shape_derive::Rect;

#[derive(Rect)]
struct Rectangle {
    width: f64,
    length: f64,
}

fn main() {
    let r = Rectangle {
        length: 30.0,
        width: 20.0,
    };
    println!("Area of a rectangle is: {}", r.area());
}
