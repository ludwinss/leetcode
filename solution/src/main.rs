fn main() {
    assert_eq!(min_operations(vec![1, 2, 7], vec![4, 5, 3]), 1);
    assert_eq!(min_operations(vec![2, 3, 4, 5, 9], vec![8, 8, 4, 4, 4]), 2);
    assert_eq!(min_operations(vec![1, 5, 4], vec![2, 5, 3]), -1);
    println!("All tests passed!");
}
pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();

    fn cost(nums1: &[i32], nums2: &[i32], max1: i32, max2: i32, extra: i32) -> Option<i32> {
        let mut swap = extra;
        for index in 0..nums1.len() - 1 {
            let (x, y) = (nums1[index], nums2[index]);

            if x <= max1 && y <= max2 {
                continue;
            }

            if x <= max2 && y <= max1 {
                swap += 1;
            } else {
                return None;
            }
        }
        Some(swap)
    }
    let keep = cost(&nums1, &nums2, nums1[n - 1], nums2[n - 1], 0);
    let change_last = cost(&nums1, &nums2, nums2[n - 1], nums1[n - 1], 1);

    match (keep, change_last) {
        (Some(x), Some(y)) => x.min(y),
        (Some(x), None) => x,
        (None, Some(y)) => y,
        _ => -1,
    }
}
