use std::fmt::Debug;

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==DATA_STRUCTURE
#[derive(Debug, Clone)]
pub struct Node<T: Clone>
where
    Node<T>: Sized,
{
    value: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T: Clone>
where
    Node<T>: Sized,
{
    root: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct IntoIter<T: Clone>
where
    Node<T>: Clone + Sized,
{
    stack: Vec<(Box<Node<T>>, usize)>,
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ADD_LOGIC

impl<T: Ord + Copy + Debug + Clone> BinaryTree<T>
where
    Node<T>: Clone + Sized,
{
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

impl<T: Ord + Copy + Debug + Clone> Node<T>
where
    Node<T>: Clone + Sized,
{
    fn add_node(&mut self, mut new_node: Box<Node<T>>) {
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
impl<T: Debug + Clone> Iterator for IntoIter<T>
where
    Node<T>: Clone + Sized,
{
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, depth)) = self.stack.pop() {
            if let Some(right) = node.right {
                self.push_left_branch(Some(right), depth + 1);
            }

            Some((node.value, depth))
        } else {
            None
        }
    }
}

impl<T: Debug + Clone> IntoIterator for BinaryTree<T>
where
    Node<T>: Clone,
{
    type Item = (T, usize);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = IntoIter { stack: Vec::new() };
        iter.push_left_branch(self.root, 0);
        iter
    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==PUSH_LEFT_BRANCH
impl<T: Debug + Clone> IntoIter<T>
where
    Node<T>: Clone + Sized,
{
    fn push_left_branch(&mut self, mut node: Option<Box<Node<T>>>, mut depth: usize) {
        while let Some(mut current_node) = node {
            self.stack.push((current_node.clone(), depth));
            node = current_node.left.take();
            depth += 1;
        }
    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==COMPLEXE_AGREG

#[derive(Debug, Default, PartialEq, Clone)]
struct Analysis<T> {
    max_value: T,
    depth: usize,
}

pub fn analyze_tree<T: Ord + Copy + std::fmt::Debug>(tree: BinaryTree<T>) -> Option<(T, usize)>
where
    Node<T>: Clone + Sized,
{
    // Indice : Le type de l'accumulateur de fold pourrait être Option<Analysis<T>>
    // L'état initial serait None. La première valeur itérée crée le premier `Analysis`.
    // Les valeurs suivantes mettent à jour cet `Analysis`.

    let result = tree.into_iter().fold(None, |acc, (value, depth)| {
        println!("value: {:?}, depth: {}", value, depth);
        match acc {
            None => Some(Analysis {
                max_value: value,
                depth: depth,
            }),
            Some(data) => Some(Analysis {
                max_value: value.max(data.max_value),
                depth: depth.max(data.depth),
            }),
        }
    });

    result.map(|analysis| (analysis.max_value, analysis.depth))
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==TESTS

#[cfg(test)]
mod tests {
    use super::*;
    // ...

    #[test]
    fn test_analyze_empty_tree() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(analyze_tree(tree), None);
    }

    #[test]
    fn test_analyze_single_node() {
        let mut tree = BinaryTree::new();
        tree.add(100);
        assert_eq!(analyze_tree(tree), Some((100, 0)));
    }

    #[test]
    fn test_analyze_right_skewed() {
        let mut tree = BinaryTree::new();
        tree.add(10); // 
        tree.add(20);
        tree.add(30); // Profondeur 3
        tree.add(40); // Profondeur 4 
        assert_eq!(analyze_tree(tree), Some((40, 3)));
    }

    #[test]
    fn test_analyze_balanced_tree() {
        let mut tree = BinaryTree::new();
        tree.add(10);
        tree.add(5);
        tree.add(15);
        tree.add(12);
        // Profondeur max est au niveau de 3, 7, 12 ou 20 (profondeur 3)
        // Max value est 20
        assert_eq!(analyze_tree(tree), Some((15, 2)));
    }
}
