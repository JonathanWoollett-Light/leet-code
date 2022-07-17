/// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Remove Duplicates from Sorted Array.
/// Memory Usage: 2.4 MB, less than 7.78% of Rust online submissions for Remove Duplicates from Sorted Array.
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    unsafe {
        let mut front = 0;
        for i in 1..nums.len() {
            if nums.get_unchecked(i) != nums.get_unchecked(front) {
                front += 1;
                *nums.get_unchecked_mut(front) = *nums.get_unchecked(i);
            }
        }
        (front+1) as i32
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn remove_duplicates_one() {
        let mut vec = vec![1,1,2];
        assert_eq!(remove_duplicates(&mut vec),2);
        assert_eq!(&vec[0..2],[1,2]);
    }
    #[test]
    fn remove_duplicates_two() {
        let mut vec = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(remove_duplicates(&mut vec),5);
        assert_eq!(&vec[0..5],[0,1,2,3,4]);
    }
}