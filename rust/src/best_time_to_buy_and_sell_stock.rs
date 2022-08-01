pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut max_delta = i32::MIN;
    for i in 0..prices.len() {
        min = std::cmp::min(min, unsafe { *prices.get_unchecked(i) });
        max_delta = std::cmp::max(max_delta, unsafe { *prices.get_unchecked(i) } - min);
    }
    max_delta
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit_one() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
    #[test]
    fn max_profit_two() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
