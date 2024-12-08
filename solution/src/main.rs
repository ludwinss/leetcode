fn main() {
    let input = vec![1, 2, 1];

    assert_eq!(next_greater_element(input), vec![2, -1, 2]);

    let input = vec![1, 2, 3, 4, 3];

    assert_eq!(next_greater_element(input), vec![2, 3, 4, -1, 4]);
    println!("All test passed!");
}

pub fn next_greater_element(nums1: Vec<i32>) -> Vec<i32> {
    // avoid use clone an duplicate the memory
    //let circle_elements = Vec::from([nums1.clone(), nums1.clone()].concat());
    let total_len = nums1.len();
    let mut stack: Vec<i32> = Vec::with_capacity(total_len);
    let mut result: Vec<i32> = vec![-1; total_len];

    for idx in (0..total_len * 2).rev() {
        // use properties o base number to extract the number in nums1
        let num = nums1[idx % total_len];
        while let Some(&header_value) = stack.last() {
            if header_value <= num {
                stack.pop();
            } else {
                break;
            }
        }

        // if idx is less than total_len, then we can store the result
        if idx < total_len {
            if let Some(&total) = stack.last() {
                result[idx] = total;
            }
        }

        stack.push(num)
    }

    result
}
