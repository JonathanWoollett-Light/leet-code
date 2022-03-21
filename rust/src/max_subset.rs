// O(2^n) | O(1)
pub fn max_subset(x: &[i32]) -> i32 {
    match x.len() {
        0 => 0,
        1 => std::cmp::max(x[0], 0),
        _ => std::cmp::max(
            std::cmp::max(x[0], 0) + max_subset(&x[2..]),
            max_subset(&x[1..]),
        ),
    }
}
// O(n) | O(1)
pub fn max_subset_iterative(x: &[i32]) -> i32 {
    // Shrinks `x` by removing consecutive zero or negative elements (this is not ultimately
    //  necessary in this approach but it helps reduce complexity for now while this solution is
    //  WIP, it makes this solution O(2n) instead of O(n), we can remove this in a bit when this works).
    // O(n)
    let mut stripped_x = x
        .iter()
        .cloned()
        // Effectively clamps values in `x` into `u32`s
        .map(|a| std::cmp::max(a, 0) as u32)
        .collect::<Vec<_>>();
    // Removes consecutive zeros
    stripped_x.dedup_by(|a, b| *a == 0 && a == b);
    // println!(
    //     "{} -> {} (-{:.2?}%)",
    //     x.len(),
    //     stripped_x.len(),
    //     100. * (1. - stripped_x.len() as f32 / x.len() as f32)
    // );

    // For testing we output the resulting new set
    std::fs::write(
        "testing.txt",
        stripped_x
            .iter()
            .map(|s| format!("{} ", s))
            .collect::<String>()
            .as_bytes(),
    )
    .unwrap();
    // println!("{:?}", stripped_x.iter().take(40).collect::<Vec<_>>());

    let mut max = 0;
    // O(n)
    let mut iter = stripped_x.into_iter().peekable();
    // We make sure our starting value is non-zero by skipping the first value if its zero (if we didn't dedup this would need to be a peeking take while)
    if let Some(0) = iter.peek() {
        iter.next();
    }
    // println!("max adds:\n");

    while let Some(next) = iter.next() {
        // At this point we know `next>0`, thus if `iter.next()==0` or `iter.next()==None` there is
        //  no competing subsequence.
        if let None = iter.peek() {
            max += next;
            // println!("{:05} {:010}", next, max);

            // We could call `iter.next()` here although we don't need to, as we know the loop will exit next condition check.
            // But this would let us join this case and the `Some(0)` case.
        } else if let Some(0) = iter.peek() {
            max += next;
            // println!("{:05} {:010}", next, max);

            // We call `.next()` to skip over the zero value such that `next` is either some non-zero value or none next iteration.
            // If we didnt dedup this would need to be peeking take while
            iter.next();
        } else {
            // Else we have adjacent non-zero values e.g. `stripped_x[i] != 0 && stripped_x[i+1] != 0`
            //  and have to pick which we include.
            // In this case we now effectively have 2 possible sequences [i,i+2,i+4,...] or
            //  [i+1,i+3,i+5,...] (in both cases these sequences end at the first zero value we find).
            let mut max_options = [next, iter.next().unwrap()];
            for flip in (0..2).cycle() {
                match iter.next() {
                    None => break,
                    Some(0) => break,
                    Some(following) => {
                        max_options[flip] += following;
                    }
                }
            }
            max += std::cmp::max(max_options[0], max_options[1]);
            // println!(
            //     "{:05} {:010} (s) ",
            //     std::cmp::max(max_options[0], max_options[1]),
            //     max
            // );

            // If we didnt dedup we would need a peeking take while here
        }
    }
    max as i32
}
#[cfg(test)]
mod tests {
    use super::{max_subset, max_subset_iterative};
    #[test]
    fn max_subset_testa() {
        assert_eq!(max_subset_iterative(&[3, 5, -7, 8, 10]), 15);
    }
    #[test]
    fn max_subset_test0() {
        assert_eq!(max_subset_iterative(&[3, 7, 4, 6, 5]), 13);
    }
    #[test]
    fn max_subset_test1() {
        assert_eq!(max_subset_iterative(&[2, 1, 5, 8, 4]), 11);
    }
    #[test]
    fn max_subset_test2() {
        let temp = std::fs::read_to_string("max_subset_tc0.txt")
            .unwrap()
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        assert_eq!(temp.len(), 81570);
        assert_eq!(max_subset_iterative(&temp), 151598486);
    }
    #[test]
    fn max_subset_test3() {
        let temp = std::fs::read_to_string("max_subset_tc1.txt")
            .unwrap()
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        assert_eq!(temp.len(), 3923);
        let hundred = temp.iter().cloned().take(100).collect::<Vec<_>>();
        assert_eq!(max_subset_iterative(&hundred), 188428);
        let two_hundred = temp.iter().cloned().take(200).collect::<Vec<_>>();
        assert_eq!(max_subset_iterative(&two_hundred), 392453);
    }
    #[test]
    fn max_subset_test4() {
        assert_eq!(
            max_subset_iterative(&[
                2, -1, 3, -4, -3, 4, 5, 7, -1, 3, -5, 2, -5, -2, 2, 3, 9, -3, 2, 9, 3, -5
            ]),
            41
        );
    }
    #[test]
    fn max_subset_test5() {
        assert_eq!(
            max_subset_iterative(&[
                5, 6, 2, -1, 3, -4, -3, 4, 5, 7, -1, 3, -5, 2, -5, -2, 2, 3, 9, -3, 2, 9, 3, -5
            ]),
            46
        );
    }
    #[test]
    fn max_subset_test6() {
        assert_eq!(
            max_subset_iterative(&[
                -1, -2, 5, 6, 2, -1, -3, -1, 3, -4, -3, 4, 5, 7, -3, -1, 3, -5, -3, -2, 2, -5, -5,
                -5, -2, 2, 3, 9, -3, 2, 9, 3, -5
            ]),
            46
        );
    }
    #[test]
    fn max_subset_test7() {
        assert_eq!(
            max_subset_iterative(&[
                3776, 2443, 7152, -4592, 9211, 6241, -8026, 3199, -9273, -7330, -255, -6936, 8203,
                -1357, 5067, 996, -707, 568, -2377, -5388, -2063, -9075, 5741, 3957, -7629, 1448,
                4623, -3207, 8552, 1922, -2159, -8151, -2636, 7479, -3135, 8619, 4717, 5484, -3474,
                5085, 8963, 7172, 9146, -1918, -7668, 1354, -9592, 2689, -7036, -1782, -933, 3717,
                6082, 297, -9262, 7839, 7279, -9468, -5284, -7819, -7662, -3167, -463, 6279, 5462,
                -1925, -1017, -9917, -6449, -2582, 8871, -6873, -4589, -2211, -2843, 4531, -3917,
                9375, 8406, -1830, -9541, 2961, -195, 9493, 9976, 2772, -7973, -1655, 3057, -1430,
                -9168, -1436, -9053, 8527, 1424, 9404, -8714, -2856, -328, 9411, 2722, -7286,
                -5657, -3420, 8328, 2382, 6715, -9205, -8684, 7661, -211, 2309, 5975, -3320, -4412,
                -3487, 5331, 4072, 3421, -3221, 6209, -6396, 6137, -4993, -6381, 1557, 5900, 3863,
                2069, -7385, 7582, 66, 6265, -3297, -261, 4494, -4816, 2288, 5686, -7667, 7278,
                -979, -2501, 4320, -1805, 2198, -3236, 7656, 7697, 5548, 7779, 4858, 789, 2871,
                -6248, 1628, 5368, -5813, -4709, 5724, -1777, -2346, 9037, -3776, 9440, -8071,
                -2705, -3297, -7034, -6954, -8008, 3586, -6283, 2343, 8945, -4105, 4238, -2503,
                7782, 1915, -7456, 708, -5091, 5670, -8792, -536, -2893, -2611, 5633, 1499, 12,
                6367, -845, 1937, 6965, 9282, -2558, -7132, 6405, 7940, -8274, -489, -4832, -2570,
                3258, 873, 1351, -38, -4217, -7442, 3432, -6741, -6173, -2814, -2821, -4868, -4126,
                -9635, 6024, 1168, -1025, -5260, -3621, 6582, 9102, -9998, 5267, -5983, -3984,
                -3406, -4218, -5890, -7354, 1679, -3338, 4484, -4524, -334, 8453, 8803, -7030,
                1126, 7037, 6402, -9227, -2348, -4150, 4311, -220, 2219, 6391, -2312, -3150, -8079,
                -5758, 2123, -2565, 2535, -7089, 4325, -4563, 7614, 1758, 593, 3273, 1561, -5296,
                -88, 9126, 1753, 4795, 8647, -4901, 6858, -1450, -5980, 4285, -6024, -6498, 7315,
                -6587, 8559, -2341, 8802, -3946, 6887, -7512, -6215, 2512, 1449, 3777, -3370,
                -3298, -4893, -9937, -4491, -6660, -4375, -6290, 3448, 1316, 1759, -5590, -9787,
                -7123, -4132, -7547, -1042, 9284, -924, 7656, 4960, 856, -5941, 5860, 6017, 8766,
                4810, -5497, 7668, -7890, -5315, 5235, -7574, 1335, 2631, 4428, 2438, -6434, 4807,
                6296, -3939, -4877, -9766, -2329, -7822, -3480, 6540, -1488, -5246, -4437, -6667,
                -2543, -1062, 5004, -6325, -4349, 4580, 6340, 1211, 8274, -9467, -7536, 288, 5209,
                -5236, 3581, 621, -6103, 31, -4052, -529, -2507, -9585, 3917, 67, 5132, 3084, 2642,
                -2873, -9598, -2704, -1733, -1658, -5121, -9651, -6044, 3696, -5577, 8439, -8530,
                4899, 8777, 4885, 8224, 4153, -2733, 9749, -1327, 3611, -5886, 9246, -7077, -7574,
                2096, 6423, -9487, 7750, -1362, -2661, 8967, -1815, 9167, -5349, -766, -3570, 6112,
                71, 7884, -9972, -1414, 3187, 1102, -2143, -7042, -6937, 9952, 6833, -283, -5899,
                -206, -5897, 1494, 9895, 6043, -1642, 6233, -6344, 6030, -2660, -4630, -9051,
                -6053, 4938, -2299, 9886, 410, 2469, 7644, 6616, 1248, 7808, 3247, 9068, 2152,
                -3305, -2838, -6779, -5501, -5405, 4604, 5286, 9358, -8938, -1752, 212, -9270,
                -2697, 428, 9673, 6418, 3118, -6649, -1709, -3389, -2876, 3911, 6662, -5627, 5309,
                -7397, -1142, 8403, -4178, 9981, 9216, -6899, -7399, 2760, -2637, 2391, 4051, -576,
                7071, 9739, 6448, 1887, -2135, 5438, -5244, 1803, -4944, 6873, 3982, 4837, -2901,
                -8161, 6839, 6519, 6716, 8334, 2960, 8397, 4162, -9789, 2453, 3353, 6933, -361,
                -4968, -944, -7328, -9540, -9984, 3447, -1926, 973, -9293, 6770, 2118, 5628, -5647,
                -6494, 44, -7353, 5846, 9200, 4224, 689, 4025, -261, 8077, -3562, 3900, 2521, 5552,
                -394, -1892, 2489, 2875, 8380, 4087, 3215, -5500, -4879, 599, -2241, 6059, -7562,
                -5955, 8810, -3226, 8128, 7297, -380, 5014, 9669, 5808, 4567, 1424, 2084, -8438,
                1422, -8061, 5007, -4489, 8125, -9305, 5538, 5140, 4392, -5614, -4585, -9175, 3694,
                -5675, -5688, 3848, 8229, -5682, -2645, 2565, 6832, 2189, -9994, 2378, -1674, 951,
                7040, 2983, -779, 4290, 1483, -9646, -2947, 7136, 5084, 1417, -6934, 5913, -2177,
                -8123, -517, -5949, 6056, 1218, 9457, -4495, 5665, -7416, -7121, 8376, 9576, -2375,
                3254, 9207, -476, -3556, 1219, -6341, 420, 9521, -7911, -3294, -2787, 6235, -4278,
                -8800, -6200, -627, 9451, -3953, -8643, -9011, -8780, -8257, 3938, -6343, 4644,
                -7791, -7548, -7147, -8809, 7686, -8104, -5491, 1592, 9565, 2972, 9962, 6369,
                -5992, -7990, -3702, -6991, 7093, -17, 8397, -7394, 6035, -7350, -8674, 7993, 1127,
                5364, 1312, 50, 6156, 6286, -7559, -746, 491, 185, -3047, 7873, -8401, 8446, 4243,
                9124, 4279, -1537, -59, 8596, -9362, 7316, 3546, 9618, -4547, 728, -5674, -5156,
                -8547, 7204, 4, 8372, -8746, -2060, -10000, -6325, -5141, 5163, -2639, -687, 4739,
                8481, 9693, -1948, -8954, -5727, -6276, -1874, 1956, 4281, -1558, 9335, -9838,
                -4066, 5717, -6571, -7296, -8883, -3461, -5700, -4, 5188, -5309, 6862, -6601, 541,
                2143, 4166, 2205, -7894, -777, 2072, -9342, 7163, 8502, -8174, -6735, 7425, 6949,
                -3319, 2579, 8771, 8204, 5896, 3356, 9138, 9934, 923, -235, 6423, -9109, -2766,
                -1049, 7737, -4922, -9592, 7259, 3592, 2831, 8929, 7468, -9587, -93, 6427, 6150,
                8182, 6053, 3344, 278, -7250, -7332, 4778, 404, 9559, -5021, -340, 1053, -4306,
                -5060, 739, 6801, -9697, -5399, -4243, -1039, -3884, -184, -6395, -705, -2516,
                5511, 8618, 2537, 9785, -5723, -8130, -769, -6425, 7929, -7143, 6255, 4040, -2881,
                -5979, 1020, -7768, -3793, -4614, 8690, -4631, -5747, -9045, -373, 2082, -2562,
                7928, -705, -2041, 6775, 5310, -9398, 8561, -8528, 4380, 9510, 7375, -4088, 8534,
                1661, -8887, -7504, 5130, 9568, -4251, 7217, 8666, -9967, -2834, -6669, -5726,
                8702, 5287, 5247, 4497, 613, -8154, -7504, -9848, -60, 1035, -1503, 2038, -2386,
                2012, 7429, -3835, 6045, -3000, 7704, 9263, -381, -7862, 9452, 8881, 4516, 8935,
                -6130, -6534, 2191, -6947, 6065, 2728, 5390, 3911, -7488, 3088, 8319, 8676, 7080,
                2634, 5507, -3374, -6982, -2518, 2433, 8278, 1385, -3317, 156, -8031, 7956, 864,
                -8304, -4368, 2420, 3635, 1609, -7401, -6102, -775, 3073, -5301, 8857, 1036, -2287,
                -5483, -2928, 8166, -5929, 3236, 5504, 7658, -4505, -5982, -9123, -4360, -9328,
                -2637, -7663, -8770, 4801, 3828, -2785, 5075, 7888, 4568, 822, 2866, -8468, 2518,
                7061, 2804, -9387, 2303, 1346, 7055, -3544, -5276, 7402, 3332, -1589, -2708, 9204,
                -8724, -5285, 5205, 9124, 5038, -6269, -4077, 4544, 7068, 3070, 1843, -4320, -7925,
                3053, 8053, -1758, -9032, 4995, -1171, -5569, -6983, 5792, 2204, 2581, 754, 2642,
                -7323, -4696, -8554, -4385, 2500, 3814, -1984, -8833, -3920, 5541, 6564, 8294,
                -3974, 7240, -7877, 4457, 2756, -8215, 2460, -8018, -2327, 7291, 1540, -8888, 6219,
                2658, 4301, -4473, 8736, -3494, 8311, 3962, -5896, -5555, 7653, 6478, 9087, -7060,
                -1082, 4504, 9037, 6886, 899, -4178, -8081, -4438, 5840, -9700, 6342, -4272, -3715,
                2904, -5439, 9654, -2772, -6661, -9166, 1487, -9753, -2625, 886, -6282, -2851,
                3764, -8155, 2414, 9206, -9169, -4806, 1127, 3617, -8288, 5079, -2870, 6256, 8639,
                8429, -1798, -258, -6025, -6969, -1597, -4856, 6136, -9578, -3586, -6358, -7829,
                -1363, -148, -3246, -7829, 253, 6868, -8253, 7874, 6226, 8643, -7460, -9472, -2203,
                2946, 3351, 3115, -7317, 5462, 7519, -4785, -5169, 5420, -8836, -6196, -7667,
                -2005, -7180, 9060, -9200, 9978, 5047, -7969, 8653, 1308, -1930, 5019, -7878,
                -9285, 2280, 5911, -5069, -3903, -5687, -5239, 209, 2506, -6101, 2652, -2014, 635,
                5290, 3278, 7983, -4042, -2437, -34, -417, -394, 9619, 5325, -3593, -7809, 5615,
                -8966, 8732, -903, -5994, -4154, 4855, -1985, -3504, -7662, -1657, 4464, -3417,
                -5409, 5282, 8413, -7872, -504, 2114, 5912, -1995, 561, -6622, 6462, -997, 5988,
                -8322, 2376, -5459, -7627, 6428, -4397, -7486, -2974, 5880, 4641, 2251, 9255,
                -8968, 7809, 308, 1519, 8906, -1459, -4005, -8228, 8279, 3350, 3082, 3144, 9103,
                892, -3778, -8062, 8396, 5090, -2147, 6140, -2114, -4635, -703, 8929, 1876, -7500,
                552, -328, -6306, -2230, 8041, 1860, -322, 9721, 3116, -5690, -4564, -2336, 5570,
                -3590, -7023, 3622, 7378, 422, 3721, -672, 1587, 1201, -6724, -8970, 6822, -9628,
                -3909, 1365, 9171, -7003, 5634, -1895, -3086, -4450, 6914, -8201, -4180, 218,
                -3978, 4991, -4983, -6952, 2467, 7955, -4526, -7206, 9035, 9713, 8995, -796, 2470,
                1374, -5356, -9616, -4507, -9960, -2180, -4460, 5113, -7695, 472, -3142, 9542,
                2413, 6013, -9976, 9315, 474, 6333, -3277, 194, 430, 2292, 3761, 11, 1997, 42,
                -2518, 1158, -2861, 8839, 4075, 8537, 4441, -151, 107, -2519, 2627, 4430, -7285,
                -9350, -3964, -8905, 6159, 6830, -8535, 8753, -8017, 353, 3890, -1646, 5291, -2606,
                7457, 3420, -5234, -1477, -1011, 1988, 4098, -7387, 3766, -5165, 8903, 660, -4218,
                -9495, -4180, 7893, -1803, -9694, -8675, 3379, 773, -6248, -9781, -7625, -2109,
                -9449, -5419, 3099, -9223, 2797, 9432, 9807, 3695, -9666, 1153, 937, 5280, 3991,
                9184, -2480, 1490, 537, -6460, 2817, -7609, -1133, 1078, -7739, 2079, 9306, -4509,
                4103, 3496, -4133, -8466, 788, 5420, -4041, -5763, 5499, 5095, 4021, 9697, -6896,
                5534, 2366, -8401, -7893, -4271, 2893, 4996, -6233, -953, -587, -9749, -9162, 4743,
                -3566, 1394, -5832, -7061, -4761, -974, 5251, -4298, -4956, 2171, -8231, 1848,
                6252, 785, -4491, -7393, -9680, -2035, 118, 5551, 1108, 7424, -4677, -6393, 6629,
                2753, -1482, -8560, -1427, -2516, 7619, -5961, 5978, 6797, -1536, -515, -6720, 967,
                -7606, 8463, 1135, -3535, -235, -1840, -9682, 4009, 822, -6494, 4009, 7004, -7726,
                2001, 5776, 991, 2143, 6119, -4787, 2546, 883, -1036, -925, -3930, 9174, 8337,
                9867, -5427, 1697, 2457, -6697, 425, -2616, 7833, -6114, -2008, -9576, 3779, 3163,
                7375, -7314, 510, -5775, -7580, -3648, 9010, -3573, -8017, -3379, -7890, 4404,
                -5915, 2794, 7843, -9632, 9971, 1371, 693, 3761, -6351, -6332, -415, 3619, 1188,
                1597, 9418, -4574, 130, -2106, 5589, -4456, -8570, 9883, -6693, 5626, 883, 6776,
                -9663, 9665, -9646, -6615, -1155, -7589, -555, 6462, 111, -6143, 3126, -8013, 8552,
                -6550, -718, -9472, 3823, -2892, -1429, -4713, -7628, -5520, -9162, 5659, -1353,
                -6361, 8844, -9748, 2661, -6340, -7164, 1897, -5373, 3059, 3013, 7669, 9871, 9815,
                -5655, -6494, 8826, 4896, 8886, 305, -3359, 6331, 2141, 4832, -9964, -3021, -5454,
                4275, -5849, -4102, -8894, 5058, -9446, 2731, 491, 8977, 7050, -8291, 8827, -6061,
                -2324, -9325, -4982, 9329, 8740, 3557, 2418, -5326, -5824, -4168, 4213, 6132, 576,
                4746, 6578, -8467, 5348, -1457, 7160, 8334, 4276, 904, 1182, 5467, 1968, 5823,
                5890, 5362, 3478, -8793, -1789, 7159, 6335, 4323, 1379, -7000, 7422, 5, 8745, 703,
                -4628, 9326, -1180, -9334, -5291, 3693, -7088, 3738, 25, -9979, 6673, 2868, 1509,
                350, -6219, 1087, -3315, 8565, -3785, -7122, -2369, -6577, 8323, -7678, -126,
                -6087, 1108, 9837, 2438, -2714, 6263, 7547, 5569, 633, 2232, 3415, -2689, 4635,
                -9179, 209, 5296, -6802, 2320, -666, -6468, -668, -9114, 5126, 3208, -475, 5252,
                -9550, 2232, 4130, -6718, 2500, 9659, -5197, 9023, 262, -5248, -9123, 9204, 5948,
                843, -5706, -3665, 427, -6899, 3070, 6825, -7007, -6462, -9286, 3024, -2246, 5506,
                5695, 8057, 7767, -2250, -9333, 9660, -1685, 9708, 6989, 4732, 2773, -9310, 6326,
                -5318, -4598, 334, 3431, -5297, 3970, -8402, 4015, -4974, 9973, -908, 5040, 4161,
                8675, -484, -6340, -6878, 2301, -19, -1487, 9307, -3986, 4551, -4297, -9837, -1114,
                -2758, -1366, -721, -5419, -4672, 7750, 604, -1238, 1166, -9915, 7000, -991, 3800,
                -8346, 3088, 6587, 1359, -5062, 2867, -3721, 7549, 1361, 6049, -6707, 2396, -4138,
                6977, -9805, -351, -8544, -5519, 7694, -8599, -2881, 7949, 8819, 848, 7074, 3928,
                8672, 7899, -2561, -9149, 3423, -8611, 8753, 2674, -1582, -1531, -5930, -7240,
                -9146, -421, -3653, 6599, -5801, -5268, 7539, 227, 7209, 7696, -9534, -9063, -3899,
                -771, -833, 9786, 3554, -7654, -9718, 1270, -396, -1427, -5408, -3351, -4273, 8435,
                -208, 2966, 504, 2757, 5173, -7927, -1588, 8674, 2943, -7483, 6753, -9224, -344,
                6169, -5328, 8927, -1889, -1372, -6325, -4075, -5636, -3636, 1239, 9199, 5737,
                -7980, -7446, 9493, -5020, 4906, 2322, 7555, -3527, -8280, 3929, -1468, -3031,
                -4715, -2437, -3658, -9411, 8697, 8404, -3090, 9926, -542, 6847, 8617, 9886, -6398,
                5511, 7264, -2163, 7381, 693, 3889, 1951, -3644, -5201, -8970, -1164, -5107, 389,
                -6550, 4945, 2363, 2183, 8973, 3023, 6996, 9823, -372, -916, -3728, 7200, 8703,
                -9767, 8787, 7810, -6163, 2446, 6246, -3462, -8908, -2493, 911, 8365, -1220, -7743,
                3803, -6205, -5896, -1308, 5662, 685, -364, -3866, 8593, 17, 6788, -2507, -648,
                -7537, -7478, 7455, 4093, -7465, -4628, 7705, -9373, 6567, -5510, 4870, -596,
                -3542, -3224, -8400, -7049, -6794, 1707, -3729, -9468, 2675, -4346, -6704, 1024,
                7522, 3144, -2954, -7750, -2261, 9634, -7561, -9004, 1561, 3848, 4550, -6265,
                -1527, 9575, -4699, 271, -2099, -6909, -5858, -5058, 7616, -2881, 691, -8904,
                -5744, 8947, 2611, -8608, 2151, -2563, -251, -4826, -7263, -6458, -1789, 2608,
                6847, -670, -1457, 1052, 4569, -6820, -779, -6457, 2492, -3058, -1544, -2672, 1168,
                -1574, 9482, -1823, -4580, 8029, 928, -1695, -3359, 9665, 245, -4691, -4927, -4339,
                152, -5912, 7591, 4876, 7112, 7066, -8058, 7278, 438, 7893, -2075, -6507, -1044,
                1409, -6574, -412, -4606, 6892, -3257, 7907, 1688, -2208, 5925, -622, -8012, -1947,
                -2454, 2274, 1872, 6161, -751, 4541, 9446, 9199, -173, 7150, 1803, 7393, -235,
                -2098, -7351, 9779, 7289, -3477, -236, -8323, -3525, 9809, -9612, -622, -6626,
                -8274, 5368, -1436, -4695, -4854, -3254, 7992, 3668, 5609, -4092, 1061, -8589,
                -7062, -1782, -2985, -177, 9552, -6601, -7459, 8311, 7767, 9358, -8641, 9057, 4150,
                -8725, -6534, 4715, -708, 5081, -1737, 589, 6531, 6082, -5069, 4969, -9472, 9890,
                1028, 1154, -3824, 4819, 8227, 9533, 2103, 8552, 7633, 1583, 7443, 5209, -2792,
                -8936, -2623, -7461, 4503, -1341, -3063, 5945, 8323, -7335, -9886, 3182, 5705,
                -4737, -6963, -9308, 6290, 6041, -1430, -3690, -5319, -1567, 2497, -739, -6272, 12,
                -2901, -9506, 9791, -3365, 7005, 1582, -870, -6423, 996, -5285, 2207, -9042, 3837,
                8531, -2476, -3899, 8838, -2561, 5529, 9056, 7764, 4888, -8645, -4395, -912, -2674,
                381, 5395, 7574, -4632, 5899, -6293, -5490, 3146, 7074, 2833, 464, 7304, 4279,
                -2141, 4981, -9954, -2725, -5462, -4438, 3778, -8328, -2735, 9466, 3789, 3242,
                8811, 3384, 2696, 2053, -2706, -434, 6406, 6038, 1839, -8612, -8140, 706, 3230,
                -7420, 1125, 5040, 1049, 7190, -9515, -6179, -3541, 2056, 4299, 957, 347, -2205,
                2071, 116, 7862, -1924, 778, -8364, 1798, -6554, 1200, 1302, 3664, -7590, 9720,
                5613, -4247, -9627, 7695, -6153, -7750, -7974, -3127, 4637, 7657, 1979, 8009,
                -5793, 4378, -974, 829, 3043, 4981, 418, 8959, 9358, 9103, -5875, 5375, 2568, 8334,
                3981, -4533, 5805, -3231, -4342, 3176, 1503, 4587, 8761, -5771, 6730, -4004, 1213,
                3878, -9735, -3881, -2804, 5637, -4445, 9321, 5170, -3720, 7886, 3257, -8214, 7789,
                6420, -475, -8705, -7602, 2930, -7829, -3870, -1715, 2261, 9393, 9436, 7005, -6090,
                -373, 8441, 5219, 8598, -9227, -9930, 6084, -4697, -4490, 2335, 2231, -7315, -9798,
                6411, -446, -3192, -8874, -9420, -1303, 9854, 7590, 8003, -150, 5613, 9686, 1710,
                -5071, 6889, -1412, -7861, 6358, 7159, 5894, 5003, 4113, 2052, -8677, -798, 1244,
                7528, 750, 612, -1491, -3871, 5479, 7162, 8715, -7017, -4194, -4742, -430, -7814,
                2111, 1227, -3442, 6280, 1416, -4291, -2171, -8937, -7270, -1154, 2159, 6853,
                -3994, -2667, -416, 6876, -4801, 487, 434, -5506, -3044, -5083, 359, -4650, 1219,
                8534, 2410, -6724, -5782, 1024, -9510, -2854, -9748, 1057, -6747, -2138, -6100,
                -1037, -3682, 8663, -7787, 4400, -9030, 6161, 9653, -6344, 5253, -5434, 2916, 7633,
                3716, -2626, 7147, 657, 1637, 2835, 1389, -4287, 9758, 3879, 1152, -1369, -5454,
                -4307, -3812, -9766, -3190, -5076, -1913, 7996, -4664, -7192, -8366, -8881, 163,
                1237, 7188, 288, 731, -7516, -945, 7354, 3808, 4251, 3905, -481, -3110, 3355,
                -5357, -3618, 5177, 7127, 3747, -5970, -2917, 74, -9073, -1362, -258, -7042, 6428,
                7015, 3931, -7283, 7565, 3641, 2969, -9921, -8591, 8455, 8322, 1885, -2664, 5333,
                -3105, 2811, 4950, 7009, 1071, -2851, 7866, -2587, 4808, -7681, 3455, 454, -9586,
                6251, -8655, 3840, 7824, -2037, -1806, -4870, -4415, 8614, 5689, -7051, 4687, 1408,
                8022, 8341, 1259, 3265, -9472, -1985, 7556, -9889, -6971, -8735, 8932, -7462, 1263,
                -1993, -3199, 9444, 4991, 9416, 4568, 9111, -1618, 7944, -8447, 4469, -7125, 8488,
                2278, -788, 5286, -9946, 4884, 3813, -1968, -5260, -5736, -6158, -171, 3873, -4143,
                -4367, 8811, 2132, -5294, 3813, -9357, 3187, 2247, 3475, -2917, 8813, -3069, 7101,
                -1165, 1543, 350, 9907, -8357, 6670, 6868, -1098, -5557, -406, 7648, -9596, -932,
                4185, -316, 7420, -672, 9643, -5564, 8900, -7802, 9910, 1633, -3284, -6285, -450,
                -9082, 7390, -1952, -5828, 4254, -3031, -9169, 8865, 3518, 4101, -9361, 799, 6434,
                7147, -3511, 4056, 9061, -322, -9902, -9739, 1132, -9941, -8080, 896, -4811, -4886,
                -9414, 1870, 688, 2651, 9904, 3056, -8547, 2807, -8877, -1713, 4883, -992, 9085,
                -871, 1054, -4761, 9029, -1745, -3023, 9577, -9928, 5484, -5753, -2977, -2942,
                -2113, -7573, 9543, 5759, -5111, -9448, 9197, -6013, 3379, 5179, -9351, 841, 3986,
                -6888, -4271, 3854, -6708, 8819, 8008, -8294, -5494, 9081, 3483, -2651, -9824,
                3853, 4576, 7936, -478, 944, 8054, 7221, -1903, -8151, 8743, -6844, -9082, -551,
                -5814, 8902, 198, 2781, 851, 5870, -6698, -1356, 9936, -4148, 2851, 2180, -3144,
                -965, -415, 5766, 8376, -2304, -4623, 1305, -4667, 9395, 6016, -1762, -9377, -6977,
                -8042, 5779, 8953, -247, -8607, -9698, -7147, 6687, -415, 9238, -8893, -7951, 3544,
                4123, -2983, -8580, 2030, -8721, 7109, 1285, -1279, -1713, -1291, -7779, -6071,
                4856, 1643, -7261, 6031, -976, 2047, -8063, -2340, 6866, -4768, 4724, 6055, -6835,
                -4973, -7341, 6205, 5467, 1256, 8276, 7420, 7116, 9447, -6926, 5094, 5217, 4999,
                -6615, 3248, 4512, -6857, -4251, -8884, -2123, -4344, 2878, -538, 765, -2986,
                -2507, 1589, 9752, -7234, -7359, -5088, 2178, -4914, -3398, 2826, -855, -9455,
                7122, -6978, 602, -5359, -1084, 6341, 412, 9585, -1479, 4454, 996, 6553, 6024,
                -6771, 2245, -5947, -1757, 7189, 588, -8255, -7528, -8373, 3274, 8706, 5959, 582,
                9837, -4081, -7474, -7176, -829, 9580, -2504, -9872, -8825, -8032, -8781, -5620,
                -2128, -781, -5220, -1722, 8457, 8344, -4600, -6913, 848, -874, -6225, -8574,
                -5903, -3185, -5399, -9067, -974, -4394, -4648, -3692, -7390, -1047, 3859, -8690,
                -3701, -9748, 7847, -9104, 1538, -81, -4688, 2073, -9105, 9331, -4253, 8814, -2028,
                9820, -6330, 3717, -8469, 627, 3560, -8857, -5846, -5904, 8080, 9445, 9755, 9100,
                -5079, -7560, 5438, -4770, -994, -825, -4152, 5973, -7210, -3618, 6111, 7923,
                -2176, -6883, 9406, 9882, 6842, 1612, -9685, 6964, -8703, 4463, -4144, 7116, 6141,
                3675, -7099, 5355, 562, -1792, 1311, 2278, -7274, -9486, 2684, -1757, -6288, -2880,
                7103, 266, 6109, -5810, -1418, 2545, -651, 4551, -515, -4730, 478, -6289, -6302,
                3623, 1026, -916, 9748, -6632, 9021, -9892, -7350, 8015, 4284, -9597, 8848, 1595,
                860, -5908, -2258, 1458, 4989, -1812, 7055, 4802, 8830, -1365, -8348, 7579, -6136,
                9953, 3789, 8902, 98, 1957, -4177, 2293, -7230, 7589, 7813, -3996, -3336, -9661,
                3053, -1273, 9016, -1473, 7325, 4350, 2339, -6933, 46, 4093, 4254, 7589, -6655,
                6152, -5186, -2864, -6622, -1141, -4787, -6439, -5599, -455, 8715, -2838, -951,
                4727, 303, -9898, -4943, -9260, -608, 460, -6058, -7826, 6966, 7618, -1824, -6743,
                4364, -7765, 4545, -6281, 7760, -4374, -2915, -5555, -4668, 3281, -4768, 8808,
                -9748, 3849, 285, 2897, 1107, 6659, 4045, -8029, 5985, 3569, -646, -201, 8143,
                -8027, 697, 936, 8610, 9439, 4502, -7335, 6062, -6203, 9864, -2445, -9319, 3422,
                5369, 2935, -5367, -3653, -3465, -4365, 2176, 1881, -9024, -3233, 6594, -4038,
                8562, 1139, 9703, -907, -9390, -3365, 5754, -8613, -5387, -5554, 189, 650, -3468,
                7132, -9807, -1648, 3155, 9831, -429, -2449, -5020, 9719, 974, -9111, -3383, 2871,
                -3861, 6142, 1490, -4805, -1799, 2960, 4264, -1042, 553, -4966, -2794, 4987, -2076,
                -1432, -2483, 8131, 7008, -6884, -1829, -5807, 6036, 4723, 6464, 246, 258, 2017,
                -2328, -7744, -8590, -9166, 8369, 2572, -3685, -1879, 7370, -7756, 7120, -465,
                1838, -8622, -3521, 7686, -7095, 7608, -9464, -1928, -9266, 5816, -2722, -4353,
                -3298, -6191, 6890, 16, 6223, -6366, -8189, 4413, -5596, 1040, -4174, 7758, 3014,
                -4481, 2600, 8570, -9604, -3442, -9385, 4886, -6922, -9540, -8148, 1473, 9538,
                -2192, 3902, -1846, -6824, -2859, 2096, -9772, -499, 2167, -4234, -6202, -4346,
                6019, 3534, -1640, -297, -2233, 9593, -3954, 9649, 7616, 2987, -5508, -8798, -8765,
                -1664, 8841, -9963, 4752, -3533, -764, -3203, -8772, 5635, 8476, 8831, -5377,
                -4027, -319, -5617, -6976, 7749, -4628, -4391, -2765, 1265, 7596, 7301, -4945,
                -5953, -3699, -5537, 2453, 9618, -3385, -6858, 3366, 1497, 2816, 8162, -9664, 9014,
                -1252, -421, -3513, -8509, -4270, -7727, 5253, 992, 4152, -2947, 8606, 5342, -9213,
                4645, -4066, -7382, -8213, -3289, 1589, 2072, -9727, -4364, 4291, -8379, 722,
                -2496, 6677, 1154, -3503, 4467, 4625, -9212, 9082, -5126, -9354, -3615, 4717,
                -9913, 7656, 2695, 1128, -1972, -3872, 2166, 4195, -6441, -6876, -5660, 4336,
                -1259, -6209, -1767, -4600, 3637, 1464, 7092, 9335, 8035, -8653, 4137, -8958,
                -2147, 3050, -6140, 6133, -9338, -8070, 9981, -5488, 2506, 8216, -9660, 5499, 6991,
                9812, -1281, 9897, 4507, 383, -1881, -9930, -3004, 7449, -8724, 9660, -1421, -4728,
                -8606, -3095, -9384, 2539, -349, -7510, 4795, 2777, 8031, -9887, 2491, -1579,
                -8510, 2784, -6195, 2652, 8852, -4763, 3350, -3875, -9874, 8760, -8185, 9563, 4657,
                1873, -5946, -7548, -6166, -7931, -7566, 2697, -9470, -2581, 1128, -7546, -9041,
                -8429, 9713, -6317, 9171, -2384, 8879, -4844, -779, -6005, 4855, 3759, 4150, 4515,
                -81, -3646, -679, 9189, 3788, -5739, 3672, -6645, 7633, -7744, 3041, 5189, -7353,
                2226, -9308, 8393, -3449, -951, 9435, -63, -2890, -896, 1135, -1225, 4480, 3168,
                -8087, 1625, -3914, 3840, -844, -4100, 9912, 2879, 2649, -4800, 5032, -2287, 8011,
                2851, 525, -2272, 4543, -7406, 3047, 7411, 9919, -5218, -651, -2748, -1730, 165,
                1557, -2798, -9163, -8456, 6454, -7028, -8701, 6050, 9276, 8159, 5002, 3609, 1629,
                -8976, 4131, 827, 3554, 9352, -6389, 7827, 1826, 2394, -8739, 7524, -7059, -7321,
                9185, -5077, 3234, 8589, -8701, -7285, -7338, -9977, -1950, -9398, -7291, -1938,
                -2901, 1580, 8041, 2133, -9361, -4187, -3747, 876, -306, -6976, -8017, 2842, 371,
                -4249, 7163, -9309, -8326, 8650, 7057, -2567, 7307, -5861, -9120, -7731, 4655,
                -7235, 4924, -1103, 9000, -2744, 6123, 2652, 6109, 9449, -9944, -349, 3283, 2189,
                7664, -5044, -9603, 8220, -2109, -8485, 1301, -6101, 330, -700, -719, 9291, 219,
                1456, -1684, 2754, 2033, -8423, -1449, 133, 2512, 1971, -8742, -304, 5994, -63,
                -5271, -8524, 6155, 6062, -6351, 8248, 6652, 2572, 2481, -2825, 474, 1957, 4523,
                5671, -1547, 7532, -1565, -9690, 5271, 9472, -6016, -3431, 2630, 3018, -7588, 9530,
                5337, 6510, -2193, 3495, 7493, -8810, -4600, 429, 650, -2856, -6659, -6940, 5159,
                -4563, -3517, 6911, 9007, -6155, -5220, 322, 5350, 1341, -3913, -8216, 2986, 806,
                -9871, -8120, -4519, -4703, -8581, -5075, -8250, -9041, 2258, -3203, 7432, 1680,
                4082, 4947, -1820, -9118, 1554, -5097, 8214, -1122, 747, 9553, -7658, 5669, -1212,
                -5285, -8134, 8032, 4983, 3714, 4423, 4198, -3666, 8045, 9172, -1131, -2269, -326,
                -1891, -7100, 2080, -9116, -5028, 5849, 3199, 6401, -3880, -3623, -396, 2488, 8023,
                -2718, 701, -1496, -7683, 8192, 6322, 2339, -6852, 8998, -9018, 2456, -9837, 3948,
                3292, -6113, -1546, 273, -815, -4404, -470, -2899, -1909, -4829, -4046, -534,
                -5171, 1418, 7381, -6381, 137, 7063, 6040, 4447, -8359, 1779, -6094, -4928, 3372,
                -9976, 8067, -6021, -4149, -2506, 6035, -2264, 5187, -1797, -2006, 3085, 2705,
                -6018, -7349, 1480, 2581, -7059, 7814, 4967, 4811, -9176, -1390, -7707, -4468, 654,
                -7798, 2295, 6566, 8514, 5216, -3592, -4288, -2105, 9265, -6393, 8501, -5533,
                -9914, 2618, 7951, 2464, -4639, 9356, 6605, 2563, 6103, 800, -6247, 3952, -1431,
                -6563, 8564, -7036, 3048, -4112, 9016, 2488, 9673, -261, 3503, -8818, -1543, -6901,
                -2849, -2117, -6953, 1887, -3357, 5496, -2722, 3764, 6506, -6606, -6825, 3235,
                -501, 7474, 7652, -3463, 3466, 6405, 7793, 5286, -7460, -6050, -5505, 9657, 6255,
                -1825, 9637, -5906, 8219, -9296, -3151, 1650, -4213, -955, 1608, -3596, 115, 9130,
                153, 483, -5550, -7341, -2743, 3601, 5089, -4436, -950, -3392, -4245, -6041, 6154,
                8335, -4280, -4325, -3380, -3457, 3174, 9553, -8617, 9849, -714, -2318, -8060, -45,
                -9387, 2637, 7285, 4820, 6158, -1005, -974, 2876, -1839, -9395, 236, -9419, -5894,
                -1018, -6897, -4473, 2608, 9645, 591, -3776, 1520, -528, 9300, 1378, 4159, 1937,
                2866, 2671, 9841, 7235, -786, 4264, 2836, -9536, -1049, -9399, 5733, -3028, 5696,
                -2433, -5960, -8146, -4887, 9504, 8792, -5941, -8179, 5961, -965, 9551, -3227,
                -7457, -1143, -6047, -9096, -9494, 4794, 7837, 6433, -1955, -7746, 4796, -2525,
                6398, -1801, -7409, 7586, 2195, -7052, 7549, -6099, -3310, 9504, -5300, -3824,
                9214, 490, 1963, -9584, -6094, 3861, -4955, -5535, 8350, -8385, 7337, 1451, 2637,
                -8899, 5605, 4679, 8305, -799, 4464, -3077, -3612, -6020, 7039, 9691, 8937, -4870,
                6637, -6986, -7981, 8160, -2470, 6401, 7022, 656, 9751, 1958, -8711, -7133, -4338,
                -5460, 8488, -203, -5525, -2042, 4109, -7698, 3126, -8487, -756, -9885, -6107,
                4894, 795, 5140, -140, 3055, -472, 3019, 3629, 2989, -8395, -4659, 6648, 4180, 898,
                -5400, 9206, 866, 7753, 7428, -4584, 8710, -140, -2027, -2256, 175, -7609, 7486,
                -2609, -1320, 9747, 8765, -4279, -3936, 4023, 6488, 1133, -2577, -2277, 3617, 4153,
                3442, 1731, 6231, -9846, -8463, 1903, 2913, 2675, 8156, 2753, 4531, 8196, 1067,
                -6592, 5148, 8757, -5497, -1200, 4752, -3653, 5830, -1574, 5811, -2272, 9496,
                -6612, -8673, 9704, -4933, -1793, 6327, 651, 4022, 8465, 4715, 5933, -6105, -3079,
                1892, 2718, -737, 2333, 6171, 341, 1066, -1049, -6556, -1152, -2148, 5889, -8901,
                1279, 9454, -9865, 801, -5031, 8188, -165, -8970, -3069, 6925
            ]),
            7412694
        );
    }
}
