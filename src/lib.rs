//! # My Crate
//! this is a crate description

/// Adds one to the number given
/// # Examples
/// ```
/// let a = 1;
/// let b = 2;
/// assert_eq!(3, type_doc::add(a, b))
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
