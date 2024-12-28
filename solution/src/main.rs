fn main() {
    assert_eq!(is_happy(19), true);
    assert_eq!(is_happy(2), false);

    println!("All test passed!");
}

pub fn is_happy(mut n: i32) -> bool {
    let mut last_value = 0;
    let mut is_first = true;
    loop {
        let mut tmp_value = 0;

        while n > 0 {
            let digit = n % 10;
            tmp_value += digit * digit;
            n /= 10;
        }

        n = tmp_value;

        if last_value == n {
            return false;
        }

        if is_first {
            last_value = tmp_value;
            is_first = false;
        }

        if tmp_value == 1 {
            return true;
        }
    }
}
