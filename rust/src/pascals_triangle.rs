// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Pascal's Triangle.
// Memory Usage: 2.2 MB, less than 43.07% of Rust online submissions for Pascal's Triangle.
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut vec = (1..num_rows + 1)
        .map(|i| vec![1i32; i as usize])
        .collect::<Vec<_>>();
    // Calculate left side unique elements
    for i in 2..num_rows as usize {
        vec[i][1] = i as i32;
        for j in 2..(i + 1) / 2 {
            vec[i][j] = vec[i - 1][j - 1] + vec[i - 1][j];
        }

        if i % 2 == 0 {
            let n = i / 2;
            vec[i][n] = 2 * vec[i - 1][n - 1];
        }
    }

    // Copy elements into right side
    for i in 3..num_rows as usize {
        for j in 1..((i + 1) / 2) {
            vec[i][i - j] = vec[i][j];
        }
    }
    vec
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pascals_triangle_one() {
        assert_eq!(
            generate(5),
            [
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
    #[test]
    fn pascals_triangle_two() {
        assert_eq!(generate(1), [vec![1]]);
    }
    #[test]
    fn pascals_triangle_three() {
        assert_eq!(
            generate(6),
            [
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1]
            ]
        );
    }
}
