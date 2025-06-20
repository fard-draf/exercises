//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==DATA_STRUCTURE
pub struct Node<T> {
    value: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

pub struct IntoIter<T> {
    stack: Vec<Box<Node<T>>>,
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ADD_LOGIC

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn add(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            right: None,
            left: None,
        });

        if let Some(root) = &mut self.root {
            root.add_node(new_node);
        } else {
            self.root = Some(new_node);
        }
    }
}

impl<T: Ord> Node<T> {
    fn add_node(&mut self, new_node: Box<Node<T>>) {
        if new_node.value < self.value {
            if let Some(left_node) = &mut self.left {
                left_node.add_node(new_node);
            } else {
                self.left = Some(new_node)
            }
        } else {
            if let Some(right_node) = &mut self.right {
                right_node.add_node(new_node);
            } else {
                self.right = Some(new_node)
            }
        }
    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ITER_LOGIC
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(node_to_return) => {
                if let Some(right_node) = node_to_return.right {
                    self.push_left_branch(Some(right_node));
                }
                Some(node_to_return.value)
            }
        }
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = IntoIter { stack: Vec::new() };
        iter.push_left_branch(self.root);
        iter
    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==PUSH_LEFT_BRANCH
impl<T> IntoIter<T> {
    fn push_left_branch(&mut self, mut node: Option<Box<Node<T>>>) {
        while let Some(mut current_node) = node {
            node = current_node.left.take();
            self.stack.push(current_node);
        }
    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_node_tree() {
        let mut tree = BinaryTree::new();
        tree.add(10);
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_in_order_traversal() {
        let mut tree = BinaryTree::new();
        // Structure:    10
        //              /  \
        //             5    15
        //            / \   /
        //           3   7 12
        tree.add(10);
        tree.add(5);
        tree.add(15);
        tree.add(3);
        tree.add(7);
        tree.add(12);

        let result: Vec<_> = tree.into_iter().collect();
        assert_eq!(result, vec![3, 5, 7, 10, 12, 15]);
    }

    #[test]
    fn test_left_skewed_tree() {
        let mut tree = BinaryTree::new();
        tree.add(50);
        tree.add(40);
        tree.add(30);
        tree.add(20);

        let result: Vec<_> = tree.into_iter().collect();
        assert_eq!(result, vec![20, 30, 40, 50]);
    }

    #[test]
    fn test_right_skewed_tree() {
        let mut tree = BinaryTree::new();
        tree.add(20);
        tree.add(30);
        tree.add(40);
        tree.add(50);

        let result: Vec<_> = tree.into_iter().collect();
        assert_eq!(result, vec![20, 30, 40, 50]);
    }
}
