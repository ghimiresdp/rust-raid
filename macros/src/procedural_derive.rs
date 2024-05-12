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

#[cfg(test)]
mod tests {
    use shape::Rect;

    use crate::Rectangle;

    #[test]
    fn test_derive_macro() {
        assert_eq!(
            (Rectangle {
                length: 10.0,
                width: 10.0
            })
            .area(),
            100f64
        );
    }
}
