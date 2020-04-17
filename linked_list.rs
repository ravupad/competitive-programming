use std::ptr;
use std::iter::DoubleEndedIterator;
use std::iter::Iterator;

pub struct LinkedList<T> {
	head: *mut Node<T>,
	tail: *mut Node<T>,
}

pub struct Node<T> {
	elem: T,
	next: *mut Node<T>,
	prev: *mut Node<T>,
}

impl<T> Node<T> {
	fn new_node(elem: T, next: *mut Node<T>, prev: *mut Node<T>) -> *mut Node<T> {
		Box::into_raw(Box::new(Node { elem, next, prev }))
	}
}

impl<T> LinkedList<T> {
	pub fn new() -> Self {
		LinkedList { 
			head: ptr::null_mut(), 
			tail: ptr::null_mut(),
		}
	}

	pub fn push_head(&mut self, elem: T) {
		let old_head = self.head;
		self.head = Node::new_node(elem, old_head, ptr::null_mut());
		unsafe { &mut *old_head }.prev = self.head;
		if self.tail.is_null() {
			self.tail = self.head;
		}
	}

	pub fn pop_head(&mut self) -> Option<T> {
		match self.head.is_null() {
			true => None,
			false => {
				let old_head_ptr = self.head;
				let old_head = unsafe { &*old_head_ptr };
				let new_head_ptr = old_head.next;
				let new_head = unsafe { &mut *new_head_ptr };
				new_head.prev = ptr::null_mut();
				self.head = new_head_ptr;
				if new_head_ptr.is_null() {
					self.tail = ptr::null_mut();
				}
				let val = unsafe { ptr::read(&old_head.elem as *const T) };
				unsafe { ptr::drop_in_place(old_head_ptr) };
				Some(val)
			},
		}
	}

	pub fn push_tail(&mut self, elem: T) {
		let new_tail_ptr = Node::new_node(elem, ptr::null_mut(), self.tail);
		let new_tail = unsafe { &mut *new_tail_ptr };
		match self.head.is_null() {
			true => {
				self.head = new_tail_ptr;
				self.tail = new_tail_ptr;
			}
			false => {
				let old_tail_ptr = self.tail;
				let old_tail = unsafe { &mut *old_tail_ptr };
				old_tail.next = new_tail_ptr;
				new_tail.prev = old_tail_ptr;
				self.tail = new_tail_ptr;
			}
		}
	}

	pub fn head(&mut self) -> Option<&mut Node<T>> {
		match self.head.is_null() {
			true => None,
			false => Some(unsafe { &mut *self.head })
		}
	}
}

impl<T> Drop for LinkedList<T> {
	fn drop(&mut self) {
		let mut current = self.head;
		loop {
			match current.is_null() {
				true => break,
				false => {
					let temp = current;
					current = unsafe { &*current }.next;
					unsafe { ptr::drop_in_place(temp) };
				}
			}
		}
	}
}

fn main() {
	let mut l: LinkedList<i32> = LinkedList::new();
	l.push_tail(6);
	l.push_tail(0);
	l.push_head(7);
	l.push_tail(1);
}