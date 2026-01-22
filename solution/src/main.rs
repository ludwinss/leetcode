fn main() {
    assert_eq!(max_capacity(vec![4, 8, 5, 3], vec![1, 5, 7, 2], 8), 8);

    assert_eq!(max_capacity(vec![3, 5, 7, 4], vec![2, 4, 3, 6], 5), 6);

    println!("All tests passed!");
}

pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
    let mut mapped: Vec<(&i32, i32)> = costs.iter().zip(capacity).map(|slc| slc).collect();
    mapped.sort_unstable_by_key(|a| *a);

    let mut left = 0;
    let mut right = mapped.len() - 1;

    let mut best = 0;

    while left < right {
        let middle = left + (right - left) / 2;

        let value = (mapped[middle].0 + mapped[middle].1) as i32;
        println!("{:?}", value);
        if value > budget as i32 {
            best = value;
        }

        if (middle as i32) < value {
            left = right - left / 2 + 1;
        } else {
            right = right - left / 2 - 1;
        }
    }

    print!("{:?}", best);

    0
}
