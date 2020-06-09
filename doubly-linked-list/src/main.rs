use std::fmt::Debug;

mod doubly_linked_list;

use doubly_linked_list::DoublyLinkedList;
use crate::doubly_linked_list::Drop;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Person {
    name: String,
    age: u8,
    height: f32,
}

fn main() {
    let mut dl: DoublyLinkedList<Person> = DoublyLinkedList::new();
    dl.push_back(Person {
        name: String::from("John"),
        age: 15,
        height: 1.7,
    });
    dl.push_back(Person {
        name: String::from("John"),
        age: 25,
        height: 2.0,
    });
    dl.push_back(Person {
        name: String::from("John"),
        age: 18,
        height: 2.0,
    });
    dl.push(Person {
        name: String::from("Emily"),
        age: 12,
        height: 1.25,
    }, 3);
    //test 1
    println!("The first element: {:?}",dl.get(0));
    //test 2
    println!("Initial order:");
    dl.print_line();
    //test 3
    dl.reverse();
    println!("Reversed order:");
    dl.print_line();
    //test 4
    dl.sort();
    println!("Sorted order:");
    dl.print_fmt('\n');
    //test 5
    dl.pop_back();
    println!("after popping back:");
    dl.print_fmt('\n');
    //test 6
    dl.pop(1);
    println!("after popping 1st:");
    dl.print_fmt('\n');
    //test 7
    dl.drop();
    println!("{}",dl.size());
}


