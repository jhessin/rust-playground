use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::List::*;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn new(
        item: i32,
        list: RefCell<Rc<List>>,
    ) -> RefCell<Rc<List>> {
        RefCell::new(Rc::new(Cons(item, list)))
    }

    fn nil() -> RefCell<Rc<List>> {
        RefCell::new(Rc::new(Nil))
    }

    fn clone(
        list: &Rc<List>
    ) -> RefCell<Rc<List>> {
        RefCell::new(Rc::clone(list))
    }

    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    ) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent,
            children,
        })
    }
}

fn main() {
    // A list with a memory leak
    let a = List::new(5, List::nil());
    let a = a.borrow();

    println!(
        "a initial rc count = {}",
        Rc::strong_count(&a)
    );
    println!("a next item = {:?}", a.tail());

    let b = List::new(10, List::clone(&a));
    let b = b.borrow();

    println!(
        "a rc count after b creation = {}",
        Rc::strong_count(&a)
    );
    println!(
        "b initial rc count = {}",
        Rc::strong_count(&b)
    );
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!(
        "b rc count after changing a = {}",
        Rc::strong_count(&b)
    );
    println!(
        "a rc count after changing a = {}",
        Rc::strong_count(&a)
    );

    // uncomment for a crash!
    //    println!("a next item = {:?}",
    // a.tail());

    // Using the new Node system
    let leaf = Node::new(
        3,
        RefCell::new(Weak::new()),
        RefCell::new(vec![]),
    );

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Node::new(
            5,
            RefCell::new(Weak::new()),
            RefCell::new(vec![Rc::clone(&leaf)]),
        );

        *leaf.parent.borrow_mut() =
            Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
