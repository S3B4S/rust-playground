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
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40,
    };

    println!(
        "I can hold it: {}",
        rect1.can_hold(&rect3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_can_hold_smaller() {
        let largest = Rectangle { width: 30, height: 50 };
        let smaller = Rectangle { width: 20, height: 30 };

        assert!(largest.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let largest = Rectangle { width: 30, height: 50 };
        let smaller = Rectangle { width: 20, height: 30 };

        assert!(!smaller.can_hold(&largest));
    }

}