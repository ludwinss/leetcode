fn main() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();

    assert_eq!(str_str(haystack, needle), 0);

    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();

    assert_eq!(str_str(haystack, needle), -1);

    println!("All test passed!");
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let max_len_haystack: usize = haystack.chars().map(|cha| cha.len_utf8()).sum();
    let max_len_needle: usize = needle.chars().map(|cha| cha.len_utf8()).sum();

    for idx in 0..max_len_haystack {
        if idx + max_len_needle > max_len_haystack {
            break;
        }

        if &haystack[idx..max_len_needle + idx] == needle {
            return idx as i32;
        }
    }
    -1
}
