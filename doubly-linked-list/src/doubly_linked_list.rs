#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::{cmp, mem};
use std::fmt::Debug;

mod list;

use list::List;
use crate::doubly_linked_list::list::Drop as list_drop;

// Doubly Linked List
#[derive(Debug, PartialEq, PartialOrd)]
pub struct DoublyLinkedList<T> {
    left: List<T>,
    right: List<T>,
    size: i32,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        let mut left_list = List::new();
        let mut right_list = List::new();
        DoublyLinkedList {
            left: left_list,
            right: right_list,
            size: 0,
        }
    }

    pub fn get_size(&self) -> Option<&i32> {
        Some(&self.size)
    }

    pub fn size(&self) -> i32 {
        *self.get_size().unwrap()
    }

    pub fn get_current(&mut self) -> Option<&T> {
        self.left.get_top()
    }

    pub fn get_current_position(&mut self) -> i32 {
        self.left.get_size() - 1
    }

    fn empty(&self) -> bool {
        let res: bool = self.size() == 0;
        res
    }

    fn check_empty(&mut self) {
        if (self.empty()) {
            panic!("Doubly-Linked-List is Empty!!!");
        }
    }

    fn check_size(&mut self, index: i32) {
        self.check_empty();
        if (self.size() <= index || index < 0) {
            panic!("Index Out of Bounds!!!");
        }
    }

    pub fn next(&mut self) {
        if (self.right.get_size() != 0) {
            self.right.pop().map(|node| { self.left.push(node); });
        }
    }

    pub fn previous(&mut self) {
        if (self.left.get_size() != 0) {
            self.left.pop().map(|node| { self.right.push(node); });
        }
    }

    pub fn push_back(&mut self, elem: T) {
        while (self.right.get_size() != 0) {
            self.next();
        }
        self.left.push(elem);
        self.size += 1;
    }

    pub fn shift(&mut self, index: i32) {
        while (self.get_current_position() != index) {
            if (index > self.get_current_position()) {
                self.next();
            } else {
                self.previous();
            }
        }
    }

    pub fn push(&mut self, elem: T, index: i32) {
        if (index == self.size()) {
            self.shift(index - 1);
            self.right.push(elem);
        } else {
            self.check_size(index);
            self.shift(index - 1);
            self.left.push(elem);
        }
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        self.shift(self.size() - 1);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn pop(&mut self, index: i32) {
        self.check_size(index);
        self.shift(index);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn get(&mut self, index: i32) -> &T {
        self.check_size(index);
        self.shift(index);
        self.left.get_top().unwrap()
    }

    pub fn edit_current(&mut self, value: T) {
        self.left.pop();
        self.left.push(value);
    }

    pub fn edit(&mut self, index: i32, value: T) {
        self.check_size(index);
        self.shift(index);
        self.left.pop();
        self.left.push(value);
    }

    pub fn sort(&mut self)
        where T: std::cmp::PartialEq + std::cmp::PartialOrd
    {
        if (self.empty()) {
            return;
        }
        self.shift(0);
        let mut exp = 2;
        let mut buffer: List<T> = List::new();
        let size = self.size();
        while ((exp >> 1) < size) {
            let init = (exp >> 1) - 1;
            self.shift(init);
            for idx in init..size {
                self.shift(idx);
                if ((idx - init) % exp == 0) {
                    {
                        let mut left_element = self.left.pop();
                        let mut right_element = self.right.pop();
                        let mut half_segment_length = (exp >> 1);
                        while (right_element != None && left_element >= right_element && half_segment_length > 0) {
                            match left_element.take() {
                                Some(value) => {
                                    buffer.push(value);
                                }
                                None => {
                                    break;
                                }
                            }
                            left_element = self.left.pop();
                            half_segment_length -= 1;
                        }
                        if (left_element != None) {
                            left_element.take().map(|value| { self.left.push(value); });
                        }
                        let mut buffer_element = buffer.pop();
                        half_segment_length = (exp >> 1);
                        while (buffer_element != None) {
                            if (half_segment_length > 0 && right_element != None && right_element < buffer_element) {
                                match right_element.take() {
                                    Some(value) => {
                                        self.left.push(value);
                                    }
                                    None => {
                                        break;
                                    }
                                }
                                right_element = self.right.pop();
                                half_segment_length -= 1;
                            } else {
                                match buffer_element.take() {
                                    Some(value) => {
                                        self.left.push(value);
                                    }
                                    None => {
                                        break;
                                    }
                                }
                                buffer_element = buffer.pop();
                            }
                        }
                        if (right_element != None) {
                            right_element.take().map(|value| { self.right.push(value); });
                        }
                    }
                }
            }
            exp = exp << 1;
        }
    }
    pub fn reverse(&mut self) {
        mem::swap(&mut self.left, &mut self.right);
    }
    pub fn print_line(&mut self)
        where T: std::fmt::Debug
    {
        self.check_empty();
        let sz = self.size();
        for x in 0..sz {
            print!("{:?} ", self.get(x));
        }
        println!();
    }

    pub fn print_fmt(&mut self, separator: char)
        where T: std::fmt::Debug
    {
        self.check_empty();
        let sz = self.size();
        for x in 0..sz {
            print!("{:?}{}", self.get(x), separator);
        }
        if (separator != '\n') {
            println!();
        }
    }
}

pub trait Drop {
    fn drop(&mut self);
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        self.left.drop();
        self.right.drop();
        self.size = 0;
    }
}