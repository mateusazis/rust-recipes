/// Adds two numbers
///
/// ```
/// assert_eq!(rust_recipes::util::testing::sum(9,4), 13);
/// ```
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// unit tests
#[cfg(test)]
mod tests {

    #[test]
    fn test_sum_returns_the_sum() {
        assert_eq!(crate::util::testing::sum(8, 5), 13);
    }
}
