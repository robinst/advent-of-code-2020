//! https://adventofcode.com/2020/day/15

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2020/day15.txt");
    let numbers: Vec<_> = input
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("One: {}", solve(&numbers));
    println!("Two: {}", solve2(&numbers, 30000000));
}

fn solve(numbers: &[usize]) -> usize {
    let mut history: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut last = 0;
    for i in 0..2020 {
        let number = if let Some(starting) = numbers.get(i) {
            *starting
        } else {
            if let Some(indexes) = history.get(&last) {
                if indexes.len() == 1 {
                    0
                } else {
                    indexes[indexes.len() - 1] - indexes[indexes.len() - 2]
                }
            } else {
                unreachable!("Previous number expected in history");
            }
        };

        history.entry(number).or_insert(Vec::new()).push(i);
        last = number;
    }
    last
}

fn solve2(numbers: &[usize], end: usize) -> usize {
    let mut history: HashMap<usize, usize> = HashMap::new();

    let mut last = 0;
    for i in 0..end {
        let number = if let Some(starting) = numbers.get(i) {
            *starting
        } else {
            if let Some(previous) = history.get(&last) {
                i - 1 - previous
            } else {
                0
            }
        };

        if i != 0 {
            history.insert(last, i - 1);
        }
        last = number;
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(solve(&[0, 3, 6]), 436);
        assert_eq!(solve(&[1, 3, 2]), 1);
        assert_eq!(solve(&[2, 1, 3]), 10);
        assert_eq!(solve(&[2, 3, 1]), 78);
        assert_eq!(solve(&[3, 2, 1]), 438);
        assert_eq!(solve(&[3, 1, 2]), 1836);

        assert_eq!(solve2(&[0, 3, 6], 30000000), 175594);
        assert_eq!(solve2(&[1, 3, 2], 30000000), 2578);
        assert_eq!(solve2(&[2, 1, 3], 30000000), 3544142);
        assert_eq!(solve2(&[1, 2, 3], 30000000), 261214);
        assert_eq!(solve2(&[2, 3, 1], 30000000), 6895259);
        assert_eq!(solve2(&[3, 2, 1], 30000000), 18);
        assert_eq!(solve2(&[3, 1, 2], 30000000), 362);
    }
}
