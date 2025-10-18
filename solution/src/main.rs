fn main() {
    assert_eq!(
        vowel_strings(
            vec!["are".to_string(), "amy".to_string(), "u".to_string(),],
            0,
            2
        ),
        2
    );

    assert_eq!(
        vowel_strings(
            vec![
                "hey".to_string(),
                "aeo".to_string(),
                "mu".to_string(),
                "ooo".to_string(),
                "artro".to_string(),
            ],
            1,
            4
        ),
        3
    );

    println!("All tests passed!");
}

pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    fn is_vowel(test: char) -> bool {
        matches!(test, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    words[(left as usize)..=(right as usize)]
        .iter()
        .filter(|word| {
            let mut chars_words = word.chars();

            match (chars_words.next(), chars_words.next_back()) {
                (Some(l), Some(r)) => is_vowel(l) && is_vowel(r),
                (Some(l), None) => is_vowel(l),
                _ => false,
            }
        })
        .count() as i32
}
