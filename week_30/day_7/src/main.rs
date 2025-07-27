struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: *mut Node<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: core::ptr::dangling_mut(),
        }
    }

    fn push(&mut self, value: T) {
        let old_head = self.head;
        let node = Box::new(Node {
            elem: value,
            next: old_head,
        });
        self.head = Box::into_raw(node);
    }
}


fn main() {
    
    let mut linkedlist: LinkedList<String> = LinkedList::new();
    let node1 = String::from("Blab");
    linkedlist.push(node1);
    let node2 = String::from("Blab2");
    linkedlist.push(node2);

    println!("Linked list: {:#?}", linkedlist);
}
