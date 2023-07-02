#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn width(&self) -> bool {
        return self.width > 0;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Rectangle {
    // Associated functions that aren’t methods are often used for constructors
    // that will return a new instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 40,
        height: 65,
    };

    let square1 = Rectangle::square(3);

    print!("Can rect1 hold rect2 -> {}", rect1.can_hold(&rect2));
    print!("Can rect1 hold rect3 -> {}", rect1.can_hold(&rect3));

    print!("Is square with size 3 -> {:#?}", square1);

    dbg!(&rect1);

    if rect1.width() {
        print!("rectangle has no zero width with value {}", rect1.width);
    }

    println!(
        "The area of rectangle is {:#?} square pixels.",
        rect1.area()
    )
}
