/// We begin with the widest container, following this every container that contains more water 
///  needs to be taller.
pub fn max_area(height: Vec<i32>) -> i32{
    let mut water = 0;
    let (mut i,mut j) = (0,height.len()-1);

    while i < j {
        let h = std::cmp::min(height[i], height[j]);
        water = std::cmp::max(water,(j-i) as i32 * h);

        // The next container we check must have both sides (vertical lines) taller than `h` (the 
        //  min of our 2 previous vertical lines).
        // Thus we skip towards the center until we find lines taller than `h` for both the front 
        //  `i` and back `j`.
        
        while height[i] <= h && i < j {
            i += 1;
        }
        while height[j] <= h && i < j {
            j -= 1;
        }
    }
    water
}
#[cfg(test)]
mod tests {
    use super::max_area;
    #[test]
    fn max_area_test1() {
        assert_eq!(max_area(vec![1,1]), 1);
    }
    #[test]
    fn max_area_test2() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}