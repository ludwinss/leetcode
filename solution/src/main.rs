fn main() {
    assert_eq!(max_diff(555), 888);

    assert_eq!(max_diff(9), 8);

    assert_eq!(max_diff(123456), 820000);

    println!("All tests passed!");
}

pub fn max_diff(num: i32) -> i32 {
    let num_str = num.to_string();

    let max_num = num_str
        .chars()
        .find(|&c| c != '9')
        .map_or(num_str.clone(), |c| num_str.replace(c, "9"));

    let first_value = num_str.chars().next().unwrap();

    let min_num = if first_value != '1' {
        num_str.replace(first_value, "1")
    } else {
        match num_str.chars().skip(1).find(|&c| c != '0' && c != '1') {
            Some(from) => num_str.replace(from, "0"),
            None => num_str.clone(),
        }
    };

    println!("{} {}", max_num, min_num);

    max_num.parse::<i32>().unwrap() - min_num.parse::<i32>().unwrap()
}
