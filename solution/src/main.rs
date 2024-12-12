fn main() {
    let input = 123;
    assert_eq!(reverse(input), 321);

    let input = -123;

    assert_eq!(reverse(input), -321);

    println!("All test passed!");
}

pub fn reverse(x: i32) -> i32 {
    let mut x_abs = x.abs();
    let mut vec_bytes = [0_u8; 10];

    let mut idx = 0;

    while x_abs > 0 {
        vec_bytes[idx] = (x_abs % 10) as u8;
        x_abs /= 10;
        idx += 1;
    }

    for j in 0..idx {
        if let Some(k) = x_abs.checked_mul(10) {
            if let Some(l) = k.checked_add(vec_bytes[j] as i32) {
                x_abs = l;
                continue;
            }
        };
        return 0;
    }

    x_abs * if x < 0 { -1 } else { 1 }
}
