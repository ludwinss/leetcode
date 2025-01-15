use std::collections::HashMap;

fn main() {
    assert_eq!(single_number(vec![1, 2, 3, 1, 3]), 2);
    assert_eq!(single_number(vec![1, 1, 5, 3, 3, 5, 4]), 4);
    println!("All test passed!");
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut memoization: HashMap<i32, i32> = HashMap::new();

    for digit in &nums {
        *memoization.entry(*digit).or_insert(0) += 1;
    }

    for (digit, value) in &memoization {
        if *value == 1 {
            return *digit;
        }
    }

    0
}
