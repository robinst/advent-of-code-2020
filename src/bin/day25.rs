//! https://adventofcode.com/2020/day/23

fn main() {
    let input = include_str!("../../input/2020/day25.txt");

    let puzzle = parse(input);

    println!("One: {}", solve(&puzzle));
}

#[derive(Clone, Debug)]
struct Puzzle {
    key1: u64,
    key2: u64,
}

fn parse(input: &str) -> Puzzle {
    let mut nums = input.trim().lines().map(|l| l.parse::<u64>().unwrap());
    let key1 = nums.next().unwrap();
    let key2 = nums.next().unwrap();
    Puzzle { key1, key2 }
}

fn solve(puzzle: &Puzzle) -> u64 {
    let loop_size1 = find_loop_size(7, puzzle.key1);
    let loop_size2 = find_loop_size(7, puzzle.key2);

    transform(puzzle.key1, loop_size2)
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn find_loop_size(subject_number: u64, key: u64) -> u64 {
    let mut value = 1;
    for count in 1.. {
        value *= subject_number;
        value %= 20201227;
        if value == key {
            return count;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(find_loop_size(7, 5764801), 8);
        assert_eq!(find_loop_size(7, 17807724), 11);

        let input = "5764801\n17807724";

        let puzzle = parse(input);
        assert_eq!(solve(&puzzle), 14897079);
    }
}
