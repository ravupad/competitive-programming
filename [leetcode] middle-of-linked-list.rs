#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            next: None,
            val
        }))
    }

    #[inline]
    fn new2(val: i32, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            next: tail,
            val,
        }))
    }
}

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut len = 0;
        let mut node = &head;
        while !node.is_none() {
            len += 1;
            node = & node.as_ref().unwrap().next;
        }
        let mut node = &mut head;
        for _ in 0..len/2 {
            node = &mut node.as_mut().unwrap().next;
        }
        node.take()
    }
}

fn main() {
    let mut ll = ListNode::new(0);
    for i in 1..5 {
        ll = ListNode::new2(i, ll);
    }
    println!("{:?}", Solution::middle_node(ll));
}
