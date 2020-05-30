use std::cmp;
use std::fmt::Debug;
use std::ops::Deref;

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
        if(self.size()==0 || self.size()<=index){
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
        while (self.left.get_size() != index + 1) {
            if (index + 1 > self.left.get_size()) {
                self.next();
            } else {
                self.previous();
            }
        }
    }

    pub fn push(&mut self, elem: T, index: i32) {
        self.shift(index);
        self.left.push(elem);
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        self.shift(*self.get_size().unwrap() - 1);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn pop(&mut self, index: i32) {
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
        self.shift(0);
        let mut exp = 2;
        let mut buffer:List<T> = List::new();
        let size = self.size();
        while ((exp>>1) - 1 < size - 1){
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
    pub fn print_all(&mut self)
        where T: std::fmt::Debug
    {
        let sz = self.size();
        for x in 0..sz {
            print!("{:?} ", self.get(x));
        }
        println!();
    }

}

pub fn print_dll(dl: &mut DoublyLinkedList<i32>) {
    let sz = dl.size();
    for x in 0..sz {
        print!("{:?} ", dl.get(x));
    }
}

#[derive(Debug,PartialEq, PartialOrd)]
pub struct Person {
    name: String,
    age: u8,
    height : u8
}
fn main() {

    let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
    dl.push_back(5);
    dl.push_back(3);
    dl.push_back(6);
    dl.push_back(7);
    dl.push_back(4);
    dl.push_back(2);
    dl.push_back(1);
    dl.push_back(1123);
    dl.push_back(231);
    dl.push_back(1234);
    dl.push_back(441);
    dl.push_back(651);
    dl.push_back(731);
    dl.push_back(14);
    dl.push_back(51);
    dl.push_back(771);
    dl.push_back(8881);
    dl.push_back(31);
    dl.sort();
    dl.print_all();

    // let person1 = Person{
    //     name:String::from("John"),
    //     age:25,
    //     height:2
    // };
    // let person2 = Person{
    //     name:String::from("John"),
    //     age:25,
    //     height:2
    // };

    // let mut dl: DoublyLinkedList<Person> = DoublyLinkedList::new();
    // dl.push_back(Person {
    //     name: "nick",
    //     age: 15,
    // });
    // dl.push_back(Person {
    //     name: "John",
    //     age: 22,
    // });
    // dl.push_back(Person {
    //     name: "Pablo",
    //     age: 33,
    // });
    // dl.push_back(Person {
    //     name: "Stefan",
    //     age: 14,
    // });
    // dl.push_back(Person {
    //     name: "Jose",
    //     age: 88,
    // });
    // dl.push_back(Person {
    //     name: "Jake",
    //     age: 44,
    // });
}


