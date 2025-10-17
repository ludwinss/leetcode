fn main() {
    let mut a = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut a);
    assert_eq!(a, vec![0, 0, 1, 1, 2, 2]);

    let mut b = vec![2, 0, 1];
    sort_colors(&mut b);
    assert_eq!(b, vec![0, 1, 2]);

    println!("All tests passed!");
}

pub fn sort_colors(nums: &mut Vec<i32>) {
    const K: usize = 3;

    let mut counts = [0usize; K];

    for &value in nums.iter() {
        counts[value as usize] += 1;
    }

    for i in 1..K {
        counts[i] += counts[i - 1];
    }

    let mut output = vec![0; nums.len()];

    for &x in nums.iter().rev() {
        counts[x as usize] -= 1;

        output[counts[x as usize]] = x;
    }

    nums.copy_from_slice(&output);
}
