use std::collections::HashSet;

fn main() {
    assert_eq!(is_happy(19), true);
    assert_eq!(is_happy(2), false);

    println!("All test passed!");
}

pub fn is_happy(mut n: i32) -> bool {
    let mut memo_digits = HashSet::new();
    loop {
        let mut tmp_value = 0;

        while n > 0 {
            let digit = n % 10;
            tmp_value += digit * digit;
            n /= 10;
        }

        if tmp_value == 1 {
            return true;
        }

        if memo_digits.contains(&tmp_value) {
            return false;
        }

        memo_digits.insert(tmp_value);

        n = tmp_value;
    }
}
