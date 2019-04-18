pub fn main() {
  let v = vec![1, 2, 3];

  // Using Match
  match v.get(1) {
    Some(i) => {
      println!("{}", i);
    }
    None => {}
  }

  // Using if let
  if let Some(i) = v.get(1) {
    println!("{}", i);
  }

  // iterating over the vector
  for i in &v {
    println!("{}", i);
  }

  // Can I print the vector?
  println!("{:?}", v); // YES!

  // Using enums and vectors together to have a vector of different types
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }
}
