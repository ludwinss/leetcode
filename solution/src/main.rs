fn main() {
    let input = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    let solutions = longest_common_prefix(input);
    println!("{:?}", solutions);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    if strs.len() == 1 {
        return strs[0].to_string();
    }

    let mut order_vect_prefix: Vec<String> = strs.clone();

    order_vect_prefix.sort_by_key(|chars| chars.len());

    let word = String::from(order_vect_prefix[0].clone());

    let mut idx_common = 0;

    for idx_next_char in (1..=order_vect_prefix.len() + 1).rev() {
        order_vect_prefix.iter().for_each(|string_find| {
            let word_slice: String = string_find.chars().take(idx_next_char).collect();
            if word_slice == word.chars().take(idx_next_char).collect::<String>() {
                idx_common = idx_next_char;
            } else {
                idx_common = 0;
            }
        });
        if idx_common != 0 {
            break;
        }
    }
    word.chars().take(idx_common).collect::<String>()
}
