fn main() {
    let num = vec![2, 1, 5];
    let k = 806;

    let solutions = add_to_array_form(num, k);
    println!("{:?}", solutions);
}

pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let mut carry = k;
    let mut result = Vec::new();

    for digit in num.iter().rev() {
        carry += *digit;
        result.push(carry % 10);
        carry /= 10;
    }

    while carry > 0 {
        result.push(carry % 10);
        carry /= 10;
    }

    result.reverse();

    result
}
