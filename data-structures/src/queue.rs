/// Queues
/// Stacks are data types that follows Last In First Out principle. The item
/// first added to the queue should be the item  that gets out of the queue.
/// We can take an analogy of a ticket counter as a queue where the person with
/// the first token will be the first in the queue.
///
/// In this example, we are creating a custom structure `Queue` that stores
/// items of the queue in a vector.
///
/// The queue contains methods `enqueue` and `dequeue` methods to add and remove
/// items from the queue. The item first added to the queue will be removed from
/// the queue first.
struct Queue<T> {
    content: Vec<T>, // container for items to store in the stack
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
    fn enqueue(&mut self, item: T) {
        self.content.push(item)
    }
    fn dequeue(&mut self) -> T {
        return self.content.remove(0);
    }

    fn length(&self) -> usize {
        return self.content.len();
    }
}

/// To run / test, please run the following command in your terminal:
/// * cargo run --bin queue
/// * cargo test --bin queue
///
fn main() {
    let mut queue: Queue<usize> = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    println!("Length of the queue: {}", queue.length()); // 0

    println!("first item: {}", queue.dequeue()); // 10
    println!("first item: {}", queue.dequeue()); // 20
    println!("first item: {}", queue.dequeue()); // 30

    println!("Length of the queue: {}", queue.length()); // 0
}
