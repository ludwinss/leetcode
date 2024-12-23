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
    let array_bytes = s.trim_end().as_bytes();

    let mut i = array_bytes.len() - 1;

    while i > 0 {
        i -= 1;
        if array_bytes[i] == b' ' {
            return (array_bytes.len() - i - 1) as i32;
        }
    }

    0
}
