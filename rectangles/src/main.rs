#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn diagonal(&self) -> f32 {
        let pytha = (self.width.pow(2) + self.height.pow(2)) as f32;
        pytha.sqrt()
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Diagonal: {}", rectangle.diagonal());
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

    #[test]
    fn diagonal_correct() {
        let rectangle = Rectangle { width: 24, height: 7 };

        assert_eq!(rectangle.diagonal(), 25 as f32);
    }
}