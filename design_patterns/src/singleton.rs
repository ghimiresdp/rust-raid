/**
 * SINGLETON DESIGN PATTERN
 *
 * Singleton design pattern ensures that a class has only one instance and
 * provides a global point of access to it.
 *
 * Generally, Singleton design pattern is popular in Object-oriented programming
 * languages, however as we can create objects of structs in rust, we can use it
 * in structures to use the singleton pattern.
 *
 * We use ``std::sync::Mutex`` to hold the reference of the newly created
 * instance of the struct. This will not completely create a singleton however
 * we can see similar behavior from it.
 */
use std::sync::Mutex;

// initialize display mutex with default w and h set to 0
static DISPLAY: Mutex<Display> = Mutex::new(Display::new(0, 0));

#[derive(Debug)]
struct Display {
    width: u32,
    height: u32,
}

impl Display {
    const fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    fn set_dimension(w: u32, h: u32){
        let mut guard = DISPLAY.lock().unwrap();
        guard.height = h;
        guard.width = w;
    }

    fn get_instance() -> Self {
        // Remember that get_instance() is going to give you the same instance
        // no matter how many times you call, however, if you try to initialize
        // your own instance and set it to another variable, it might behave
        // differently. But you'll still be able to access singleton instance by
        // calling `get_instance()` method from any instance of struct Display.
        let guard = DISPLAY.lock().unwrap();
        Self { width: guard.width, height: guard.height}
    }
}

fn main() {
    println!("Displaying Singleton Data");
    Display::set_dimension(1920, 1080);
    println!("{:?}", Display::get_instance());
    println!("Displaying Singleton Data from same instance");
    Display::set_dimension(3840, 2160);
    println!("{:?}", Display::get_instance());

    let d = Display {width: 20, height: 30};
    println!("{:?}", d);
}
