fn main() {
    assert_eq!(is_power_of_two(1), true);
    assert_eq!(is_power_of_two(16), true);
    assert_eq!(is_power_of_two(3), false);
    println!("All test passed!");
}

pub fn is_power_of_two(n: i32) -> bool {
    if n == 1 {
        return true;
    }

    if n % 2 !=0 {
        return false;
    }

    let mut left = 0;
    let mut right = n;

    while left <= right {
        let mid: i32 = left + (right - left)/2;

        let pow_value = (2 as u32).pow(mid as u32) as i32;

        if n.abs() == pow_value {
            return true;
        }

        if pow_value > n.abs() {
            right = mid -1;
        } else {
            left = mid +1;
        }

        println!("{:?}, {:?}", left, right);
    }

    false
}
