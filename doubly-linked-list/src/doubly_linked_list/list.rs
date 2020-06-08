#![allow(unused_parens)]

use std::cmp;
use std::fmt::Debug;

// Structures
#[derive(Debug, PartialEq, PartialOrd)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    size: i32,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

// Iterator Structures

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

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node: &Box<Node<T>>| { &**node }),
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
        let mut current: Option<Box<Node<T>>> = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
        }
    }
}

#[test]
fn create_new() {
    let list: List<i32> = List::new();
    assert_eq!(list, List {
        head: None,
        size: 0,
    } as List<i32>);
}

#[test]
fn push_and_pop() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

#[test]
fn getters() {
    let mut list: List<i32> = List::new();
    list.push(33);
    list.push(22);
    list.push(11);

    assert_eq!(list.get_size(), 3);
    assert_eq!(list.get_top(), Some(&11));
}

#[test]
fn list_iter() {
    let mut list: List<i32> = List::new();
    list.push(33);
    list.push(22);
    list.push(11);
    let mut it: Iter<i32> = list.iter();
    assert_eq!(it.next(), Some(&11));
    assert_eq!(it.next(), Some(&22));
    assert_eq!(it.next(), Some(&33));
    //Iterator has no influence to the list
    assert_eq!(list.get_top(), Some(&11));
}

#[test]
fn list_iter_mut() {
    let mut list: List<i32> = List::new();
    list.push(33);
    list.push(22);
    list.push(11);
    let mut it: IterMut<i32> = list.iter_mut();
    assert_eq!(it.next(), Some(&mut 11));
    assert_eq!(it.next(), Some(&mut 22));
    assert_eq!(it.next(), Some(&mut 33));
    //Iterator has no influence to the list
    assert_eq!(list.get_top(), Some(&11));
}

