// This is an example from the book section 5.2
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn main() {
    // The long way
    let width = 30;
    let height = 50;

    // The tuple way
    // let rect1 = (30, 50);

    // The Struct way
    let rect1 = Rectangle { width, height };
    let rect2 = Rectangle::square(25);

    if rect1.can_hold(&rect2) {
        println!("Rect1: {:?} can hold Rect2: {:?}", rect1, rect2);
    } else {
        println!("Rect1 cannot hold Rect2");
    }

    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    println!(
        "The area of the rectangle: {:?} is {} square pixels.",
        &rect1,
        rect1.area()
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
