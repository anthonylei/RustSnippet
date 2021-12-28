#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// benefit of using method is much cleaner
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associate function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            width: size, 
            height: size 
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect: {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        // automate referencing and deferencing
        rect.area()
    );

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    println!("rect can hold rect1: {}", rect2.can_hold(&rect1))
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }