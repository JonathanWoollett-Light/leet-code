pub fn my_atoi(s: String) -> i32 {
    let mut digits = s.chars().skip_while(|&c| c == ' ').peekable();

    let mut index_in = false;
    let mut digits_vec = Vec::new();
    let mut neg = false;
    while let Some(next) = digits.next() {
        let d = next as u8;
        match d {
            48..=57 => {
                index_in = true;
                digits_vec.push(d - 48);
            }
            // -
            45 => {
                if index_in {
                    break;
                }
                // If the following character is a digit
                if let Some(following) = digits.peek() {
                    let t = *following as u8;
                    if t >= 48 && t <= 57 {
                        neg = true;
                        index_in = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            // +
            43 => {
                if index_in {
                    break;
                }
                if let Some(following) = digits.peek() {
                    let t = *following as u8;
                    if t >= 48 && t <= 57 {
                        neg = false;
                        index_in = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
    // println!("digits_vec: {:?}", digits_vec);
    let sum = digits_vec
        .into_iter()
        .rev()
        .enumerate()
        .fold(0i32, |acc, (i, c)| {
            let temp = (c as i32).saturating_mul(10i32.saturating_pow(i as u32));
            match neg {
                true => acc.saturating_sub(temp),
                false => acc.saturating_add(temp),
            }
        });
    // println!("sum: {}", sum);
    sum
}
#[cfg(test)]
mod tests {
    use super::my_atoi;
    #[test]
    fn test0() {
        assert_eq!(my_atoi(String::from("42")), 42);
    }
    #[test]
    fn test1() {
        assert_eq!(my_atoi(String::from("  000123asd2")), 123);
    }
    #[test]
    fn test2() {
        assert_eq!(my_atoi(String::from("   -42")), -42);
    }
    #[test]
    fn test3() {
        assert_eq!(my_atoi(String::from("words and 987")), 0);
    }
    #[test]
    fn test4() {
        assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    }
    #[test]
    fn test5() {
        assert_eq!(my_atoi(String::from("+-12")), 0);
    }
    #[test]
    fn test6() {
        assert_eq!(my_atoi(String::from("9223372036854775808")), 2147483647);
    }
}
