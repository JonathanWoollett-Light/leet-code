/// Results vary wildly
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    unsafe {
        for i in (0..digits.len()).rev() {
            if *digits.get_unchecked(i) == 9 {
                *digits.get_unchecked_mut(i) = 0;
            }
            else {
                *digits.get_unchecked_mut(i) += 1;
                return digits;
            }
        }
        digits.insert(0,1);
        digits
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plus_one_one() {
        assert_eq!(plus_one(vec![1,2,3]),vec![1,2,4]);
    }
    #[test]
    fn plus_one_two() {
        assert_eq!(plus_one(vec![4,3,2,1]),vec![4,3,2,2]);
    }
    #[test]
    fn plus_one_three() {
        assert_eq!(plus_one(vec![9]),vec![1,0]);
    }
}