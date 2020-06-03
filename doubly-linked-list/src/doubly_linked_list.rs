#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::{cmp, mem};
use std::fmt::Debug;
mod list;
use list::List;

// Doubly Linked List
#[derive(Debug)]
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

    fn check_size(&mut self,index:i32){
        if(self.size()==0 || self.size()<=index || index<0){
            panic!("Index Out of Bounds!!!");
        }
    }
    fn check_empty(&mut self){
        if(self.size()==0){
            panic!("Doubly-Linked-List is Empty!!!");
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
        while (self.left.get_size() != index + 1) {
            if (index + 1 > self.left.get_size()) {
                self.next();
            } else {
                self.previous();
            }
        }
    }

    pub fn push(&mut self, elem: T, index: i32) {
        self.check_size(index);
        self.shift(index - 1);
        self.left.push(elem);
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        self.shift(*self.get_size().unwrap() - 1);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn pop(&mut self, index: i32) {
        self.check_size(index);
        self.shift(index);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn get_size(&self) -> Option<&i32> {
        Some(&self.size)
    }

    pub fn get_current(&mut self) -> Option<&T> {
        self.left.get_top()
    }

    pub fn get_current_position(&mut self) -> i32 {
        self.left.get_size()
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

    pub fn size(&mut self) -> i32 {
        *self.get_size().unwrap()
    }

    pub fn sort (&mut self)
        where T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug
    {
        if (self.size() == 0){
            return;
        }
        self.shift(0);
        let mut exp = 2;
        let mut buffer:List<T> = List::new();
        let size = self.size();
        while ((exp>>1)  < size ){
            let init = (exp>>1) - 1;
            self.shift(init);
            for idx in init..size {
                self.shift(idx);
                if( (idx - init) % exp == 0){
                    let mut l = self.left.pop();
                    let mut r = self.right.pop();
                    let mut q = (exp>>1);
                    while (l>=r  && q>0){
                        match l.take(){
                            Some(value) => {
                                buffer.push(value);
                            },
                            None => {
                                break;
                            }
                        }
                        l = self.left.pop();
                        q-=1;
                    }
                    if(l!=None){
                        l.take().map(|value|{self.left.push(value);});
                    }
                    let mut b = buffer.pop();
                    q = (exp>>1);
                    while(b!=None){
                        if(q>0 && r!=None && r<b){
                            match r.take(){
                                Some(value) => {
                                    self.left.push(value);
                                },
                                None => {
                                    break;
                                }
                            }
                            r=self.right.pop();
                            q-=1;
                        }
                        else{
                            match b.take(){
                                Some(value) => {
                                    self.left.push(value);
                                },
                                None => {
                                    break;
                                }
                            }
                            b = buffer.pop();
                        }
                    }
                    if(r!=None){
                        r.take().map(|value|{self.right.push(value);});
                    }
                }

            }
            // self.print_all();
            exp = exp<<1;
        }
    }
    pub fn reverse(&mut self){
        mem::swap(&mut self.left,&mut self.right);
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

    pub fn print_fmt(&mut self, separator : char)
        where T: std::fmt::Debug
    {
        self.check_empty();
        let sz = self.size();
        for x in 0..sz {
            print!("{:?}{}", self.get(x),separator);
        }
        if(separator!='\n') {
            println!();
        }
    }

}