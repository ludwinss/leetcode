fn main() {
    assert_eq!(is_ugly(1), true);
    assert_eq!(is_ugly(6), true);
    assert_eq!(is_ugly(14), false);
    println!("All test passed!");
}

pub fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    for factor in [5, 3, 2] {
        while n % factor == 0 {
            n /= factor;
        }
    }
    n == 1
}
