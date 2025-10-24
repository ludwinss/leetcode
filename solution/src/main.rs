fn main() {
    assert_eq!(number_of_cuts(1), 0);
    assert_eq!(number_of_cuts(2), 1);
    assert_eq!(number_of_cuts(3), 3);
    assert_eq!(number_of_cuts(4), 2);
    assert_eq!(number_of_cuts(5), 5);
    assert_eq!(number_of_cuts(6), 3);
    assert_eq!(number_of_cuts(7), 7);
    assert_eq!(number_of_cuts(8), 4);
    assert_eq!(number_of_cuts(10), 5);
    assert_eq!(number_of_cuts(100), 50);
    println!("All tests passed!");
}

pub fn number_of_cuts(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    match n % 2 {
        0 => n / 2,
        _ => n,
    }
}
