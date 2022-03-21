pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    // Digits in `x`
    let len = (x as f32).log10();
    // Positions
    let middle = (len / 2f32) as u32;
    let left_range = middle..len as u32 + 1;
    let right_range = 1..middle + 2;
    // Starting values
    let mut right = 0;
    let mut prev_right_power = 1i32;
    // For each pair of digits moving in from the outermost digits.
    for (l, r) in left_range.rev().zip(right_range) {
        // Gets left digit
        let left = (x / 10i32.pow(l)) % 10;

        // Gets right digit
        let temp_right_power = 10i32.pow(r);
        right = ((x % temp_right_power) - right) / prev_right_power;
        prev_right_power = temp_right_power;

        // Checks digits match
        if left != right {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::is_palindrome;
    #[test]
    fn is_palindrome_test0() {
        assert_eq!(is_palindrome(121), true);
    }
    #[test]
    fn is_palindrome_test1() {
        assert_eq!(is_palindrome(-121), false);
    }
    #[test]
    fn is_palindrome_test2() {
        assert_eq!(is_palindrome(123454321), true);
    }
    #[test]
    fn is_palindrome_test3() {
        assert_eq!(is_palindrome(10), false);
    }
    #[test]
    fn is_palindrome_test4() {
        assert_eq!(is_palindrome(11), true);
    }
    // #[test]
    // fn special() {
    //     println!("1: {}",121i32.div_euclid(1));
    //     println!("10: {}",121i32.div_euclid(10));
    //     println!("100: {}",121i32.div_euclid(100));
    //     assert!(false);
    // }
}
