pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    unsafe {
        // Technically best
        // -------------
        let mut front = 0;
        let mut back = nums.len();
        while front < back {
            if *nums.get_unchecked(front) == val {
                *nums.get_unchecked_mut(front) = *nums.get_unchecked(back - 1);
                back -= 1;
            } else {
                front += 1;
            }
        }
        back as i32
        // Close
        // -------------
        // for i in 0..nums.len() {
        //     if *nums.get_unchecked(i) != val {
        //         *nums.get_unchecked_mut(front) = *nums.get_unchecked(i);
        //         front += 1;
        //     }
        // }
        // front as i32

        // Easiest
        // -------------
        // nums.retain(|&x|x!=val);
        // nums.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_element_one() {
        let mut vec = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut vec, 3), 2);
        assert_eq!(&vec[0..2], [2, 2]);
    }
    #[test]
    fn remove_element_two() {
        let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut vec, 2), 5);
        assert_eq!(&vec[0..5], [0, 1, 4, 0, 3]);
    }
}
