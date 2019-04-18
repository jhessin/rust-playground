use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// A complex number
#[derive(Debug)]
struct Complex {
  real: f64,
  imaginary: f64,
}

// Implement for fmt::Display
impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.imaginary > 0.0 {
      write!(f, "{0} + {1}i", self.real, self.imaginary)
    } else if self.imaginary < 0.0 {
      write!(f, "{0} - {1}i", self.real, -self.imaginary)
    } else {
      write!(f, "{}", self.real)
    }
  }
}

pub fn main() {
    let minmax = MinMax(0, 14);
  let complex1 = Complex {
    real: 5.0,
    imaginary: 0.0,
  };
  let complex2 = Complex {
    real: 5.0,
    imaginary: 7.0,
  };
  let complex3 = Complex {
    real: 5.0,
    imaginary: -7.0,
  };

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
  println!("Display: {}", complex1);
  println!("Debug: {:?}", complex1);
  println!("Display: {}", complex2);
  println!("Debug: {:?}", complex2);
  println!("Display: {}", complex3);
  println!("Debug: {:?}", complex3);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
