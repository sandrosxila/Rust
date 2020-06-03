#![allow(unused_parens)]

use std::cmp;
use std::fmt::Debug;

// Structures
#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    size: i32,
}

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

// Iterator Structures
pub struct IntoIter<T> (List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// Implementations
impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node { elem: elem, next: self.head.take() }));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.size = cmp::max(self.size - 1, 0);
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
        // self.head.take().map(|node|{
        //     self.head = node.next;
        //     node.elem
        // })
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_top(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(node) => {
                Some(&node.elem)
            }
            None => None
        }
    }

    // Iterator Methods
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| { &**node }),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| { &mut **node }),
        }
    }
}

// Traits
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Drop {
    fn drop(&mut self);
}

// Traits Implementation
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|next_node| { &**next_node });
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|next_node| { &mut **next_node });
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
        }
    }
}