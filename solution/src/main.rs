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
    let romans = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut results = String::new();

    for &(val_roman, roman) in &romans {
        while val_roman <= num {
            num -= val_roman;
            results.push_str(roman);
        }
    }

    results
}
