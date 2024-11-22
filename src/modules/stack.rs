// 1. Implement a Stack
// A stack is a Last-In-First-Out (LIFO) structure, so implement push, pop, and peek methods.
// Hint: Use a Vec (Rust’s growable array) as the underlying container.
// Practice Focus: Ownership, mutability, and error handling (e.g., returning Option when pop or peek is called on an empty stack).

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T: std::fmt::Display> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.items.last()
    }

    pub fn print(&mut self) {
        for item in self.items.iter() {
            println!("{}", item);
        }
    }
}

// fn test() {
//     let mut stack: Stack<i32> = Stack::new();
//     stack.push(0).push(1).push(2);

//     stack.print();

//     let tos = stack.peek();
//     match tos {
//         Some(item) => println!("Top of stack: {}", item),
//         None => println!("Stack is empty!"),
//     }

//     let popped = stack.pop();
//     match popped {
//         Some(item) => println!("pop value: {}", item),
//         None => println!("Stack is empty!"),
//     }

//     stack.print();
// }
