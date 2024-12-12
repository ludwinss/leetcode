fn main() {
    let input = 123;
    assert_eq!(reverse(input), 321);

    let input = -123;

    assert_eq!(reverse(input), -321);

    println!("All test passed!");
}

pub fn reverse(x: i32) -> i32 {
    let is_negative = x < 0;

    let x = x.abs();

    let x_invert: String = x
        .to_string()
        .chars()
        .rev()
        .fold(String::new(), |mut acc, char| {
            acc.push(char);
            acc
        });

    let x_invert = x_invert.parse::<i32>().unwrap_or(0);

    if is_negative {
        return x_invert * -1;
    }

    x_invert
}
