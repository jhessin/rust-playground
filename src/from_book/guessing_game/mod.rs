pub fn main() {
    let s = String::from("This is a string");

    println!("The string is {}", s);

    // Can explicitly clone the object
    take_ownership(s.clone());

    println!("The string is {}", s);

    // can return the object
    let s = return_ownership(s);

    println!("The string is {}", s);

    // can return the object as part of a tuple
    let (s, length) = calculate_length(s);

    println!("{} is {} long", s, length);

    // can pass a reference instead
    let length = calculate_length_ref(&s);

    println!("{} is {} long", s, length);

    // If we want to change a reference we need to pass a mutable version of it
    let mut s = s;
    change(&mut s);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn return_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push('s');
}
