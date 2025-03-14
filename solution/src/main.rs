fn main() {
    let input = vec![2, 2, 3, 2];
    assert_eq!(single_number(input), 3);

    let input = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(single_number(input), 99);

    println!("All test passed!");
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut input1 = 0;
    let mut input2 = 0;
    for num in &nums {
        input1 = (input1 ^ num) & !input2;
        input2 = (input2 ^ num) & !input1;
    }
    input1
}
