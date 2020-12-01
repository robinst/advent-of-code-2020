//! https://adventofcode.com/2020/day/1

use itertools::enumerate;

fn main() {
    let input = include_str!("../../input/2020/day01.txt");
    let numbers: Vec<u64> = input
        .lines()
        .map(|line| line.parse().expect(&format!("{:?}", line)))
        .collect();
    println!("{}", solve(&numbers));
    println!("{}", solve2(&numbers));
}

fn solve(numbers: &[u64]) -> u64 {
    for (i, a) in enumerate(numbers) {
        for b in &numbers[i + 1..] {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    return 0;
}

fn solve2(numbers: &[u64]) -> u64 {
    for (i, a) in enumerate(numbers) {
        for (j, b) in enumerate(&numbers[i + 1..]) {
            for c in &numbers[i + j + 1..] {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve(&vec![1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(&vec![1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
