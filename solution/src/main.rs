fn main() {
    assert_eq!(is_power_of_two(1), true);
    assert_eq!(is_power_of_two(16), true);
    assert_eq!(is_power_of_two(3), false);
    assert_eq!(is_power_of_two(2147483647), false);
    println!("All test passed!");
}

pub fn is_power_of_two(n: i32) -> bool {
    let mut next_pow = 1;
    if n == 1 {
        return true;
    }

    if n % 2 != 0 {
        return false;
    }

    loop {
        if n.abs() == next_pow {
            return true;
        }
        next_pow <<= 1;
        if next_pow > n {
            break;
        }
    }
    println!("left:{}, right:{}", n << 1, n);
    false
}
