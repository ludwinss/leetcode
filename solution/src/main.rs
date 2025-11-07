fn main() {
    assert_eq!(digit_sum("11111222223".to_string(), 3), "135");
    assert_eq!(digit_sum("00000000".to_string(), 3), "000");
    println!("All tests passed!");
}

pub fn digit_sum(mut s: String, k: i32) -> String {
    let k: usize = k as usize;

    while s.len() > k {
        s = s
            .as_bytes()
            .chunks(k)
            .map(|chunks| {
                chunks
                    .iter()
                    .map(|b| (b - b'0') as u32)
                    .sum::<u32>()
                    .to_string()
            })
            .collect();
    }

    s
}
