#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        (self.width > another.width && self.height > another.height)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(20);
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Area of rect1: {}", rect1.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width: 20, height: 10};
        let smaller = Rectangle{width: 10, height: 8};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_smaller() {
        let larger = Rectangle{width: 8, height: 2};
        let smaller = Rectangle{width: 2, height: 2};

        assert!(!smaller.can_hold(&larger));
    }
}