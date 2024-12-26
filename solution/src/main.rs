fn main() {
    assert_eq!(is_perfect_square(16), true);
    assert_eq!(is_perfect_square(14), false);
    assert_eq!(is_perfect_square(1), true);
    assert_eq!(is_perfect_square(4), true);
    assert_eq!(is_perfect_square(2147395600), true);
    assert_eq!(is_perfect_square(2_147_483_647), false);

    println!("All test passed!");
}
pub fn is_perfect_square(num: i32) -> bool {
    let mut left = 1;
    let mut right = num;

    while left <= right {
        let mid = left + (right - left) / 2;

        let square = mid as i64 * mid as i64;

        if square == num as i64 {
            return true;
        } else if square > num as i64 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    false
}
