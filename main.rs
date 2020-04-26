use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

type Depth = i32;
type Diameter = i32;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn diameter_of_binary_tree(root: Node) -> i32 {
        Solution::depth_and_diameter(&root).1
    }

    fn depth_and_diameter(root: &Node) -> (Depth, Diameter) {
        if let Some(node) = root {
            let (l_depth, l_diameter) = Solution::depth_and_diameter(&node.borrow().left);
            let (r_depth, r_diameter) = Solution::depth_and_diameter(&node.borrow().right);
            (max(l_depth, r_depth)+1, max(l_depth+r_depth, max(l_diameter, r_diameter)))
        } else {
            (0, 0)
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Node, right: Node) -> Node {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left,
            right
        })))
    }
}

struct Solution;

fn main() {
    let mut node1;
    let mut node2;
    node1 = TreeNode::new(1, None, None);
    node2 = TreeNode::new(2, None, None);
    node1 = TreeNode::new(3, node1, node2);
    node2 = TreeNode::new(4, None, None);
    node1 = TreeNode::new(5, node1, node2);
    println!("{}", Solution::diameter_of_binary_tree(node1));
}
