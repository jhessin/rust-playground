/* Using a hash map and vectors,
create a text interface to allow a user to add employee names
to a department in a company. For example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then let the user retrieve a list of all people
in a department or all people in the company by department,
sorted alphabetically.
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

  fn dept(&self, department: &str) -> Company{
    let mut new_company = Company::new();
    if let Some(employees) = self.data.get(department) {
      for emp in employees {
        new_company.add(department, emp);
      }
    }
    new_company
  }

  fn departments(&self) -> Vec<String> {
    let mut depts = vec![];
    for (k,v) in &self.data {
      depts.push(k.clone());
    }
    depts
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
    println!("Please enter a comand: (Add), (Display), (Departments), (Quit)");

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
        if let Some(department) = iter.next() {
          println!("{}", company.dept(department));
        } else {
          println!("{}", company);
        }
      } else if cmd.eq_ignore_ascii_case("departments") {
        for dept in company.departments() {
            println!("{}", dept);
        }
      } else if cmd.eq_ignore_ascii_case("quit") {
        println!("Goodbye");
        break;
      }
    }
  }
}
