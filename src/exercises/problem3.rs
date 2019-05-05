/* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Company {
  data: HashMap<String, Vec<String>>,
}

impl Company {
  fn new() -> Company {
    Company {
      data: HashMap::new(),
    }
  }

  fn add(&mut self, department: &str, name: &str) {
    let department = department.to_owned();
    let name = name.to_owned();

    let entry = self.data.entry(department).or_insert(vec![]);
    entry.push(name);
  }
}

impl Display for Company {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for (k, v) in &self.data {
      writeln!(f, "{}", &k)?;
      let mut v = v.clone();
      v.sort();
      for name in v {
        writeln!(f, "\t{}", &name)?;
      }
    }
    write!(f, "")
  }
}
pub fn main() {
  use std::io;
  let mut company = Company::new();

  loop {
    println!("Please enter a comand: (Add), (Display), (Quit)");

    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Error reading line");

    let mut iter = input.split_whitespace();
    if let Some(cmd) = iter.next() {
      if cmd.eq_ignore_ascii_case("add") {
        if let Some(department) = iter.next() {
          for name in iter {
            company.add(department, name);
            println!("{} added to {}", name, department);
          }
        }
      } else if cmd.eq_ignore_ascii_case("display") {
        println!("{}", company);
      } else if cmd.eq_ignore_ascii_case("quit") {
        println!("Goodbye");
        break;
      }
    }
  }
}
