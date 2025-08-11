#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All functions defined within an impl block are called associated functions 
// because they’re associated with the type named after the impl
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {rect1:?}");
}

