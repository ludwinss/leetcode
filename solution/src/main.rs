fn main() {
    let height: Vec<i32> = vec![1, 2, 3, 4, 5];
    let treshold = 2;

    let solutions = stable_mountains(height, treshold);
    println!("{:?}", solutions);
}

pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut mountains = Vec::new();
    if height.len() <= 1 {
        return mountains;
    }
    for idx_mountain in 1..=height.len() - 1 {
        if height[idx_mountain - 1] > threshold {
            mountains.push(idx_mountain as i32);
        }
    }
    mountains
}
