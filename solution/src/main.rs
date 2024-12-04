fn main() {
    let num = String::from("(1)+((2))+(((3)))");

    let solutions = max_depth(num);
    println!("{:?}", solutions);
}

pub fn max_depth(s: String) -> i32 {
    let mut max_value: i32 = 0;
    let mut stack_parentheses = Vec::new();

    for strin in s.chars() {
        if strin == '(' {
            stack_parentheses.push(strin);
        }
        if strin == ')' {
            stack_parentheses.pop();
        }

        if max_value < stack_parentheses.len() as i32 {
            max_value = stack_parentheses.len() as i32;
        }
    }
    max_value
}
