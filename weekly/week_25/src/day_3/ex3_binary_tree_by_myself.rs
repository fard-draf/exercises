//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==DATA_STRUCTURE

pub struct Node<T> {
    value: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}
/// On lui donne une valeur, elle garde cette valeur et doit savoir si elle a des enfants (right, left) qui sont du meme type qu elle meme
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>
}
/// Son seul job et de savoir si elle a une racine. 
pub struct IntoIter<T> {
    stack: Vec<Box<Node<T>>>
}
/// IntoIter est l iterateur de BinaryTree. Il doit avoir en memoire quels nodes sont a visiter -> sur la todolist, pour cela, il a une stack de Node
//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ADD_LOGIC

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }
// cette methode initialise l instance Binarytree
    pub fn add(&mut self, value: T) {
        let new_node = Box::new(Node { value, right: None, left: None});
        
        if let Some(root) = &mut self.root {
            root.add_node(new_node);
        } else {
            self.root = Some(new_node);
        }
    }
// cette methode permet de creer la racine. 
// La racine existe -> Elle delegue au service adapte Node::add_node 
// La racine n existe pas -> Initialisation de la racine en node (valeur, right None, left None)
}

impl<T: Ord> Node<T> {
    pub fn add_node(&mut self, new_node: Box<Node<T>>) {
        if new_node.value < self.value {
            if let Some(left_node) = &mut self.left {
                left_node.add_node(new_node);
            } else {
                self.left = Some(new_node);
            }
        } else {
            if let Some(right_node) = &mut self.right {
                right_node.add_node(new_node);
            } else {
                self.right = Some(new_node);
            }    
        }    
    }    
// add_node doit etre en mesure d ajouter un nouveau noeud au bon endroit.
// pour cela, elle va comparer la valeur du nouveau noeud avec la sienne.
// si le noeud est plus grand et qu un enfant right existe -> delagation a l enfant right -> recursion add_node avec current_node = enfant.right 
// si le noeud est plus grand MAIS que l enfant right n existe pas -> add_node cree le noeud.right sur cette instance, current_node.right = Some(value, None, None)
// si le noeud est plus petit, meme principe qu avec right mais pour left
}    

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==BRANCHES_HELPER

impl<T> IntoIter<T> {

    fn push_left_branch(&mut self, mut node: Option<Box<Node<T>>>) {

        while let Some(mut current_node) = node {
            node = current_node.left.take();
            self.stack.push(current_node);
        }
    }
}
// push_left_branch doit etre en mesure d aller inspecter tous les nodes de left jusqu a trouver left: None, et pour chaque valeur, elle take la valeur pour faire une recursion. Elle push le noeud courant lui meme sur la stack.
// elle va faire un magnifique move -> si il y a un enfant left -> current_node = node, ce qui  l iteration avec le while let.

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==ITER_LOGIC

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(mut return_node) => {
                if let Some(right_child) = return_node.right.take() {
                    self.push_left_branch(Some(right_child))
                    
                }    
                Some(return_node.value)
            }     
        }    
    }    
// le job de next est de prendre le dernier item sur la stack (pop), de le retourner, mais avant de le retourner, il doit regarder si cette valeur a des enfants right. 
// si ils en ont, il appelle push_left_branch qui vient investiguer dans cet enfant.right et creuse jusqu a trouver None. Elle pousse egalement tous les enfants left  du current_node sur la stack donc la stack est rechargee.
// dans tous les cas, le current_node est retourne  
}    


impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = IntoIter { stack: Vec::new()};
        iter.push_left_branch(self.root);
        iter
    }    
}    
// le job d into iter sur BinaryTree est d intialiser un storage adapte pour la stack (ici un Vec)
// il rempli la pile initialement avec toutes les valeurs de gauches. Toutes les valeurs de gauches seront inspectees tour a tour pour voir si elles ont des valeurs de droite. 

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