// 3. Build a Singly Linked List
// Implement methods for inserting, deleting, and searching.
// Hint: Use an enum for recursive nodes (e.g., Node<T>) with Option for nullable pointers.
// Practice Focus: Recursion, Box, and Option.

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> Node<T> {
    pub fn new(data: T) -> Self {
        Node { next: None, data }
    }

    pub fn add_next(&mut self, next: Node<T>) -> &mut Node<T> {
        self.next = Some(Box::new(next));
        self
    }

    pub fn has_next(&mut self) -> bool {
        match self.next {
            Some(_) => true,
            None => false,
        }
    }

    pub fn print(&mut self) {
        println!("{}", self.data);
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // pub fn insert_node(&mut self, data: T) -> &mut Self {
    //     let new_node = Node { data, next: None };

    //     let current_head = self.head;
    //     match current_head {
    //         Some(head) => {
    //             let mut current = head.as_mut();
    //             while current.has_next() {
    //                 current = current.next;
    //             }

    //             current.add_next(new_node);
    //         }
    //         None => self.head = Some(Box::new(new_node)),
    //     }

    //     self
    // }

    pub fn insert_node(&mut self, data: T) -> &mut Self {
        let new_node = Box::new(Node { data, next: None });

        match self.head.as_mut() {
            Some(mut current) => {
                // Traverse to the end of the list
                while let Some(next) = current.next.as_mut() {
                    current = next;
                }

                // Add the new node at the end
                current.next = Some(new_node);
            }
            None => {
                // List is empty, set head to the new node
                self.head = Some(new_node);
            }
        }

        self
    }

    pub fn print(&mut self) {}
}
