pub fn solve_both(input: &str) -> (i32, i32) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    // Part 1
    let mut left_sorted = left_list.clone();
    let mut right_sorted = right_list.clone();
    left_sorted.sort_unstable();
    right_sorted.sort_unstable();

    let total_distance: i32 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    // Part 2
    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| {
            let count = right_list.iter().filter(|&&x| x == num).count() as i32;
            num * count
        })
        .sum();

    (total_distance, similarity_score)
}
