fn main() {
    let input = 2;
    assert_eq!(my_sqrt(input), 1);

    let input = 8;
    assert_eq!(my_sqrt(input), 2);

    let input = 10;
    assert_eq!(my_sqrt(input), 3);

    let input = 2147395599;
    assert_eq!(my_sqrt(input), 46339);

    println!("All test passed!");
}

pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    let mut guess = x as f64 / 2.0;

    loop {
        let next_guess = (guess + x as f64 / guess) / 2.0;

        if (next_guess - guess).abs() < 1e-6 {
            break;
        }

        guess = next_guess;
    }

    let result = guess as i32;

    if result * result > x {
        result - 1
    } else {
        result
    }
}
