pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut longest = String::new();
    let mut front;
    let mut min_length = i32::MAX as usize;
    let bytes = strs
        .iter()
        .map(|s| {
            let b = s.as_bytes();
            min_length = std::cmp::min(min_length, b.len());
            b
        })
        .collect::<Vec<_>>();

    for i in 0..min_length {
        front = bytes[0][i];
        for j in 1..bytes.len() {
            if bytes[j][i] != front {
                return longest;
            }
        }
        longest.push(front as char);
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_common_prefix_test_1() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
    }
    #[test]
    fn longest_common_prefix_test_2() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::new()
        );
    }
}
