use std::u8;

fn main() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();

    assert_eq!(str_str(haystack, needle), 0);

    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();

    assert_eq!(str_str(haystack, needle), -1);

    let haystack = "aabaaabaaac".to_string();
    let needle = "aabaaac".to_string();

    assert_eq!(str_str(haystack, needle), 4);

    println!("All test passed!");
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    let lsp_vector = create_lsp(needle_bytes);

    println!("{:?}",lsp_vector);

    let mut idx_needle = 0;
    let mut idx_haystack = 0;

    while idx_haystack < haystack_bytes.len() {
        if haystack_bytes[idx_haystack] == needle_bytes[idx_needle] {
            idx_needle += 1;
            idx_haystack += 1;
            if idx_needle == needle_bytes.len() {
                return (idx_haystack - idx_needle) as i32;
            }
        } else {
            if idx_needle > 0 {
                idx_needle = lsp_vector[idx_needle - 1];
            } else {
                idx_haystack += 1;
            }
        }
    }
    -1
}

pub fn create_lsp(pattern: &[u8]) -> Vec<usize> {
    let mut lsp_vector: Vec<usize> = vec![0; pattern.len()];
    let mut length = 0;
    let mut idx = 1;

    while idx < pattern.len() {
        if pattern[idx] == pattern[length] {
            length += 1;
            lsp_vector[idx] = length;
            idx += 1;
        } else if length > 0 {
            length = lsp_vector[length - 1];
        } else {
            lsp_vector[idx] = 0;
            idx += 1;
        }
    }

    lsp_vector
}
