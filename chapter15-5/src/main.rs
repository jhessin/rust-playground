use std::cell::RefCell;
use std::rc::Rc;

use crate::List::*;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // Self musing (using shadowing and ownership
    // to modify mutability)
    let x = 5;
    let mut x = x;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    let x = x;
    println!("{}", x);

    // Back to the book...
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(
        Rc::clone(&value),
        Rc::new(Nil),
    ));

    let b = Cons(
        Rc::new(RefCell::new(6)),
        Rc::clone(&a),
    );
    let c = Cons(
        Rc::new(RefCell::new(10)),
        Rc::clone(&a),
    );

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!(
        "value count after = {}",
        Rc::strong_count(&value)
    );
    println!(
        "a count after = {}",
        Rc::strong_count(&a)
    );
}

pub trait Messenger {
    fn send(
        &self,
        msg: &str,
    );
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(
        messenger: &T,
        max: usize,
    ) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(
        &mut self,
        value: usize,
    ) {
        self.value = value;

        let percentage_of_max =
            self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send(
                "Error: you are over your quota!",
            );
        } else if percentage_of_max >= 0.9 {
            self.messenger.send(
                "Urgent warning: You've used up over 90% of \
                 your quota!",
            );
        } else if percentage_of_max >= 0.75 {
            self.messenger.send(
                "Warning: You've used up over 75% of your \
                 quota!",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(
                    vec![],
                ),
            }
        }
    }

    impl Messenger for MockMessenger {
        /// NOTE: This doesn't work
        ///
        /// ```
        /// let mut one_borrow = self.sent_messages.borrow_mut();
        /// // let mut two_borrow = self.sent_messages.borrow_mut();
        ///
        /// one_borrow.push(String::from(msg));
        /// // two_borrow.push(String::from(msg));
        /// ```
        fn send(
            &self,
            msg: &str,
        ) {
            self.sent_messages
                .borrow_mut()
                .push(msg.to_owned());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message(
    ) {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100,
        );

        limit_tracker.set_value(80);

        assert_eq!(
            mock_messenger
                .sent_messages
                .borrow()
                .len(),
            1
        );
    }
}
