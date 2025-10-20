fn main() {
    assert_eq!(maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3), 8);

    assert_eq!(maximum_tastiness(vec![1, 3, 1], 2), 2);

    assert_eq!(maximum_tastiness(vec![7, 7, 7, 7], 2), 0);

    println!("All tests passed!");
}

pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
    price.sort();

    fn is_candidate(value_expect: i32, input: &[i32], k: i32) -> bool {
        let mut taken = 1;
        let mut last = input[0];

        for &value in input.iter().skip(1) {
            if value - last >= value_expect {
                taken += 1;
                last = value;
                if taken >= k {
                    return true;
                }
            }
        }

        false
    }

    let mut low = 0;
    let mut high = price[price.len() - 1] - price[0];

    while low < high {
        let mid = (low + high + 1) / 2;

        if is_candidate(mid, &price, k) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low
}
