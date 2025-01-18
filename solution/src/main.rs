fn main() {
    assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 0, 0, 1, 1, 1]), 3);
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 1, 0, 0, 1]), 3);
    assert_eq!(find_max_consecutive_ones(vec![0, 1, 1, 0, 0, 1]), 2);
    assert_eq!(find_max_consecutive_ones(vec![0, 0, 1, 1, 0, 1]), 2);
    println!("All test passed!");
}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    nums.iter().fold((0, 0), |(count, mx), next| match next {
        1 => (count + 1, mx.max(count + 1)),
        _ => (0, mx),
    }).1
}
