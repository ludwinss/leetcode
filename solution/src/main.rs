use std::collections::HashMap;

fn main() {
    let example = String::from("aaaaabbc");
    assert_eq!(max_difference(example), 3);

    let example = String::from("abcabcab");
    assert_eq!(max_difference(example), 1);

    println!("All test passed!");
}

pub fn max_difference(s: String) -> i32 {
    let mut hash_value: HashMap<u8, i32> = HashMap::new();

    for &value in s.as_bytes().iter() {
        hash_value
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let (min, max) = hash_value
        .values()
        .fold((i32::MAX, i32::MIN), |(min, max), &next| {
            if next == 1 {
                (min, max)
            } else {
                (min.min(next), max.max(next))
            }
        });
    max - min
}
