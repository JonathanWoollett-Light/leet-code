pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;

    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        sum += match (c, iter.peek()) {
            ('I', Some('V')) => {
                iter.next().unwrap();
                4
            }
            ('I', Some('X')) => {
                iter.next().unwrap();
                9
            }
            ('X', Some('L')) => {
                iter.next().unwrap();
                40
            }
            ('X', Some('C')) => {
                iter.next().unwrap();
                90
            }
            ('C', Some('D')) => {
                iter.next().unwrap();
                400
            }
            ('C', Some('M')) => {
                iter.next().unwrap();
                900
            }
            _ => value(c),
        };
    }
    sum
}
const fn value(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => unreachable!(),
    }
}
// const fn lookup(a: char, b: )

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roman_to_int_test_1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }
    #[test]
    fn roman_to_int_test_2() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }
    #[test]
    fn roman_to_int_test_3() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
