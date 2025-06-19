use std::fmt::Debug;


//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==DATA_STRUCTURE
#[derive(Debug)]
pub struct Node<T> {
    value: (T, usize),
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>
}

pub struct IntoIter<T> {
    stack: Vec<Box<Node<T>>>,
 
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ADD_LOGIC

impl<T: Ord + Copy + Debug> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn add(&mut self, data: T) {
        let new_node = Box::new(Node { value: (data, 1), right: None, left: None});

        if let Some(root) = &mut self.root {
      
            root.add_node(new_node);
        } else {

            self.root = Some(new_node);
            
        }

    }

}

impl<T: Ord + Copy + Debug> Node<T> {
    fn add_node(&mut self, mut new_node: Box<Node<T>>) {
      
        println!("value {:?}, depth {}", new_node.value.0, new_node.value.1);
        if new_node.value < self.value {
            new_node.value.1 += 1;
            if let Some(left_node) = &mut self.left {
                left_node.add_node(new_node);
            } else {
                
                self.left = Some(new_node)
            }
        } else {
            new_node.value.1 += 1;
            if let Some(right_node) = &mut self.right {
                right_node.add_node(new_node);
            } else {
                self.right = Some(new_node)
            }
        }
    }
}




//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ITER_LOGIC
impl<T: Debug> Iterator for IntoIter<T> {
    type Item = (T, usize);
    
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

impl<T: Debug> IntoIterator for BinaryTree<T> {
    type Item = (T, usize);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = IntoIter {stack: Vec::new()};
        iter.push_left_branch(self.root);
        iter        
    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==PUSH_LEFT_BRANCH
impl<T: Debug> IntoIter<T> {
    fn push_left_branch(&mut self, mut node: Option<Box<Node<T>>>)  {
    
        while let Some(mut current_node) = node {

            node = current_node.left.take();
            self.stack.push(current_node);
           
   
        }

        

    }
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==COMPLEXE_AGREG

#[derive(Debug, Default, PartialEq)]
struct Analysis<T> {
    max_value: T,
    depth: usize,
}


pub fn analyze_tree<T: Ord + Copy + std::fmt::Debug >(tree: BinaryTree<T>) -> Option<(T, usize)> 

{
    // Indice : Le type de l'accumulateur de fold pourrait être Option<Analysis<T>>
    // L'état initial serait None. La première valeur itérée crée le premier `Analysis`.
    // Les valeurs suivantes mettent à jour cet `Analysis`.

        let result = tree.into_iter().fold( None, |acc, (value, depth)| {
            println!("value: {:?}, depth: {}",value, depth);
            match  acc {
                None => Some(Analysis { max_value: value, depth: depth}),
                Some(data ) => {
                    Some(Analysis { max_value: value.max(data.max_value), depth:  depth.max(data.depth) })
                }
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
        assert_eq!(analyze_tree(tree), Some((100, 1)));
    }

    #[test]
    fn test_analyze_right_skewed() {
        let mut tree = BinaryTree::new();
        tree.add(10); // 
        tree.add(20);
        tree.add(30); // Profondeur 3
        tree.add(40); // Profondeur 4 
        assert_eq!(analyze_tree(tree), Some((40, 4)));
    }

    #[test]
    fn test_analyze_balanced_tree() {
        let mut tree = BinaryTree::new();
        tree.add(100);
        tree.add(50);
        tree.add(150);
        tree.add(40);
        tree.add(60);
        tree.add(140);
        tree.add(160);
        tree.add(41);
        tree.add(42);
        tree.add(43);
        // Profondeur max est au niveau de 3, 7, 12 ou 20 (profondeur 3)
        // Max value est 20
        assert_eq!(analyze_tree(tree), Some((160, 6)));
    }
}