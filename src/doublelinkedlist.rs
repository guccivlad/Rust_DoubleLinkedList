use std::fmt::Display;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

pub struct Iter<'a, T: 'a> {
    current: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

pub struct DoubleLinkedList<T> {
    len: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialEq + Display> DoubleLinkedList<T> {
    pub fn new() -> Self {
        return Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.len {
            self.push_back(value);
            return;
        }
        if index == 0 {
            self.push_front(value);
            return;
        }

        let mut current = self.head.clone();
        let new_node = Rc::new(RefCell::new(
                    Node {
                        value: value,
                        next: None,
                        prev: None,
                    }
                ));
        
        let mut idx: usize = 0;
        while idx < index {
            let next = current.as_ref().unwrap().borrow().next.clone();
            current = next;

            idx += 1;
        }

        new_node.borrow_mut().prev = current.as_ref().unwrap().borrow().prev.clone();
        current.as_ref().unwrap().borrow().prev.as_ref().unwrap().upgrade().as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new_node));
        current.as_ref().unwrap().borrow_mut().prev = Some(Rc::downgrade(&new_node));
        new_node.borrow_mut().next = Some(Rc::clone(&current.as_ref().unwrap()));

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.len {
            self.pop_back();
            return;
        }
        if index == 0 {
            self.pop_front();
            return;
        }

        let mut current = self.head.clone();
        let mut idx: usize = 0;
        while idx < index {
            let next = current.as_ref().unwrap().borrow().next.clone();
            current = next;

            idx += 1;
        }

        current.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow_mut().prev = current.as_ref().unwrap().borrow().prev.clone();
        current.as_ref().unwrap().borrow().prev.as_ref().unwrap().upgrade().as_ref().unwrap().borrow_mut().next =  current.as_ref().unwrap().borrow().next.clone();

        self.len -= 1;
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = self.head.clone();
        for _ in 0..self.len {
            if current.as_ref().unwrap().borrow().deref().value == *value {
                return true;
            }
            let next = current.as_ref().unwrap().borrow().next.clone();
            current = next;
        }

        return false;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(
            Node {
                value: value,
                next: None,
                prev: None,
            }
        ));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        } else {
            let tail_ref = self.tail.as_ref().unwrap();

            tail_ref.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::downgrade(&tail_ref));

            self.tail = Some(new_node);
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(
            Node {
                value: value,
                next: None,
                prev: None,
            }
        ));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        } else {
            let head_ref = self.head.as_ref().unwrap();

            head_ref.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(Rc::clone(&head_ref));

            self.head = Some(new_node);
        }

        self.len += 1;
    }

    pub fn pop_back(&mut self) {
        if self.tail.is_none() {
            println!("ERROR: DoubleLinkedList is empty");
            return;
        }

        if self.tail.as_ref().unwrap().borrow().prev.is_none() {
            self.head = None;
            self.tail = None;
        } else {
            let new_tail = self.tail.as_ref().unwrap().borrow().prev.as_ref().unwrap().upgrade();
            new_tail.as_ref().unwrap().borrow_mut().next = None;
        
            self.tail = new_tail;
        }

        self.len -= 1;
    }

    pub fn pop_front(&mut self) {
        if self.head.is_none() {
            println!("ERROR: DoubleLinkedList is empty");
            return;
        }

        if self.head.as_ref().unwrap().borrow().next.is_none() {
            self.head = None;
            self.tail = None;
        } else {
            let new_head = self.head.as_ref().unwrap().borrow().next.clone();
            new_head.as_ref().unwrap().borrow_mut().prev = None;
        
            self.head = new_head;
        }

        self.len -= 1;
    }

    pub fn clear(&mut self) {
        for _ in 0..self.len {
            self.pop_back();
        }
    }

    pub fn iter(&self) -> Iter<'_ , T> {
        return Iter {
            current: self.head.as_ref().map(|node| NonNull::new(node.as_ptr())).flatten(),
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let node_value = &(*self.current.as_ref().unwrap().as_ptr()).value;
            self.len -= 1;
            let next = (*self.current.as_ref().unwrap().as_ptr()).next.as_ref();
            self.current = next.map(|node| NonNull::new(node.as_ptr())).flatten();

            return Some(node_value);
        }
    }
}