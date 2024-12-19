use std::collections::HashMap;

fn main() {
    let g = vec![1, 2, 3];
    let s = vec![1, 1];
    assert_eq!(find_content_children(g, s), 1);

    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    assert_eq!(find_content_children(g, s), 2);

    let g = vec![1, 2, 3, 4, 5, 6];
    let s = vec![1, 3, 5];
    assert_eq!(find_content_children(g, s), 3);

    println!("All test passed!");
}
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut map_child: HashMap<i32, bool> = HashMap::new();
    let mut result = 0;

    for child in g {
        map_child.insert(child, false);
    }

    for cookie in &s {
        if let Some(&satisfied) = map_child.get(cookie) {
            if !satisfied {
                result += 1;
                map_child.insert(*cookie, true);
            }
        }
    }
    result
}
