fn main() {
    assert_eq!(is_power_of_two(1), true);
    assert_eq!(is_power_of_two(16), true);
    assert_eq!(is_power_of_two(3), false);
    assert_eq!(is_power_of_two(2147483647), false);
    assert_eq!(is_power_of_two(-2147483647), false);
    assert_eq!(is_power_of_two(-16), false);
    println!("All test passed!");
}

pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}
