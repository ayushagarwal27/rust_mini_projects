fn main() {
    let rect = Rectangle {
        width: 20,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("rect1 can hold rect2: {}", rect.can_hold(&rect2));
    println!("{:#?}", Rectangle::square(2));

    // print!("{:#?}", rect);
    // dbg!(&rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}
