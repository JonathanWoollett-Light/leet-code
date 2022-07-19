pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(x) | Err(x) => x as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_insert_one() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(search_insert(vec, 5), 2);
    }
    #[test]
    fn search_insert_two() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(search_insert(vec, 2), 1);
    }
    #[test]
    fn search_insert_three() {
        let vec = vec![1, 3, 5, 6];
        assert_eq!(search_insert(vec, 7), 4);
    }
}
