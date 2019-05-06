fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = &item;
    }
  }

  largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

use std::fmt::{Display, Formatter, Error};

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
  where T: Display
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}


#[derive(Debug)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> Display for ImportantExcerpt<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "Important!: {}", self.part)
  }
}

pub fn main() {
  // GENERICS!
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);

  // LIFETIMES!

  let string1 = String::from("long string is long");
  let result;

  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }

  // LIFETIMES in Structs
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
  let i = ImportantExcerpt { part: first_sentence };
  println!("{}", i);

  // Putting it all together!
  let longest = longest_with_an_announcement("I am longer!", "No! I'm longer!", i);
  println!("And the longest is...: {}", longest);
}
