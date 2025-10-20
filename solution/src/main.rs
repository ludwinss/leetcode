fn main() {
    assert_eq!(find_closest(2, 7, 4), 1);

    assert_eq!(find_closest(1, 5, 3), 0);

    println!("All tests passed!");
}

pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    match (x - z).abs().cmp(&(y - z).abs()) {
        std::cmp::Ordering::Equal => {
            return 0;
        }
        std::cmp::Ordering::Greater => {
            return 2;
        }
        std::cmp::Ordering::Less => {
            return 1;
        }
    }
}
