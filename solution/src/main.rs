fn main() {
    assert_eq!(
        is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(is_palindrome("race a car".to_string()), false);
    assert_eq!(is_palindrome("".to_string()), true);
    assert_eq!(is_palindrome("0P".to_string()), false);

    println!("All test passed!");
}

pub fn is_palindrome(s: String) -> bool {
    let low_palindrome = s.to_lowercase();

    let without_spaces = low_palindrome
        .chars()
        .filter(|char| char.is_alphanumeric())
        .map(|char| char.to_ascii_lowercase());

    without_spaces.clone().eq(without_spaces.rev())
}
