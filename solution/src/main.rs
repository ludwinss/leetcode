fn main() {
    assert_eq!(
        remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
        5
    );
    assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    println!("All tests passed!");
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut write: usize = 1;

    for read in 1..nums.len() {
        if nums[read] != nums[read - 1] {
            nums[write] = nums[read];
            write += 1;
        }
    }
    write as i32
}
