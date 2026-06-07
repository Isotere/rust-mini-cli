#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_of_rectangle() {
        let rect = Rectangle::new(4, 3);
        assert_eq!(rect.area(), 12);
    }

    #[test]
    fn new_square_has_equal_sides() {
        let sq = Rectangle::new_square(5);
        assert_eq!(sq.area(), 25);
    }

    #[test]
    fn area_with_zero_side_is_zero() {
        let rect = Rectangle::new(0, 7);
        assert_eq!(rect.area(), 0);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let bigger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        assert!(bigger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle::new(5, 1);
        let bigger = Rectangle::new(8, 7);
        assert!(!smaller.can_hold(&bigger));
    }

    #[test]
    fn equal_rectangles_cannot_hold() {
        // can_hold strict (>), равные не помещаются.
        let a = Rectangle::new(4, 4);
        let b = Rectangle::new(4, 4);
        assert!(!a.can_hold(&b));
    }
}
