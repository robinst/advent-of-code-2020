//! https://adventofcode.com/2020/day/22

use std::collections::{HashSet, LinkedList};

fn main() {
    let input = include_str!("../../input/2020/day22.txt");

    let puzzle = parse(input);

    println!("One: {}", solve(puzzle.one.clone(), puzzle.two.clone()));
    println!("Two: {:?}", solve2(puzzle.one.clone(), puzzle.two.clone()));
}

#[derive(Clone, Debug)]
struct Puzzle {
    one: LinkedList<usize>,
    two: LinkedList<usize>,
}

fn parse(input: &str) -> Puzzle {
    let mut parts = input.split("\n\n");
    let one = parse_nums(parts.next().unwrap().strip_prefix("Player 1:\n").unwrap());
    let two = parse_nums(parts.next().unwrap().strip_prefix("Player 2:\n").unwrap());
    Puzzle { one, two }
}

fn parse_nums(input: &str) -> LinkedList<usize> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn solve(mut one: LinkedList<usize>, mut two: LinkedList<usize>) -> usize {
    while !(one.is_empty() || one.is_empty()) {
        let a = one.pop_front().unwrap();
        let b = two.pop_front().unwrap();
        if a > b {
            one.push_back(a);
            one.push_back(b);
        } else {
            two.push_back(b);
            two.push_back(a);
        }
    }

    if !one.is_empty() {
        score(&one)
    } else {
        score(&two)
    }
}

fn solve2(mut one: LinkedList<usize>, mut two: LinkedList<usize>) -> (usize, usize) {
    let mut previous = HashSet::new();

    while !(one.is_empty() || two.is_empty()) {
        let state = (one.clone(), two.clone());
        if previous.contains(&state) {
            return (0, 0);
        } else {
            previous.insert(state);
        }

        let a = one.pop_front().unwrap();
        let b = two.pop_front().unwrap();

        let winner = if one.len() >= a && two.len() >= b {
            // recurse
            let new_one = one.iter().copied().take(a).collect();
            let new_two = two.iter().copied().take(b).collect();
            let (winner, _score) = solve2(new_one, new_two);
            winner
        } else if a > b {
            0
        } else {
            1
        };

        if winner == 0 {
            one.push_back(a);
            one.push_back(b);
        } else {
            two.push_back(b);
            two.push_back(a);
        }
    }

    if !one.is_empty() {
        (0, score(&one))
    } else {
        (1, score(&two))
    }
}

fn score(cards: &LinkedList<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"#;

        let Puzzle { one, two } = parse(input);
        assert_eq!(solve(one.clone(), two.clone()), 306);
        assert_eq!(solve2(one.clone(), two.clone()).1, 291);
    }
}
