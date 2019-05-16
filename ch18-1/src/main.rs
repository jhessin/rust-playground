fn main() {
    // The versatility of the if let syntax
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!(
            "Using blue as the background color"
        );
    }

    // Using the while let syntax
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // Patterns inside for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!(
            "{} is at index {}",
            value, index
        );
    }

    // a simple let can destructure a tuple using
    // a pattern
    let (_x, _y, _z) = (1, 2, 3);

    // However the number of arguments need to
    // match or be replaced with _ or ..
    let (_x, _y, _) = (1, 2, 3);
    let (_x, ..) = (1, 2, 3, 4, 5, 6);

    // Function parameters are also patterns
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!(
            "Current location: ({}, {})",
            x, y
        );
    }

    let point = (3, 5);
    print_coordinates(&point);
}
