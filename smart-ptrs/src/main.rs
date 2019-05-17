use std::ops::Deref;

#[allow(dead_code)]
#[allow(unused_variables)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    #[allow(dead_code)]
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use crate::List::*;

    // Box can be used like anything else
    let b = Box::new(5);
    println!("b = {}", b);

    // using List
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    #[test]
    fn box_test() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn coercion_test() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}
