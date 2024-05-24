#![allow(dead_code)]

/**
 * -----------------------------------------------------------------------------
 * BUILDER PATTERN
 *
 * To execute, please run: cargo run --bin builder
 * To run tests, please run: cargo test --bin builder
 * -----------------------------------------------------------------------------
 *
 *
 * Builder Pattern lets us construct objects step by step. As rustlang does not
 * support object-oriented programming, We can not build  different type of
 * object with the help of inheritance, however we can construct structures with
 * optional values.
 *
 * The example below shows a builder pattern that builds computer with processor
 * and memory with optional
 * Keyboard and Display.
 **/

#[derive(Debug)]
enum Processor {
    AMD,
    ARM,
    Intel,
}

#[derive(Debug)]
enum KeyboardVariant {
    Backlit,
    NonBacklit,
    Mechanical,
}

#[derive(Debug)]
enum Display {
    HD,
    FHD,
    QHD,
    UHD,
}

#[derive(Debug)]
struct Computer {
    processor: Processor,
    // all optional values will be build after instantiation
    memory: Option<u32>,
    keyboard: Option<KeyboardVariant>,
    display: Option<Display>,
}

impl Computer {
    fn new(processor: Processor) -> Self {
        Self {
            processor,
            memory: None,
            keyboard: None,
            display: None,
        }
    }

    fn set_ram(&mut self, size_mb: u32) {
        self.memory = Some(size_mb);
    }
    fn set_display(&mut self, variant: Display) {
        self.display = Some(variant);
    }
    fn set_keyboard(&mut self, variant: KeyboardVariant) {
        self.keyboard = Some(variant)
    }

    fn boot(&self) {
        match self.memory {
            Some(size) => println!(
                "The computer boots with {:?} processor and {size}MB of memory",
                self.processor
            ),
            None => println!("The Computer failed to boot!! CODE: NO_MEMORY_FOUND"),
        }
    }
    fn input(&self, key: char) {
        match &self.keyboard {
            Some(kbd) => println!("Key: '{key}' pressed on the {:?} keyboard", kbd),
            None => println!("No Keyboard found !!"),
        }
    }

    fn display_resolution(&self) {
        match self.display {
            Some(Display::HD) => println!("DISPLAY RESOLUTION IS: 1366 X 768"),
            Some(Display::FHD) => println!("DISPLAY RESOLUTION IS: 1920 X 1080"),
            Some(Display::QHD) => println!("DISPLAY RESOLUTION IS: 2560 X 1440"),
            Some(Display::UHD) => println!("DISPLAY RESOLUTION IS: 3840 X 2160"),
            None => println!("NO DISPLAY FOUND !!"),
        }
    }
}

fn main() {
    // build Computer
    let mut c = Computer::new(Processor::Intel);
    println!("{:?}", c);

    // set RAM
    c.boot(); // The Computer failed to boot!! CODE: NO_MEMORY_FOUND
    c.set_ram(4096);
    c.boot(); // The computer boots with 4096MB of RAM

    // attach keyboard
    c.input('A'); // No Keyboard found !!
    c.set_keyboard(KeyboardVariant::Mechanical);
    c.input('A'); // 'A' pressed on the Mechanical keyboard

    // attach monitor
    c.display_resolution(); // NO DISPLAY FOUND !!
    c.set_display(Display::FHD);
    c.display_resolution(); // DISPLAY RESOLUTION IS: 1920 X 1080
}
