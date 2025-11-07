fn main() {
    assert_eq!(arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    assert_eq!(arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    println!("All tests passed!");
}

pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    use std::collections::HashSet;
    let memo: HashSet<i32> = nums.iter().copied().collect();

    nums.iter()
        .filter(|value| memo.contains(&(*value + diff)) && memo.contains(&(*value + (diff * 2))))
        .count() as i32
}
