#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }   
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        }
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }
        Guess { value }
    }
}

 fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

   // #[test]
   // fn failed_test() {
   //     assert_eq!(3, 4, "The two values do not equal eachother");
   // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 20,
            height: 30,
        };
        let smaller = Rectangle {
            width: 10,
            height: 15,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 20,
            height: 30,
        };
        let smaller = Rectangle {
            width: 10,
            height: 15,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
    
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn guess_less_than_1() {
        Guess::new(-1);
    }

    #[test]
    fn valid_guess() {
        Guess::new(40);
    }
}
