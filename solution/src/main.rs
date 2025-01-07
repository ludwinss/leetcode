fn main() {
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");

    println!("All test passed!");
}

pub fn add_binary(a: String, b: String) -> String {
    let a_base10: u32 = u32::from_str_radix(&a, 2).unwrap_or(0);
    let b_base10: u32 = u32::from_str_radix(&b, 2).unwrap_or(0);

    format!("{:b}",  a_base10 + b_base10)
}
