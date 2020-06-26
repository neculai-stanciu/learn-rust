//!
//! # Examples
//! ```
//! use integration_test::sum;
//! let work_a = 4;
//! let work_b = 34;
//! let total_work = sum(work_a, work_b);

/// Sum two arguments
/// # Examples
///
/// ```
/// assert_eq!(integration_test::sum(1,1),2);
/// ```
pub fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sum_should_be_ok() {
        assert_eq!(crate::sum(2, 3), 5)
    }
}
