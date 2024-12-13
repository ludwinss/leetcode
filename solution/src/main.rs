fn main() {
    let input = vec![73, 74, 75, 71, 69, 72, 76, 73];
    assert_eq!(daily_temperatures(input), vec![1, 1, 4, 2, 1, 1, 0, 0]);

    let input = vec![30, 40, 50, 60];
    assert_eq!(daily_temperatures(input), vec![1, 1, 1, 0]);

    let input = vec![30, 60, 90];
    assert_eq!(daily_temperatures(input), vec![1, 1, 0]);

    let input = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
    assert_eq!(
        daily_temperatures(input),
        vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
    );

    println!("All test passed!");
}
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = Vec::with_capacity(temperatures.len());

    for (idx, &temperature) in temperatures.iter().enumerate() {
        while let Some(&greatest_idx) = stack.last() {
            if temperatures[greatest_idx] < temperature {
                result[greatest_idx] = (idx - greatest_idx) as i32;
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(idx);
    }

    result
}
