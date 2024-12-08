use std::collections::HashMap;

fn main() {
    let input = vec![4, 1, 2];
    let input2 = vec![1, 3, 4, 2];

    assert_eq!(next_greater_element(input, input2), vec![-1, 3, -1]);

    let input = vec![2, 4];
    let input2 = vec![1, 2, 3, 4];

    assert_eq!(next_greater_element(input, input2), vec![3, -1]);
}

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut memoize_result: HashMap<i32, i32> = HashMap::new();
    let mut monotic_stask: Vec<i32> = Vec::new();

    for &num_to_compare in nums2.iter().rev() {
        while let Some(&header_value) = monotic_stask.last() {
            println!("header_value: {:?}, num_to_compare: {:?}", header_value, num_to_compare);
            if header_value <= num_to_compare {
                monotic_stask.pop()
            } else {
                break;
            };
        }

        let next_greater = if let Some(&header_value) = monotic_stask.last() {
            header_value
        } else {
            -1
        };

        memoize_result.insert(num_to_compare, next_greater);

        monotic_stask.push(num_to_compare);
    }
    println!("-====={:?}", memoize_result);

    nums1.iter().map(|&num| *memoize_result.get(&num).unwrap_or(&-1)).collect()
}
