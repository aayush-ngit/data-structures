// 2. Create a Queue
// A queue is a First-In-First-Out (FIFO) structure, so implement enqueue and dequeue methods.
// Hint: Use a VecDeque (double-ended queue) from the std::collections module.
// Practice Focus: Working with collections and understanding how VecDeque is optimized for pushing/popping from both ends.

use std::collections::VecDeque;

pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T: std::fmt::Display> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) -> &mut Self {
        self.items.push_back(item);
        self
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn print(&mut self) {
        for item in self.items.iter() {
            println!("{}", item);
        }
    }
}

// fn test() {
//     let mut queue: Queue<i32> = Queue::new();
//     queue.enqueue(0).enqueue(1).enqueue(2);

//     println!("Before dequeue: ");
//     queue.print();

//     let dequeue_value = queue.dequeue();
//     match dequeue_value {
//         Some(item) => println!("Dequeue value: {}", item),
//         None => println!("No item in queue!"),
//     }

//     println!("After dequeue: ");
//     queue.print();
// }
