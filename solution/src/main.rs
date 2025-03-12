use std::collections::HashMap;

fn main() {
    let input = vec![2, 2, 3, 2];
    assert_eq!(single_number(input), 3);

    let input = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(single_number(input), 99);

    println!("All test passed!");
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut hash_number: HashMap<i32, i32> = HashMap::new();

    for num in &nums {
        *hash_number.entry(*num).or_insert(0) += 1;
    }

    *hash_number
        .iter()
        .find(|(_, &value)| value == 1)
        .map(|(value, _)| value)
        .unwrap_or(&0)
}
