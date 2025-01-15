fn main() {
    assert_eq!(single_number(vec![1, 2, 3, 1, 3]), 2);
    assert_eq!(single_number(vec![1, 1, 5, 3, 3, 5, 4]), 4);
    println!("All test passed!");
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, &next| acc ^ next)
}
