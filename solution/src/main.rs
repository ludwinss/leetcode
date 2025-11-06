fn main() {
    assert_eq!(
        hardest_worker(10, vec![vec![0, 2], vec![2, 5], vec![0, 9], vec![1, 15]]),
        1
    );
    assert_eq!(
        hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
        3
    );
    assert_eq!(hardest_worker(2, vec![vec![0, 10], vec![1, 10]]), 0);

    assert_eq!(
        hardest_worker(
            70,
            vec![
                vec![36, 3],
                vec![1, 5],
                vec![12, 8],
                vec![25, 9],
                vec![53, 11],
                vec![29, 12],
                vec![52, 14]
            ]
        ),
        12
    );
    println!("All tests passed!");
}

pub fn hardest_worker(_: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut max_time = i32::MIN;
    let mut prev_time = 0;
    let mut result_id = logs[0][0];

    for log in logs {
        let [id, time] = log.try_into().unwrap();
        let duration = time - prev_time;

        if duration > max_time || (duration == max_time && id < result_id) {
            max_time = duration;
            result_id = id;
        }
        prev_time = time;
    }

    result_id
}
