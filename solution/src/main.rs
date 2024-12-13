fn main() {
    let input = vec![73, 74, 75, 71, 69, 72, 76, 73];
    assert_eq!(daily_temperatures(input), vec![1, 1, 4, 2, 1, 1, 0, 0]);

    let input = vec![30, 40, 50, 60];
    assert_eq!(daily_temperatures(input), vec![1, 1, 1, 0]);

    let input = vec![30, 60, 90];
    assert_eq!(daily_temperatures(input), vec![1, 1, 0]);

    println!("All test passed!");
}
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack: Vec<(i32, i32)> = Vec::with_capacity(temperatures.len());

    for (idx, &temperature) in temperatures.iter().rev().enumerate() {
        while let Some(&greatest_temperature) = stack.last() {
            if greatest_temperature.1 <= temperature {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&greatest_temperature) = stack.last() {
            result[idx] = idx as i32 - greatest_temperature.0;
        }

        stack.push((idx as i32, temperature));
    }
    result.reverse();

    println!("{:?}", result);

    result
}
