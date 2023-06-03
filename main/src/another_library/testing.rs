/// Adds two numbers
///
/// ```
/// use another_library::testing::sum;
///
/// assert_eq!(sum(9,4), 13);
/// ```
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// unit tests
#[cfg(test)]
mod tests {

    #[test]
    fn test_sum_returns_the_sum() {
        assert_eq!(super::sum(8, 5), 13);
    }
}
