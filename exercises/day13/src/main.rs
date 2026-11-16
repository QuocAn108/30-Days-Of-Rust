fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
    #[test]
    fn it_fails() {
        assert!(false);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_panics_with_message() {
        assert_eq!("Hello", "world");
    }
}
