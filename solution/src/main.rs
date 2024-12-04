fn main() {
    let input = vec![1, 9, 9];

    let solutions = plus_one(input);
    println!("{:?}", solutions);
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    let mut result: Vec<i32> = digits
        .into_iter()
        .rev()
        .map(|digit| {
            let sum = digit + carry;
            carry = sum / 10;
            sum % 10
        })
        .collect();

    if carry > 0 {
        result.push(carry);
    }

    result.reverse();

    result
}
