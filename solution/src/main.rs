fn main() {
    assert_eq!(broken_calc(2, 3), 2);
    assert_eq!(broken_calc(5, 8), 2);
    assert_eq!(broken_calc(3, 10), 3);
    println!("All tests passed!");
}

pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
    let mut count = 0;
    while start_value < target {
        if target % 2 != 0 {
            target += 1;
        } else {
            target /= 2;
        }
        count += 1;
    }

    count + (start_value - target)
}
