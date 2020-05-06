// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3305/
// binary-tree; preorder; tree-construction

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

struct Solution;

fn main() {
    Solution::bst_from_preorder(vec![8,5,1,7,10,12]);
}

use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn construct(preorder: &Vec<i32>, offset: usize, min: i32, max: i32) -> (Node, usize) {
        if offset == preorder.len() || preorder[offset] < min || preorder[offset] > max {
            return (None, offset);
        }
        let mut node = TreeNode::new(preorder[offset]);
        let (left, offset1) = Self::construct(preorder, offset+1, min, preorder[offset]);
        let (right, offset2) = Self::construct(preorder, offset1, preorder[offset], max);
        node.left = left;
        node.right = right;
        (Some(Rc::new(RefCell::new(node))), offset2)
    }
    
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Node {
        Self::construct(&preorder, 0, std::i32::MIN, std::i32::MAX).0
    }
}
