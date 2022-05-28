// This time, we want an instance of Rectangle to take another instance of Rectangle
// and return true if the second Rectangle can fit completely within self
// (the first Rectangle); otherwise it should return false. That is, once
// we’ve defined the can_hold method, we want to be able to write the program shown in Listing 5-14

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// multiple implementations can exist for a single struct

impl Rectangle {
    // associated function, not a method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // a method
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::square(5);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("rect1 area is {}", rect1.area());
    println!("rect2 area is {}", rect2.area());
    println!("rect4 area is {}", rect4.area());
}
