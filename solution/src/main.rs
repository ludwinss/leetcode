use std::collections::HashSet;

fn main() {
    let ex1 = vec![46, 8, 2, 4, 1, 4, 10, 2, 4, 10, 2, 5, 7, 3, 1];
    let ex2 = vec![1, 2, 3, 2, 5];
    let ex3 = vec![3, 4, 5, 1, 12, 14, 13];
    let ex4 = vec![38];
    let ex5 = vec![1, 2, 3, 9, 2, 10, 8, 3, 10, 2];

    assert_eq!(missing_integer(ex1), 47);
    assert_eq!(missing_integer(ex2), 6);
    assert_eq!(missing_integer(ex3), 15);
    assert_eq!(missing_integer(ex4), 39);
    assert_eq!(missing_integer(ex5), 6);
}

pub fn missing_integer(x: Vec<i32>) -> i32 {
    // Honestly the problem sucks
    let a = x.iter().cloned().collect::<HashSet<_>>();
    let mut i = 1;
    let mut total = x[0];
    while i < x.len() && x[i] == x[i - 1] + 1 {
        total += x[i];
        i += 1;
    }
    while a.contains(&total) {
        total += 1
    }
    total
}
