/// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Length of Last Word.
/// Memory Usage: 2.2 MB, less than 26.26% of Rust online submissions for Length of Last Word.
pub fn length_of_last_word(s: String) -> i32 {
    unsafe {
        let bytes = s.as_bytes();
        let mut i = s.len();
        while *bytes.get_unchecked(i-1) == 32 {
            i -= 1;
        }
        let end = i;
        for j in (0..i).rev() {
            if *bytes.get_unchecked(j) == b' ' {
                return (end - (j + 1)) as i32;
            }
        }
        end as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_of_last_word_one() {
        assert_eq!(length_of_last_word(String::from("a")), 1);
    }
    #[test]
    fn length_of_last_word_two() {
        assert_eq!(length_of_last_word(String::from("Hello World")), 5);
    }
    #[test]
    fn length_of_last_word_three() {
        assert_eq!(
            length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
    }
    #[test]
    fn length_of_last_word_four() {
        assert_eq!(
            length_of_last_word(String::from("luffy is still joyboy")),
            6
        );
    }
    #[test]
    fn length_of_last_word_five() {
        assert_eq!(length_of_last_word(String::from("a ")), 1);
    }
}
