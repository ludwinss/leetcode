fn main() {
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    println!("All tests passed!");
}

pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }

    let (mut prev, mut next) = (1, 2);

    for _ in 3..n {
        let curr = prev + next;

        prev = next;
        next = curr;
    }

    next
}
