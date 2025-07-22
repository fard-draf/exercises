use std::fmt::Debug;

//==============================================================================DATA_STRUCTURE
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct IntoIter<T> {
    stack: Vec<Box<Node<T>>>,
}
//==============================================================================HELPER_LEFT
impl<T: Debug> IntoIter<T> {
    fn push_left_branch(&mut self, mut node: Option<Box<Node<T>>>) {
        while let Some(mut current_node) = node {
            node = current_node.left.take();
            self.stack.push(current_node);
        }
    }
}
//==============================================================================ADD_LOGIC
impl<T: Ord + Debug> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn add(&mut self, value: T) {
        println!("ROUND {:#?}", value);
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        if let Some(root) = &mut self.root {
            root.add(new_node);
            println!("added : {:#?}", root);
        } else {
            println!("First node: {:#?}", new_node);
            self.root = Some(new_node)
        }
    }
}

impl<T: Ord> Node<T> {
    fn add(&mut self, new_node: Box<Node<T>>) {
        if new_node.value < self.value {
            if let Some(left) = &mut self.left {
                left.add(new_node);
            } else {
                self.left = Some(new_node);
            }
        } else {
            if let Some(right) = &mut self.right {
                right.add(new_node);
            } else {
                self.right = Some(new_node);
            }
        }
    }
}
//==============================================================================TREE ITERATOR -> APPEL INITIAL
impl<T: Debug> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        // TODO: Initialiser l'état de l'itérateur.
        // Pour un parcours in-order, il faut "empiler" tous les nœuds de gauche en partant de la racine.
        // Crée une fonction privée pour ça !

        let mut iter = IntoIter { stack: Vec::new() };
        iter.push_left_branch(self.root);

        println!("Stack state: {:#?}", iter);

        iter
    }
}

//==============================================================================ITERATOR LOGIC
impl<T: Debug> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: C'est le cœur de l'exercice.
        // L'algorithme général :
        // 1. Dépiler un nœud. C'est le prochain élément à retourner.
        // 2. AVANT de le retourner, regarder si ce nœud a un sous-arbre droit.
        // 3. Si oui, il faut empiler tous les enfants gauches de ce sous-arbre droit.
        // 4. Retourner la valeur du nœud dépilé.
        // 5. Si la pile est vide, le parcours est terminé
        match self.stack.pop() {
            // Empty node, it's over
            None => None,
            Some(mut node_to_return) => {
                // There is a node but we have to check if it had some kids before to return it.
                if let Some(right_child) = node_to_return.right.take() {
                    // same logic for the left kids
                    self.push_left_branch(Some(right_child));
                }
                // we can now return the node value
                Some(node_to_return.value)
            }
        }
    }
}

//==============================================================================TESTS

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
