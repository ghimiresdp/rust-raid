/**
 * -----------------------------------------------------------------------------
 * DECORATOR PATTERN
 *
 * To execute, please run: cargo run --bin decorator
 * To run tests, please run: cargo test --bin decorator
 * -----------------------------------------------------------------------------
 *
 *
 * decorator pattern allows adding new functionality to the existing object
 * without changing the structure of the original object.
 *
 * The example below shows how decorator can be used as a logger in an
 * application
 **/

// Auth Trait that prototypes login() method.
trait Auth {
    fn login(&self, username: String, password: String);
}

// User Structure rthat implements trait Auth.
struct User;

impl Auth for User {
    fn login(&self, username: String, password: String) {
        println!(
            "Login\t\tusername: {}\t\tpassword: {}",
            username,
            "*".repeat(password.len())
        );
    }
}

// struct Logger that holds original struct `User`.
struct Logger<T: Auth> {
    model: T,
}

// initializer for the logger
impl<T: Auth> Logger<T> {
    fn new(model: T) -> Logger<T> {
        Logger { model }
    }
}

// implement Auth for Logger component so that it later uses login() for user
impl<T: Auth> Auth for Logger<T> {
    fn login(&self, username: String, password: String) {
        print!("[Log] [Logger Decorator]:\t\t");
        self.model.login(username, password);
    }
}

// to run, execute "cargo run --bin decorator"
fn main() {
    let u = User {};
    // undecorated
    u.login("root".to_string(), "pass".to_string());

    // decorated
    let dec = Logger::new(u);
    dec.login("user".to_string(), "pass".to_string())
}
