//! https://adventofcode.com/2020/day/23

use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};
use itertools::Itertools;

fn main() {
    println!("One: {}", solve("368195742"));
    println!("Two: {}", solve2("368195742"));
}

fn solve2(input: &str) -> usize {
    let mut nums: Vec<_> = parse(input);
    let max_input = *nums.iter().max().unwrap();

    let mut num = max_input + 1;
    while nums.len() != 1_000_000 {
        nums.push(num);
        num += 1;
    }

    run2(&mut nums, 10_000_000)
}

fn run2(nums: &mut Vec<usize>, moves: i32) -> usize {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    // Our numbers start at 1, so we need one additional slot.
    let mut next = vec![0; nums.len() + 1];
    for (&c, &n) in nums.iter().tuple_windows() {
        next[c] = n;
    }
    next[nums[nums.len() - 1]] = nums[0];

    let mut current = nums[0];
    for _ in 0..moves {
        let a = next[current];
        let b = next[a];
        let c = next[b];

        let dest = find_destination(current, min, max, a, b, c);
        let after_dest = next[dest];
        let after_c = next[c];

        next[dest] = a;
        next[c] = after_dest;
        next[current] = after_c;

        current = after_c;
    }

    let a = next[1];
    let b = next[a];

    a * b
}

fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect()
}

fn solve(input: &str) -> String {
    let mut nums = parse(input);

    let result = run(&mut nums, 100);

    let result = result.iter().join("");
    let parts: Vec<_> = result.split("1").collect();
    parts.iter().rev().join("")
}

#[derive(Clone)]
struct Cup {
    link: LinkedListLink,
    value: usize,
}

intrusive_adapter!(CupAdapter = Box<Cup>: Cup { link: LinkedListLink });

fn run(nums: &Vec<usize>, moves: usize) -> Vec<usize> {
    let mut list = LinkedList::new(CupAdapter::new());
    for &num in nums {
        list.push_back(Box::new(Cup {
            link: LinkedListLink::new(),
            value: num,
        }));
    }
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    for _ in 0..moves {
        let current = list.pop_front().unwrap();
        let a = list.pop_front().unwrap();
        let b = list.pop_front().unwrap();
        let c = list.pop_front().unwrap();

        let dest = find_destination(current.value, min, max, a.value, b.value, c.value);
        let mut cursor = list.back_mut();
        loop {
            if let Some(cup) = cursor.get() {
                if cup.value == dest {
                    cursor.insert_after(c);
                    cursor.insert_after(b);
                    cursor.insert_after(a);
                    break;
                }
            }
            cursor.move_prev();
        }

        list.push_back(current);
    }

    list.iter().map(|cup| cup.value).collect()
}

fn find_destination(current: usize, min: usize, max: usize, a: usize, b: usize, c: usize) -> usize {
    let mut dest = current - 1;
    loop {
        if dest < min {
            dest = max;
        }
        if !(dest == a || dest == b || dest == c || dest == current) {
            return dest;
        }
        dest -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve("389125467"), "67384529");
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve2("389125467"), 149245887792);
    }
}
