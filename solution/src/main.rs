use std::collections::HashMap;

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);

    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);

    println!("All test passed!");
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    fn stock(
        idx: usize,
        can_buy: u8,
        prices: &Vec<i32>,
        memo: &mut HashMap<(usize, u8), i32>,
    ) -> i32 {
        if idx == prices.len() {
            return 0;
        }

        if let Some(&value) = memo.get(&(idx, can_buy)) {
            return value;
        }

        let ans = if can_buy == 1 {
            let skip = stock(idx + 1, 1, prices, memo);
            let buy = stock(idx + 1, 0, prices, memo) - prices[idx];
            skip.max(buy)
        } else {
            let skip = stock(idx + 1, 0, prices, memo);
            let sell = prices[idx];
            skip.max(sell)
        };

        memo.insert((idx, can_buy), ans);

        ans
    }

    let mut memo = HashMap::new();

    stock(0, 1, &prices, &mut memo)
}
