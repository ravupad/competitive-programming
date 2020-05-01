// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/532/week-5/3315/
// dfs depth first search

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn dfs(root: &Node, arr: &Vec<i32>, counter: usize) -> bool {
        if root.is_none() || counter == arr.len() {
            return false;
        }
        let node = root.as_ref().unwrap();
        if node.borrow().val != arr[counter] {
            return false;
        }
        let (left, right) = (&node.borrow().left, &node.borrow().right);
        if left.is_none() && right.is_none() && counter == arr.len()-1 {
            return true;
        }
        Solution::dfs(left, arr, counter+1) || Solution::dfs(right, arr, counter+1)
    }
    
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        Solution::dfs(&root, &arr, 0)
    }
}
