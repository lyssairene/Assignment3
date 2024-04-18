// Define a Node struct
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

// Define a LinkedList struct
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Insert data at the beginning of the list
    pub fn insert(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    // Insert data at the end of the list
    pub fn insert_at_tail(&mut self, data: T) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node::new(data)));
    }

    // Insert data at a specific index
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(ref mut node) = *current {
                current = &mut node.next;
            } else {
                return; // Index out of bounds
            }
        }
        let mut new_node = Box::new(Node::new(data));
        new_node.next = current.take();
        *current = Some(new_node);
    }

    // Delete data at a specific index
    pub fn delete(&mut self, index: usize) {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(ref mut node) = *current {
                current = &mut node.next;
            } else {
                return; // Index out of bounds
            }
        }
        if let Some(node) = current.take() {
            *current = node.next;
        }
    }

    // Update data at a specific index
    pub fn update(&mut self, index: usize, data: T) {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(ref mut node) = *current {
                current = &mut node.next;
            } else {
                return; // Index out of bounds
            }
        }
        if let Some(ref mut node) = *current {
            node.data = data;
        }
    }

    // Get data at a specific index
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        for _ in 0..index {
            if let Some(ref node) = *current {
                current = &node.next;
            } else {
                return None; // Index out of bounds
            }
        }
        current.as_ref().map(|node| &node.data)
    }
}

// Example usage
fn main() {
    let mut list = LinkedList::new();
    list.insert(9);
    list.insert(6);
    list.insert(4);

    // Display the list
    let mut current = &list.head;
    print!(" [");
    while let Some(ref node) = *current {
        print!("{}", node.data);
        current = &node.next;
    }
    println!("] ");

    list.insert_at_tail(4);

    // Display the list after adding data at tail
    let mut current = &list.head;
    print!(" [");
    while let Some(ref node) = *current {
        print!("{}", node.data);
        current = &node.next;
    }
    println!("] ");

    list.insert_at_index(2, 5);

    // Display the list after adding data at index 2
    let mut current = &list.head;
    print!(" [");
    while let Some(ref node) = *current {
        print!("{}", node.data);
        current = &node.next;
    }
    println!("] ");

    list.delete(1);

    // Display the list after deleting data at index 1
    let mut current = &list.head;
    print!(" [");
    while let Some(ref node) = *current {
        print!("{}", node.data);
        current = &node.next;
    }
    println!("] ");

    list.update(1, 6);

    // Display the list after updating data at index 1
    let mut current = &list.head;
    print!(" [");
    while let Some(ref node) = *current {
        print!("{}", node.data);
        current = &node.next;
    }
    println!("] ");

    // Get data at index 2
    if let Some(data) = list.get(2) {
        println!("Data at index 2: {}", data);
    } else {
        println!("Index out of bounds");
    }
}
