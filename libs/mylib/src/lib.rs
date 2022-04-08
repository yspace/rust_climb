//! This is my module documentation . My library is so nice!


/// four() is a function that returns `4`
/// 
/// ```
/// use mylib::four;
/// let x = 4;
/// assert_eq!(x, four());
/// ```
pub fn four() -> i32{4}

#[cfg(test)]
mod tests {
    // use crate::four;
    use super::four;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, four());
    }
}
