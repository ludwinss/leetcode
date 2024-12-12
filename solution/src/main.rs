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
    let mut is_initiate: bool = false;
    let mut result: i32 = 0;
    let mut is_negative: bool = false;

    for char in s.chars() {
        if char.eq(&' ') {
            if is_initiate {
                break;
            }
            continue;
        }

        if char.eq(&'0')
            || char.eq(&'1')
            || char.eq(&'2')
            || char.eq(&'3')
            || char.eq(&'4')
            || char.eq(&'5')
            || char.eq(&'6')
            || char.eq(&'7')
            || char.eq(&'8')
            || char.eq(&'9')
        {
            let value_parse = char.to_string().parse::<i32>().unwrap_or(0);

            if let Some(value_to_mul) = result.checked_mul(10) {
                if let Some(value_to_sum) = value_to_mul.checked_add(value_parse) {
                    result = value_to_sum;
                } else {
                    return if is_negative {
                        i32::min_value()
                    } else {
                        i32::max_value()
                    };
                }
            } else {
                return if is_negative {
                    i32::min_value()
                } else {
                    i32::max_value()
                };
            }
        } else if char.eq(&'-') {
            if is_initiate {
                break;
            }
            is_negative = true;
        } else if char.eq(&'+') {
            if is_initiate {
                break;
            }
            is_negative = false;
        } else {
            break;
        }
        is_initiate = true;
    }
    println!("result: {}", result);

    result * if is_negative { -1 } else { 1 }
}
