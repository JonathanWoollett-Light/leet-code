fn index(x: char) -> usize {
    (x as u8 - 97) as usize
}

// O(n) | O(n)
/// Runtime: 0ms faster than 100%
/// Memory usage: 2.2mb, less than 8.33%
pub fn partition_labels(s: String) -> Vec<i32> {
    // We get the range of each character from the index of its first appearance to the index of
    //  its last appearance.
    let mut ranges: [usize; 26] = [0;26];
    for (i, c) in s.chars().enumerate() {
        ranges[index(c)] = i + 1;
    }

    // By progressively adding to a bound the ranges of character we come across, we know when
    //  `bound==i` that it rests at a point where it has already covered all instances of all
    //  characters it has encountered.
    let mut bound = 1;
    let mut bound_indices = vec![0];
    for (i, c) in s.chars().enumerate() {
        // println!("({},{}): {}",i,c,bound);
        if i == bound {
            bound_indices.push(bound);
        }
        bound = std::cmp::max(ranges[index(c)], bound);
    }
    bound_indices.push(bound);

    // println!("bound_indices: {:?}",bound_indices);
    let bound_sizes = bound_indices
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();

    // Cast to `i32` since leet code doesn't seem aware `usize` exists.
    bound_sizes.into_iter().map(|v| v as i32).collect()
}
#[cfg(test)]
mod tests {
    use super::partition_labels;
    #[test]
    fn partition_labels_test_1() {
        assert_eq!(
            partition_labels(String::from("ababcbacadefegdehijhklij")),
            &[9, 7, 8]
        );
    }
    #[test]
    fn partition_labels_test_2() {
        assert_eq!(partition_labels(String::from("eccbbbbdec")), &[10]);
    }
}
