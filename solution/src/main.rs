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

    let mut s_array: Vec<char> = s.chars().collect();
    let mut t_array: Vec<char> = t.chars().collect();

    s_array.sort();
    t_array.sort();

   s_array == t_array
}

