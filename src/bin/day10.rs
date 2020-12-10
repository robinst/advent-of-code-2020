//! https://adventofcode.com/2020/day/10

use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2020/day10.txt");
    let nums: Vec<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    println!("One: {}", solve(&nums));
    println!("Two: {}", solve2(&nums));
}

fn solve(nums: &Vec<u64>) -> u64 {
    let mut nums = nums.clone();
    nums.sort();
    nums.insert(0, 0);
    let mut diffs = HashMap::new();
    diffs.insert(3, 1);
    for window in nums.windows(2) {
        let diff = window[1] - window[0];
        *diffs.entry(diff).or_insert(0) += 1;
    }
    diffs[&1] * diffs[&3]
}

fn solve2(nums: &Vec<u64>) -> u64 {
    let mut nums = nums.clone();
    nums.sort();
    nums.insert(0, 0);

    let mut memo = HashMap::new();
    arrangements(&nums, &mut memo)
}

fn arrangements(nums: &[u64], memo: &mut HashMap<usize, u64>) -> u64 {
    if nums.len() == 1 {
        return 1;
    }

    if let Some(result) = memo.get(&nums.len()) {
        return *result;
    }

    let mut count = 0;
    let first = &nums[0];
    for i in 1..nums.len() {
        if &nums[i] - first <= 3 {
            count += arrangements(&nums[i..], memo);
        } else {
            break;
        }
    }

    memo.insert(nums.len(), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let first = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let second = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(solve(&second), 220);

        assert_eq!(solve2(&first), 8);
        assert_eq!(solve2(&second), 19208);
    }
}
