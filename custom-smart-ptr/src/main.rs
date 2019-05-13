struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: "my stuff".to_owned(),
    };
    let _d = CustomSmartPointer {
        data: "other stuff".to_owned(),
    };
    println!("CustomSmartPointers created");
    drop(_c);
    println!("CustomSmartPointer dropped before the end of main.")
}
