/* Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

struct Integers {
  data: Vec<i32>,
}

impl Integers {
  fn mean(&self) -> i32 {
    let mut total = 0;
    for i in &self.data {
      total += i;
    }
    if self.data.len() == 0 {
      total
    } else {
    total / self.data.len() as i32
    }
  }

  fn median(&self) -> i32 {
    let mut data = self.data.clone();
    data.sort();
    let mid = self.data.len() / 2;
    if let Some(result) = data.get(mid) {
      *result
    } else {
      0
    }
  }

  fn mode(&self) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let data = &self.data;

    for i in data {
      let count = map.entry(i).or_insert(0);
      *count += 1;
    }
    let map = map;
    let mut max = (0, 0);
    for (k, v) in map {
      if v > max.1 {
        max = (*k, v)
      }
    }
    max.0
  }
}

pub fn main() {
  use std::io;
  let mut data: Vec<i32> = vec![]; //50, 85, 25, 65, 85];

  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if let Ok(i) = input.parse::<i32>() {
      data.push(i);
    } else {
      break;
    }
  }

  let test = Integers { data };

  println!("The mean is: {}", test.mean());
  println!("The median is: {}", test.median());
  println!("The mode is: {}", test.mode());
}
