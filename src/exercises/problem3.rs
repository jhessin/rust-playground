/* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;
use std::fmt::{ Display, Formatter, Result as FmtResult };

struct Department {
  name: String,
  employees: Vec<String>,
}

impl Display for Department {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}")
  }
}

struct Company {
  data: HashMap<String, Department>
}

impl Display for Company {
  fn add(&mut self, department: &str, employee: &str) {
    let mut Company{data} = self;

    let mut entry = data.entry(department).or_insert(Vec::new());
    entry.push(employee);
  }

  fn fmt(&self, f: &mut Formatter) -> FmtResult {

  }
}
pub fn main() {

}