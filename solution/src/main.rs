fn main() {
    let num = String::from("(1)+((2))+(((3)))");

    let solutions = max_depth(num);
    println!("{:?}", solutions);
}

pub fn max_depth(s: String) -> i32 {
    let mut max_value: i32 = 0;
    let mut current_depth = 0;

    for char in s.chars() {
        match char {
            '(' => {
                current_depth += 1;
                max_value = max_value.max(current_depth);
            }
            ')' => {
                current_depth -= 1;
            }
            _ => continue
        }
    }
    max_value
}
