use std::cmp::Ordering;

fn main() {
    assert_eq!(longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);

    assert_eq!(longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);

    assert_eq!(longest_monotonic_subarray(vec![3, 2, 1]), 3);

    assert_eq!(longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    assert_eq!(longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    assert_eq!(longest_monotonic_subarray(vec![3, 2, 1]), 3);

    assert_eq!(longest_monotonic_subarray(vec![1]), 1);
    assert_eq!(longest_monotonic_subarray(vec![1, 2]), 2);
    assert_eq!(longest_monotonic_subarray(vec![2, 1]), 2);
    assert_eq!(longest_monotonic_subarray(vec![1, 1]), 1);

    assert_eq!(longest_monotonic_subarray(vec![1, 2, 3, 2, 1]), 3);
    assert_eq!(longest_monotonic_subarray(vec![1, 2, 3, 4, 5]), 5);
    assert_eq!(longest_monotonic_subarray(vec![5, 4, 3, 2, 1]), 5);
    assert_eq!(longest_monotonic_subarray(vec![1, 2, 2, 3, 4]), 3);
    assert_eq!(longest_monotonic_subarray(vec![10, 9, 8, 8, 7, 6]), 3);

    assert_eq!(longest_monotonic_subarray(vec![1, 3, 2, 4, 6, 5, 4]), 3);
    assert_eq!(longest_monotonic_subarray(vec![1, 5, 9, 2, 3, 4, 1]), 3);
    assert_eq!(longest_monotonic_subarray(vec![5, 6, 5, 6, 5, 6, 5]), 2);

    assert_eq!(longest_monotonic_subarray(vec![2, 2, 2, 1, 2]), 2);
    assert_eq!(longest_monotonic_subarray(vec![1, 3, 3, 2, 2, 1]), 2);

    assert_eq!(longest_monotonic_subarray((1..=50).collect()), 50);
    assert_eq!(longest_monotonic_subarray((1..=50).rev().collect()), 50);

    println!("All tests passed!");
}

pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let (mut dec, mut inc, mut best) = (1, 1, 1);

    for tuple in nums.windows(2) {
        match tuple[0].cmp(&tuple[1]) {
            Ordering::Equal => {
                inc = 1;
                dec = 1;
            }
            Ordering::Greater => {
                inc = 1;
                dec += 1;
            }
            Ordering::Less => {
                inc += 1;
                dec = 1;
            }
        }

        best = best.max(dec.max(inc));
    }

    best
}
