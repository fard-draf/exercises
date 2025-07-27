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
            head: core::ptr::null_mut(),
        }
    }

    fn push(&mut self, value: T) {
        let node = Box::new(Node {
            elem: value,
            next: self.head,
        });

        self.head = Box::into_raw(node);
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        let boxed = unsafe { Box::from_raw(self.head) };
        self.head = boxed.next;
        Some(boxed.elem)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

fn main() {
    {
        let mut linkedlist: LinkedList<String> = LinkedList::new();

        for e in 0..1000 {
            let s = format!("String number {}", e);
            let s = String::from(s);
            linkedlist.push(s);
        }
        println!("Linked list: {:#?}", linkedlist);

        for _ in 0..3 {
            println!("Popped: {:?}", linkedlist.pop());
        }
    };
    println!("Value must be dropped.");
}
