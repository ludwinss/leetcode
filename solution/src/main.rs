use std::{cmp::Ordering, collections::HashMap};

fn main() {
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(19), "XIX");
    assert_eq!(int_to_roman(4), "IV");
    assert_eq!(int_to_roman(9), "IX");
    assert_eq!(int_to_roman(45), "XLV");
    assert_eq!(int_to_roman(40), "XL");
    assert_eq!(int_to_roman(91), "XCI");
    assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    assert_eq!(int_to_roman(1994), "MCMXCIV");
    assert_eq!(int_to_roman(3999), "MMMCMXCIX");

    println!("All test passed!");
}

pub fn int_to_roman(mut num: i32) -> String {
    let map_romans = HashMap::from([
        (1, ("I", "V")),
        (2, ("X", "L")),
        (3, ("C", "D")),
        (4, ("M", "")),
    ]);

    let mut idx = 1;

    let mut results = "".to_string();

    while num > 0 {
        let digit = num % 10;

        let (left, right) = map_romans.get(&idx).unwrap();

        num /= 10;

        idx += 1;

        match digit.cmp(&5) {
            Ordering::Less => {
                if digit == 4 {
                    results.insert_str(0, &right);
                    results.insert_str(0, &left);
                } else {
                    results.insert_str(0, &left.repeat((digit) as usize));
                }
                continue;
            }
            Ordering::Equal => {
                results.insert_str(0, &right);
                continue;
            }
            Ordering::Greater => {
                if digit == 9 {
                    let (left_b, _) = map_romans.get(&(idx)).unwrap();
                    results.insert_str(0, &left_b);
                    results.insert_str(0, &left);
                } else {
                    results.insert_str(0, &left.repeat((digit - 5) as usize));
                    results.insert_str(0, &right);
                }
                continue;
            }
        }
    }

    results
}
