fn main() {
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
    assert_eq!(add_binary("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(), "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()),"110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000");
    assert_eq!(add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()),"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    println!("All test passed!");
}

pub fn add_binary(a: String, b: String) -> String {
    let mut carry = 0;
    let mut idx_a = a.len();
    let mut idx_b = b.len();
    let mut results = String::new();

    while idx_b > 0 || idx_a > 0 || carry > 0 {
        let digit_a = if idx_a > 0 {
            idx_a -= 1;
            a.chars().nth(idx_a).unwrap_or('0').to_digit(2).unwrap_or(0)
        } else {
            0
        };

        let digit_b = if idx_b > 0 {
            idx_b -= 1;
            b.chars().nth(idx_b).unwrap_or('0').to_digit(2).unwrap_or(0)
        } else {
            0
        };

        let sum = digit_b + digit_a + carry;

        carry = sum / 2;

        results.push_str(&(sum % 2).to_string());
    }

    results.chars().rev().collect::<String>()
}
