fn main() {
    assert_eq!(average(vec![4000, 3000, 1000, 2000]), 2500.0);
    assert_eq!(average(vec![1000, 2000, 3000]), 2000.0);
    println!("All tests passed!");
}

pub fn average(salary: Vec<i32>) -> f64 {
    let (mut max, mut min, mut sum): (i32, i32, i32) = (i32::MIN, i32::MAX, 0);

    for value in &salary {
        sum += value;
        max = max.max(*value);
        min = min.min(*value);
    }

    (sum - max - min) as f64 / (salary.len() - 2) as f64
}
