use std::collections::HashMap;

fn main() {
    assert_eq!(
        is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    assert_eq!(is_anagram("a".to_string(), "ab".to_string()), false);
    assert_eq!(is_anagram("".to_string(), "".to_string()), true);
    println!("All test passed!");
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut memo_anagram: HashMap<char, i32> = HashMap::new();

    for char in s.chars() {
        memo_anagram
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for char in t.chars() {
        match memo_anagram.get_mut(&char) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    memo_anagram.remove(&char);
                }
            }
            None => return false,
        }
    }

    memo_anagram.is_empty()
}
