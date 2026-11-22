
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width:5, height:10};
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!("Area of rect1 is {}", rect1.area());
    let rect2 = Rectangle {width:3, height:5};
    let rect3 = Rectangle {width:5, height:10};

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("square is {:#?}", square);
}
