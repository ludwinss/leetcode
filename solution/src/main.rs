fn main() {
    let g = vec![1, 3, 2];
    let s = vec![1, 1];
    assert_eq!(find_content_children(g, s), 1);

    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    assert_eq!(find_content_children(g, s), 2);

    let g = vec![1, 2, 3, 4, 5, 6];
    let s = vec![1, 3, 5];
    assert_eq!(find_content_children(g, s), 3);

    let g = vec![10, 9, 8, 7];
    let s = vec![5, 6, 7, 8];
    assert_eq!(find_content_children(g, s), 2);

    println!("All test passed!");
}
pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort_unstable();
    s.sort_unstable();

    let mut idx_g = 0;
    let mut idx_s = 0;

    while idx_g < g.len() && idx_s < s.len() {
        if s[idx_s] >= g[idx_g] {
            idx_g += 1;
        }
        idx_s += 1;
    }
    idx_g as i32
}
