/**
 * -----------------------------------------------------------------------------
 * STRATEGY PATTERN
 *
 * To execute, please run: cargo run --bin strategy
 * To run tests, please run: cargo test --bin strategy
 * -----------------------------------------------------------------------------
 *
 * A strategy pattern is a behavioral pattern that allows us to create different
 * algorithms and encapsulate each one and make it interchangeable. It allows us
 * to select an algorithm from the list at runtime without altering the code.
 *
 * It follows SOLID principle.
 *
 * Strategy pattern consists of the following components:
 *
 * 1. Context   : It contains the reference to the strategy object.
 * 2. strategy  : It is a trait that defines the algorithm's contract.
 * 3. Concrete Strategies   : They are different algorithms that are isolated from each other.
 * 4. Client    : This is the target where the strategies are used.
 *
 * The code below shows Different payment Strategies that can be chosen by user
 * to pay bills at the time of checkout.
 *
 * > NOTE: The solution illustrate the payment example, however actual payment
 * > process is not included in the example below.
 */

//

trait Strategy {
    fn pay(&self, amount: f64) -> bool;
}

#[derive(Clone)]
struct CreditCardPayment;

#[derive(Clone)]
struct CashPayment;

impl Strategy for CreditCardPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("Received ${amount} with Credit Card");
        true
    }
}

impl Strategy for CashPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("received ${amount} with cash payment.");
        true
    }
}

#[derive(Clone)]
struct Item {
    name: String,
    price: f64,
}

#[derive(Default)]
struct Cart<T> {
    name: String,
    items: Vec<Item>,
    payment_method: T,
}

impl<T: Strategy> Cart<T> {
    fn new(name: String, payment_method: T) -> Self {
        Cart {
            name,
            items: Vec::new(),
            payment_method,
        }
    }
    fn add_item(&mut self, item: Item) {
        self.items.push(item.clone());
        println!("{} added to the cart {}", item.name, self.name)
    }

    fn checkout(&self) {
        let total_amount = self
            .items
            .iter()
            .map(|item| item.price)
            .reduce(|a, b| a + b)
            .unwrap();
        self.payment_method.pay(total_amount);
    }
}

fn main() {
    let card_payment = CreditCardPayment;
    let cash_payment = CashPayment;

    let item1 = Item {
        name: "Item 1".to_string(),
        price: 10.0,
    };
    let item2 = Item {
        name: "Item 2".to_string(),
        price: 10.0,
    };

    let mut cart1 = Cart::new("Cart 1".to_string(), cash_payment.clone());

    cart1.add_item(item1.clone());
    cart1.add_item(item2.clone());
    cart1.checkout();

    let mut cart2 = Cart::new("Cart 2".to_string(), card_payment.clone());
    cart2.add_item(item1.clone());
    cart2.add_item(item2.clone());
    cart2.checkout();
}
