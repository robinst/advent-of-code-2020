//! https://adventofcode.com/2020/day/14

use reformation::Reformation;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2020/day14.txt");
    let instructions = parse(input);

    println!("One: {}", solve(&instructions));
    println!("Two: {}", solve2(&instructions));
}

#[derive(Reformation, Clone, Debug)]
enum Inst {
    #[reformation("mask = {}")]
    Mask(String),
    #[reformation("mem\\[{}\\] = {}")]
    Mem(u64, u64),
}

fn parse(input: &str) -> Vec<Inst> {
    input.lines().map(|l| Inst::parse(l).unwrap()).collect()
}

fn solve(instructions: &[Inst]) -> u64 {
    let mut on_mask = 0;
    let mut off_mask = 0;
    let mut memory = HashMap::new();
    for inst in instructions {
        match inst {
            Inst::Mask(s) => {
                on_mask = u64::from_str_radix(&s.replace("X", "0"), 2).unwrap();
                off_mask = u64::from_str_radix(&s.replace("X", "1"), 2).unwrap();
            }
            Inst::Mem(target, mut value) => {
                value = value | on_mask;
                value = value & off_mask;
                memory.insert(target, value);
            }
        }
    }
    memory.values().sum()
}

fn solve2(instructions: &[Inst]) -> u64 {
    let mut on_mask = 0;
    let mut floating = Vec::new();
    let mut memory = HashMap::new();
    for inst in instructions {
        match inst {
            Inst::Mask(s) => {
                on_mask = u64::from_str_radix(&s.replace("X", "0"), 2).unwrap();
                floating = s
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|(_i, c)| *c == 'X')
                    .map(|(i, _c)| i)
                    .collect();
            }
            Inst::Mem(mut target, value) => {
                target = target | on_mask;
                set_floating(&mut memory, target, *value, &floating);
            }
        }
    }
    memory.values().sum()
}

fn set_floating(
    mut memory: &mut HashMap<u64, u64>,
    mut target: u64,
    value: u64,
    floating: &[usize],
) {
    if !floating.is_empty() {
        let index = floating[0];
        let mask = 1 << index;

        target = target & !mask;
        memory.insert(target, value);
        set_floating(&mut memory, target, value, &floating[1..]);

        target = target | mask;
        memory.insert(target, value);
        set_floating(&mut memory, target, value, &floating[1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let instructions = parse(input);
        assert_eq!(solve(&instructions), 165);
    }

    #[test]
    fn test_examples_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let instructions = parse(input);
        assert_eq!(solve2(&instructions), 208);
    }
}
