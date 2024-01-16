struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    // let integer = Point { x: 10, y: 20 };
    // let float = Point { x: 10.1, y: 20.1 };
    // let both = Point { x: 10, y: 20.1 };
    // println!("The x value is {}", integer.x());
    // println!("The y value is {}", integer.y());
    // println!("The x value is {}", both.x_i32());
    // println!("The y value is {}", both.y_f32());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello World", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("The x value is {}", p3.x);
    println!("The y value is {}", p3.y);
}

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }
//
//     fn y(&self) -> &U {
//         &self.y
//     }
// }
//
// impl Point<i32, f32> {
//     fn x_i32(&self) -> &i32 {
//         &self.x
//     }
//
//     fn y_f32(&self) -> &f32 {
//         &self.y
//     }
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
