//! https://adventofcode.com/2020/day/9

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/2020/day09.txt");
    let nums: Vec<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    let result = solve(&nums, 25);
    println!("One: {}", result);
    println!("Two: {}", solve2(&nums, result));
}

fn solve(nums: &Vec<u64>, preamble: usize) -> u64 {
    for window in nums.windows(preamble + 1) {
        let last = window[window.len() - 1];
        let before = &window[0..window.len() - 1];
        if !before
            .iter()
            .combinations(2)
            .any(|nums| nums.iter().map(|n| *n).sum::<u64>() == last)
        {
            return last;
        }
    }
    0
}

fn solve2(nums: &Vec<u64>, target: u64) -> u64 {
    for i in 0..nums.len() {
        let mut sum = 0;
        // There's probably a better algorithm, but this quadratic one was good enough.
        for j in i..nums.len() {
            sum += &nums[j];
            if sum == target {
                // Could remember smallest and largest while summing, but this was easier.
                return nums[i..j].iter().min().unwrap() + nums[i..j].iter().max().unwrap();
            } else if sum > target {
                break;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let nums = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(solve(&nums, 5), 127);
        assert_eq!(solve2(&nums, 127), 62);
    }
}
