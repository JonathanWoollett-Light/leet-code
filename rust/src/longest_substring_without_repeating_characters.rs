/// Runtime: 0ms faster than 100%
/// Memory usage: 2.2mb, less than 73.75%
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut found = [None; 256];
    let mut start = 0;
    let mut max = 0;
    for (i, c) in s.char_indices() {
        // found.contains_key(&s[i])
        if let Some(index) = &mut found[c as usize] {
            // If the last instance of the character duplicate of `s[i]` is before our current
            //  sequence we don't need to do anything than update the last instance index.
            if *index >= start {
                max = std::cmp::max(i - start, max);
                start = *index + 1;
            }
            *index = i;
        } else {
            found[c as usize] = Some(i);
            // found.insert(c,i);
        }
    }
    std::cmp::max(max, s.len() - start) as i32
}
#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;
    #[test]
    fn test1() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
    #[test]
    fn test4() {
        assert_eq!(length_of_longest_substring(String::from("abba")), 2);
    }
    #[test]
    fn test5() {
        assert_eq!(length_of_longest_substring(String::from("abcdef")), 6);
    }
    #[test]
    fn test6() {
        assert_eq!(length_of_longest_substring(String::from(" ")), 1);
    }
    #[test]
    fn test7() {
        assert_eq!(length_of_longest_substring(String::from("")), 0);
    }
}
