fn main() {
    assert_eq!(maximum_or(vec![12, 9], 1), 30);
    assert_eq!(maximum_or(vec![8, 1, 2], 2), 35);
    println!("All tests passed!");
}

pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
    let len: usize = nums.len();
    let nums_parsed: Vec<i64> = nums.iter().map(|&a| a as i64).collect();

    //GREEDY CALC
    let mut prefix: Vec<i64> = vec![0i64; len + 1];
    let mut sufix: Vec<i64> = vec![0i64; len + 1];

    for index in 1..=len {
        prefix[index] = prefix[index - 1] | nums_parsed[index - 1];
    }

    for index in (0..len).rev() {
        sufix[index] = sufix[index + 1] | nums_parsed[index];
    }

    (0..len)
        .map(|index| prefix[index] | sufix[index + 1] | nums_parsed[index] << k)
        .max()
        .unwrap()
}
