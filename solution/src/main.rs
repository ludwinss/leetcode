fn main() {
    let input = vec![1, 3, 2, 4, 1];
    assert_eq!(max_ice_cream(input, 7), 4);

    let input = vec![10, 6, 8, 7, 7, 8];
    assert_eq!(max_ice_cream(input, 5), 0);

    println!("All test passed!");
}

pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
    costs.sort_unstable();

    costs
        .iter()
        .scan(coins, |remaining_coins, &next_value| {
            (*remaining_coins >= next_value).then(|| {
                *remaining_coins -= next_value;
                1
            })
        })
        .sum()
}
