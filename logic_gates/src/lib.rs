mod generic_structs;

/// Implements a boolean `and` gate taking as input two bites and
/// returns a bit as output

pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0,
    }
}

/// Implememts a boolean `xor` gate taking two bites and returning a
/// bit as output
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0,
    }
}

pub fn use_container() {
    let _cont = generic_structs::Container::new("Test");
}
/// A simple generic function
pub fn give_me<T>(value: T) {
    let _ = value;
}

#[cfg(test)]
mod tests {
    use crate::{and, give_me, xor};

    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0, 0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_generic() {
        let a = "generics";
        let b = 1024;
        give_me(a);
        give_me(b);
    }
}
