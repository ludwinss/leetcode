use std::collections::HashMap;

fn main() {
    assert_eq!(climb_stairs(1), 1);
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(4), 5);
    println!("All tests passed!");
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();

    fn dn(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n <= 2 {
            return n;
        }

        if let Some(memoized) = memo.get(&n) {
            return *memoized;
        }

        let n_1 = dn(n - 1, memo);
        let n_2 = dn(n - 2, memo);

        let sum = n_1 + n_2;
        memo.insert(n, sum);

        sum
    }
    dn(n, &mut memo)
}
