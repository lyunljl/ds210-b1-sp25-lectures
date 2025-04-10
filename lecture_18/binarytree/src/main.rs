#[derive(Debug)]
struct TreeNode {
    value: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl TreeNode {
    // Constructor method
    // This method creates a new TreeNode with the given value
    // and initializes left and right child pointers to None
    fn new(value: usize) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinaryTree {
    nodes: Vec<TreeNode>,
    root: Option<usize>,
}

impl BinaryTree {
    // Constructor method
    // This method creates a new BinaryTree with an empty vector of nodes
    // and a root pointer set to None
    fn new() -> Self {
        BinaryTree { nodes: Vec::new(), root: None }
    }

    fn insert(&mut self, value: usize) {
        let new_node_index = self.nodes.len();
        self.nodes.push(TreeNode::new(value));

        match self.root {
            Some(root_index) => self.insert_at(root_index, new_node_index),
            None => self.root = Some(new_node_index),
        }
    }

    // Inserts a new node into a binary tree in the following way:
    // * Start at the specified current node index:
    //   * if both children are empty, insert the left child
    //   * if left child is occupied, insert the right child
    //   * if both children are occupied, recursively attempt to insert the new
    //     node in the left subtree
    fn insert_at(&mut self, current_index: usize, new_node_index: usize) {
        let current_node = &mut self.nodes[current_index];
        if current_node.left.is_none() {
            current_node.left = Some(new_node_index);
        } else if current_node.right.is_none() {
            current_node.right = Some(new_node_index);
        } else {
            // Insert in left subtree for simplicity, could be more complex to keep balanced
            let left = current_node.left.unwrap();
            self.insert_at(left, new_node_index);
        }
    }
    
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(6);
    tree.insert(7);
    tree.insert(11);
    tree.insert(23);
    tree.insert(34);

    println!("{:#?}", tree);
}
