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

    // avoid to modify the original vector
    //let mut order_vect_prefix: Vec<String> = strs.clone();

    // not sort just to get the shortest string
    //order_vect_prefix.sort_by_key(|chars| chars.len());
    let mut posibily_perfix = strs
        .iter()
        .min_by_key(|posible_short_string| posible_short_string.len())
        .unwrap()
        .as_str();

    // avoid clon vector
    //let word = String::from(order_vect_prefix[0].clone());

    // modified loop and improve conditional to break
    // for idx_next_char in (1..=order_vect_prefix.len() + 1).rev() {
    //     order_vect_prefix.iter().for_each(|string_find| {
    //         let word_slice: String = string_find.chars().take(idx_next_char).collect();
    //         if word_slice == word.chars().take(idx_next_char).collect::<String>() {
    //             idx_common = idx_next_char;
    //         } else {
    //             idx_common = 0;
    //         }
    //     });
    //     if idx_common != 0 {
    //         break;
    //     }
    // }

    for next_perfix in strs.iter() {
        while !next_perfix.starts_with(&posibily_perfix) {
            posibily_perfix = &posibily_perfix[0..posibily_perfix.len() - 1];
            if posibily_perfix.is_empty() {
                return String::from("");
            }
        }
    }

    posibily_perfix.to_string()
}
