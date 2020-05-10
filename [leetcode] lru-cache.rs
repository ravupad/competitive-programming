// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/531/week-4/3309/
// doubly linked list; dequeue; lru; cache;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct DoublyLinkedList<T> {
    len: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct Node<T> {
    element: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl <T> Node<T> {
    fn new(element: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            element,
            prev: None,
            next: None,
        }))       
    }
}

impl <T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            len: 0,
            head: None,
            tail: None,
        }
    }
    
    /*fn push_tail(&mut self, element: i32) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.len += 1;
    }*/

    fn push_head(&mut self, element: T) {
        let new_head = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
        self.len += 1;
    }

    fn pop_tail(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.len == 0 {
            return None;
        } else {
            self.len -= 1;
        }
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.tail.take();
                }                
            }
            old_tail
        })
    }

    /*fn pop_head(&mut self) -> Option<Rc<RefCell<Node>>> {
        if self.len == 0 {
            return None;
        } else {
            self.len -= 1;
        }
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.head.take();
                }                
            }
            old_head
        })
    }*/

    fn remove_node(&mut self, node: &mut Rc<RefCell<Node<T>>>) {
        let next = node.borrow_mut().next.take();
        let prev = node.borrow_mut().prev.take();        
        if let Some(next) = next.as_ref() {
            next.borrow_mut().prev = prev.clone();
        }
        if let Some(prev) = prev.as_ref() {
            prev.borrow_mut().next = next.clone();
        }
        if Rc::ptr_eq(&node, self.head.as_ref().unwrap()) {
            self.head = next.clone();
        }
        if Rc::ptr_eq(&node, self.tail.as_ref().unwrap()) {
            self.tail = prev.clone();
        }
        self.len -= 1;
    }
}


struct LRUCache {
    dll: RefCell<DoublyLinkedList<(i32, i32)>>,
    map: RefCell<HashMap<i32, Rc<RefCell<Node<(i32, i32)>>>>>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            dll: RefCell::new(DoublyLinkedList::new()),
            map: RefCell::new(HashMap::new()),
            capacity: capacity as usize,
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.map.borrow_mut().get_mut(&key) {
            self.dll.borrow_mut().remove_node(node);
            self.dll.borrow_mut().push_head((key, node.borrow().element.1));
        } else {
            return -1;
        }
        self.map.borrow_mut().insert(key, self.dll.borrow().head.clone().unwrap());
        self.dll.borrow().head.as_ref().unwrap().borrow().element.1
    }
    
    fn put(&self, key: i32, value: i32) {
        if let Some(node) = self.map.borrow_mut().get_mut(&key) {
            self.dll.borrow_mut().remove_node(node);
        }
        self.dll.borrow_mut().push_head((key, value));
        self.map.borrow_mut().insert(key, self.dll.borrow().head.clone().unwrap());
        if self.dll.borrow().len == self.capacity+1 {
            let node = self.dll.borrow_mut().pop_tail().unwrap();
            self.map.borrow_mut().remove(&node.borrow().element.0);
        }
    }
}

fn main() {
    let cache = LRUCache::new(3);
    cache.put(1,1);
    println!("{}", cache.get(1));
    cache.put(2,2);
    cache.put(3,3);
    cache.put(4,4); // 1 should be evicted
    println!("{}", cache.get(1));
    println!("{}", cache.get(2)); // 2 is at front
    cache.put(5,5); // 3 will be evicted 5->2->4
    println!("{}", cache.get(3));
    cache.put(6,6); // 4 will be evicted
    println!("{}", cache.get(4));
}
