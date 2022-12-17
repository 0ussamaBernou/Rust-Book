#[derive(Debug)]

// struct instantiation
struct Rectangle {
    width: u32,
    height: u32,
}
// a method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// associated function
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the area of this rectangle {:?} is: {}", rec1, rec1.area()); // {:#?} for pretty print
    let square1 = Rectangle::square(3);
    // pretty print square1
    println!("square1: {:#?}", square1);
}

fn _area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
