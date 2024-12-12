fn main() {
    let input = "42".to_string();
    assert_eq!(my_atoi(input), 42);

    let input = "   -42".to_string();
    assert_eq!(my_atoi(input), -42);

    let input = "4193 with words".to_string();
    assert_eq!(my_atoi(input), 4193);

    let input = "words and 987".to_string();
    assert_eq!(my_atoi(input), 0);

    let input = "0-1".to_string();
    assert_eq!(my_atoi(input), 0);

    let input = "+1".to_string();
    assert_eq!(my_atoi(input), 1);

    let input = "   +0 123".to_string();
    assert_eq!(my_atoi(input), 0);

    println!("All test passed!");
}

pub fn my_atoi(s: String) -> i32 {
    let s_trim = s.trim_start();
    let mut result: i32 = 0;
    let mut is_negative: bool = false;

    for (idx, char) in s_trim.chars().enumerate() {
        match char {
            '-' if idx == 0 => is_negative = true,

            '+' if idx == 0 => continue,

            '0'..='9' => {
                let digit = (char as u8 - b'0') as i32;

                result = match result
                    .checked_mul(10)
                    .and_then(|val| val.checked_add(digit))
                {
                    Some(val) => val,
                    None => return if is_negative { i32::MIN } else { i32::MAX },
                }
            }
            _ => break,
        }
    }

    result * if is_negative { -1 } else { 1 }
}
