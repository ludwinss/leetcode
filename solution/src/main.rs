fn main() {
    assert_eq!(max_power("abbbcccddddd".to_string()), 5);
    assert_eq!(max_power("leetcode".to_string()), 2);
    assert_eq!(max_power("quefuecausagaaa".to_string()), 3);
    assert_eq!(max_power("tourist".to_string()), 1);
    println!("All test passed!");
}

pub fn max_power(s: String) -> i32 {
    s.chars()
        .fold((0, 1, None), |(count, mx, last_value), next| {
            if Some(next) == last_value {
                (count + 1, mx.max(count + 1), Some(next))
            } else {
                (1, mx, Some(next))
            }
        })
        .1 as i32
}
