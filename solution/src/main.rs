use std::cmp::{max, min};

fn main() {
    let example = String::from("aaaaabbc");
    assert_eq!(max_difference(example), 3);

    let example = String::from("abcabcab");
    assert_eq!(max_difference(example), 1);

    let example = String::from("aabbbc");
    assert_eq!(max_difference(example), 1);

    let example = String::from("aaaab");
    assert_eq!(max_difference(example), -3);

    let example = String::from("aabbc");
    assert_eq!(max_difference(example), -1);

    let example = String::from("aaabbc");
    assert_eq!(max_difference(example), 1);

    let example = String::from("zzzzxxxyyy");
    assert_eq!(max_difference(example), -1);

    println!("All test passed!");
}

pub fn max_difference(s: String) -> i32 {
    let mut vector_s: Vec<i32> = vec![0; 26];
    let a = b'a';

    s.as_bytes()
        .iter()
        .for_each(|&char| vector_s[(char - a) as usize] += 1);

    let mut odd = 0;
    let mut even = i32::MAX;

    for &value in &vector_s {
        if value % 2 == 1 {
            odd = max(odd, value);
        } else if value != 0 {
            even = min(even, value);
        }
    }

    if even == i32::MAX {
        odd
    } else {
        odd - even
    }
}
