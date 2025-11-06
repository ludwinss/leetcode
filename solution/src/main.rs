fn main() {
    assert_eq!(champagne_tower(1, 1, 1), 0.00000);
    assert_eq!(champagne_tower(2, 1, 1), 0.50000);
    assert_eq!(champagne_tower(100000009, 33, 17), 1.00000);
    println!("All tests passed!");
}
pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    use std::collections::HashMap;

    fn calc(row: i32, col: i32, memo: &mut HashMap<(i32, i32), f64>, poured: f64) -> f64 {
        if col < 0 || col > row {
            return 0.0;
        }

        if row == 0 && col == 0 {
            return poured;
        }

        if let Some(&value) = memo.get(&(row, col)) {
            return value;
        }

        let left = ((calc(row - 1, col - 1, memo, poured) - 1.0) / 2.0).max(0.0);
        let right = ((calc(row - 1, col, memo, poured) - 1.0) / 2.0).max(0.0);

        let result = left + right;
        memo.insert((row, col), result);
        result
    }

    let mut memo = HashMap::new();
    let value = calc(query_row, query_glass, &mut memo, poured as f64);
    value.min(1.0)
}
