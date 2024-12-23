fn main() {
    let input = "Hello World".to_string();
    assert_eq!(length_of_last_word(input), 5);

    let input = "   fly me   to   the moon  ".to_string();
    assert_eq!(length_of_last_word(input), 4);

    let input = "luffy is still joyboy".to_string();
    assert_eq!(length_of_last_word(input), 6);

    println!("All test passed!");
}
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .last()
        .map_or(0, |last_word| last_word.len() as i32)
}
