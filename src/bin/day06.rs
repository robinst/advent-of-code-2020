//! https://adventofcode.com/2020/day/6

use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/2020/day06.txt");
    println!("One: {}", solve(input));
    println!("Two: {}", solve2(input));
}

fn solve(s: &str) -> usize {
    let groups = s.split("\n\n");
    let mut count = 0;
    for group in groups {
        let mut answers = HashSet::new();
        for person in group.lines() {
            for answer in person.chars() {
                answers.insert(answer);
            }
        }
        count += answers.len();
    }
    count
}

fn solve2(s: &str) -> usize {
    let groups = s.split("\n\n");
    let mut count = 0;
    for group in groups {
        let mut answers = HashSet::new();
        for (i, person) in group.lines().enumerate() {
            let person_answers: HashSet<_> = person.chars().collect();
            if i != 0 {
                answers = answers.intersection(&person_answers).map(|c| *c).collect();
            } else {
                answers = person_answers;
            }
        }
        count += answers.len();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let s = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve(s), 11);
        assert_eq!(solve2(s), 6);
    }
}
