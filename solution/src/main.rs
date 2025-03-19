fn main() {
    let input = vec![1, 3, 2, 4, 1];
    assert_eq!(max_ice_cream(input, 7), 4);

    let input = vec![10, 6, 8, 7, 7, 8];
    assert_eq!(max_ice_cream(input, 5), 0);

    println!("All test passed!");
}

pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let mut cost_mut = costs.clone();
    cost_mut.sort();

    let (mut count, mut sum) = (0, 0);

    for &cost in cost_mut.iter() {
        if sum + cost <= coins {
            count += 1;
            sum += cost;
        } else {
            break;
        }
    }
    count
}
