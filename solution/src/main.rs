fn main() {
    let input = vec![1, 2, 1];

    assert_eq!(next_greater_element(input), vec![2, -1, 2]);

    let input = vec![1, 2, 3, 4, 3];

    assert_eq!(next_greater_element(input), vec![2, 3, 4, -1, 4]);
}

pub fn next_greater_element(nums1: Vec<i32>) -> Vec<i32> {
    let circle_elements = Vec::from([nums1.clone(), nums1.clone()].concat());
    let mut stack: Vec<i32> = vec![];
    let mut result: Vec<i32> = vec![-1; circle_elements.len()];

    for (idx, &num) in circle_elements.iter().enumerate().rev() {
        while let Some(&header_value) = stack.last() {
            if header_value <= num {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&top) = stack.last() {
            result[idx] = top
        }

        stack.push(num)
    }

    result.truncate(nums1.len());
    result
}
