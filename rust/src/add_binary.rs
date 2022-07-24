// Char to int
const fn cti(x: u8) -> u8 {
    x - 48
}
// Int to char
const fn itc(x: u8) -> u8 {
    x + 48
}
/// O(n) | O(1)
/// Performance varies too much
pub fn add_binary(a: String, b: String) -> String {
    let mut first = a.into_bytes();
    let mut second = b.into_bytes();
    let mut overflow = 0;
    if first.len() > second.len() {
        let diff = first.len() - second.len();
        for (i, j) in (0..first.len()).rev().zip((0..second.len()).rev()) {
            let (a, b) = (cti(first[i]), cti(second[j]));
            let sum = a + b + overflow;
            let temp = sum % 2;
            overflow = sum >> 1;
            first[i] = itc(temp);
        }
        for i in (0..diff).rev() {
            let a = cti(first[i]);
            let temp = overflow ^ a;
            overflow &= a;
            first[i] = itc(temp);
        }
        if overflow == 1 {
            first.insert(0, b'1');
        }
        String::from_utf8(first).unwrap()
    } else {
        let diff = second.len() - first.len();
        for (i, j) in (0..first.len()).rev().zip((0..second.len()).rev()) {
            let (a, b) = (cti(first[i]), cti(second[j]));
            let sum = a + b + overflow;
            let temp = sum % 2;
            overflow = sum >> 1;
            second[j] = itc(temp);
        }
        for j in (0..diff).rev() {
            let a = cti(second[j]);
            let temp = overflow ^ a;
            overflow &= a;
            second[j] = itc(temp);
        }
        if overflow == 1 {
            second.insert(0, b'1');
        }
        String::from_utf8(second).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_binary_one() {
        assert_eq!(
            add_binary(String::from("11"), String::from("1")),
            String::from("100")
        );
    }
    #[test]
    fn add_binary_two() {
        assert_eq!(
            add_binary(String::from("1010"), String::from("1011")),
            String::from("10101")
        );
    }
    #[test]
    fn add_binary_three() {
        assert_eq!(
            add_binary(String::from("110010"), String::from("10111")),
            String::from("1001001")
        );
    }
}
