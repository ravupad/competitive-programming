use std::ptr;
use std::fmt;

struct BinaryTree<T: fmt::Debug> {
    elem: T,
    left: *mut BinaryTree<T>,
    right: *mut BinaryTree<T>,
    parent: *mut BinaryTree<T>,
}

impl<T: fmt::Debug> BinaryTree<T> {
    pub fn new(
        elem: T,
        left: *mut BinaryTree<T>,
        right: *mut BinaryTree<T>,
        parent: *mut BinaryTree<T>,
    ) -> *mut BinaryTree<T> {
        Box::leak(Box::new(BinaryTree {
            elem,
            left,
            right,
            parent,
        }))
    }

    pub fn left(self: &mut Self) -> Option<&mut Self> {
        match self.left.is_null() {
            true => None,
            false => Some(unsafe { &mut *self.left })
        }
    }

    pub fn right(self: &mut Self) -> Option<&mut Self> {
        match self.right.is_null() {
            true => None,
            false => Some(unsafe { &mut *self.right })
        }
    }

    pub fn parent(self: &mut Self) -> Option<&mut Self> {
        match self.parent.is_null() {
            true => None,
            false => Some(unsafe { &mut *self.parent })
        }
    }

    pub fn insert_left(self: &mut Self, elem: T) -> Option<T> {
        let old_left_ptr = self.left;
        self.left = BinaryTree::new(elem, ptr::null_mut(), ptr::null_mut(), self as *mut _);
        match old_left_ptr.is_null() {
            true => None,
            false => unsafe {
                let old_left = &mut *old_left_ptr;
                let val = Some(ptr::read(&old_left.elem));
                ptr::drop_in_place(old_left);
                val
            }
        }
    }

    pub fn insert_right(self: &mut Self, elem: T) -> Option<T> {
        let old_right_ptr = self.right;
        self.right = BinaryTree::new(elem, ptr::null_mut(), ptr::null_mut(), self as *mut _);
        match old_right_ptr.is_null() {
            true => None,
            false => unsafe {
                let old_right = &mut *old_right_ptr;
                let val = Some(ptr::read(&old_right.elem));
                ptr::drop_in_place(old_right);
                val
            }
        }
    }
}

impl<T: fmt::Debug> Drop for BinaryTree<T> {
    fn drop(self: &mut Self) {
        if !self.left.is_null() {
            (&mut *self.left).drop();
        }
        if !self.right.is_null() {
            (&mut *self.right).drop();
        }
        unsafe {
            
        }
    }
}

fn main() { 
    let mut node = unsafe { &mut *BinaryTree::new(1, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) };
    node.insert_left(2);
    node.insert_right(3);
    node = node.left().unwrap();
    node.insert_left(4);
    node.insert_left(5);
    node = node.parent().unwrap().right().unwrap();
    node.insert_left(6);
    node.insert_right(7);


}