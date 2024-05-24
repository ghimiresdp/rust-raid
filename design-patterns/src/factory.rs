/**
 * FACTORY DESIGN PATTERN
 *
 * Factory pattern provides an interface for creating an object.
 * The factory pattern will validate all the initialization parameter before it
 * gets initialized and raises error / panics if we provide an invalid data.
 *
 * The components of Factory pattern are as follows:
 *  1. client: part of code that creates object
 *  2. Factory: interface that defines methods (example: traits)
 *  3. Concrete Factory:
 *  4. Product: it is an instance that factory creates
 **/

// Create a trait that can be implemented on other structs
trait Shape {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}

// create a Rectangle and triangle structs which later implements trait Shape
struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// This is a factory implementation to create a rectangle
impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        if width <= 0.0 || height <= 0.0 {
            panic!("Non-positive value found");
        }
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f64 {
        (self.height + self.width) * 2f64
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// This is a factory implementation to create a triangle
// this can validate creation of a triangle

impl Triangle {
    fn new(a: f64, b: f64, c: f64) -> Self {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            panic!("Non-positive value found");
        }
        if (a + b < c) || a + c < b || b + c < a {
            panic!("Invalid sides for a triangle")
        }
        Self { a, b, c }
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2f64;
        ((s * (s - self.a) * (s - self.b) * (s - self.c)) as f64).sqrt()
    }
}

// To execute the code, run: cargo run --bin factory
fn main() {
    let r1 = Rectangle::new(5.0, 10.0);
    println!("Area of rectangle r1 is: {}", r1.area());

    let t1 = Triangle::new(3.0, 4.0, 5.0);
    println!("Area of triangle t1 is: {}", t1.area());

    // If we ignore the factory method new() and create our instance, then we
    // might encounter issues later in runtime

    // example: triangle side can never be greater than sum of other 2 sides
    // even though the sides are not valid, it does not panic
    let t1 = Triangle {
        a: 5.0,
        b: 6.0,
        c: 12.0,
    };
    println!("t1 area: {}", t1.area());
    // This gives NaN since result will be a complex number.

    // This should panic when uncommented
    // let t2 = Triangle::new(5.0, 6.0, 12.0);
}

#[cfg(test)]
mod tests {
    use crate::{Rectangle, Shape, Triangle};

    #[test]
    fn should_create_a_rectangle() {
        let t1 = Rectangle::new(3.0, 4.0);
        assert_eq!(t1.area(), 12.0);
        assert_eq!(t1.perimeter(), 14.0);
    }

    #[test]
    #[should_panic = "Non-positive value found"]
    fn should_panic_creating_rectangle() {
        Rectangle::new(0.0, 5.5);
    }

    #[test]
    fn should_create_a_triangle() {
        let t1 = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(t1.area(), 6.0);
        assert_eq!(t1.perimeter(), 12.0);
    }

    #[test]
    #[should_panic = "Non-positive value found"]
    fn should_panic_creating_triangle() {
        Triangle::new(5.0, 6.0, 0.0);
    }

    #[test]
    #[should_panic = "Invalid sides for a triangle"]
    fn should_panic_with_triangle_rule() {
        Triangle::new(5.0, 6.0, 12.0);
    }
}
