pub fn is_valid(s: String) -> bool {
    let mut open = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => {
                open.push(c);
            }
            ')' => match open.pop() {
                Some('(') => continue,
                _ => return false,
            },
            '}' => match open.pop() {
                Some('{') => continue,
                _ => return false,
            },
            ']' => match open.pop() {
                Some('[') => continue,
                _ => return false,
            },
            _ => unreachable!(),
        }
    }
    open.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid_test0() {
        assert_eq!(is_valid(String::from("()")), true);
    }
    #[test]
    fn is_valid_test1() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }
    #[test]
    fn is_valid_test2() {
        assert_eq!(is_valid(String::from("(]")), false);
    }
}
