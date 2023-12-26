#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.height * self.width
    }
    
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect2 = Rectangle {
        height: 40,
        width: 10,
    };

    let rect3 = Rectangle {
        height: 45,
        width: 60,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("{:#?}", &sq.area());
    println!("{}", sq.height);
}
