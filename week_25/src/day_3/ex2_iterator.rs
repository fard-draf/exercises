#[derive(Debug)]
pub struct MyBox<T> {
    pub items: Vec<T>,
}

impl<T> MyBox<T> {
    fn new() -> Self {
        MyBox { items: Vec::new() }
    }

    fn push(&mut self, data: T) {
        self.items.push(data);
    }
}

pub struct IntoIter<T> {
    inner: std::vec::IntoIter<T>,
}

impl<T> IntoIterator for MyBox<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.items.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub fn iteration_sur_collec() {
    let mut box1 = MyBox::new();
    box1.push("La belle au bois dormant".to_string());
    box1.push("Alice au pays des merveilles".to_string());
    box1.push("Le bossu de Notre Dame".to_string());

    let collection = box1
        .into_iter()
        .filter(|e| e.contains("bossu"))
        .collect::<Vec<String>>();

    println!("{:?}", collection);
}
