fn main() {
    assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 0, 0, 1, 1, 1]), 3);
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 1, 0, 0, 1]), 3);
    assert_eq!(find_max_consecutive_ones(vec![0, 1, 1, 0, 0, 1]), 2);
    assert_eq!(find_max_consecutive_ones(vec![0, 0, 1, 1, 0, 1]), 2);
    println!("All test passed!");
}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut count = 0;

    nums.iter().for_each(|&num| {
        if num == 1 {
            count += 1;
        } else {
            count = 0;
        }
        if count > max {
            max = count;
        }
    });

    max
}
