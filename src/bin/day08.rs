//! https://adventofcode.com/2020/day/8

use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/2020/day08.txt");
    let prog: Vec<_> = input.lines().map(|l| parse(l)).collect();
    println!("One: {}", solve(&prog));
    println!("Two: {}", solve2(prog));
}

fn solve(prog: &Vec<Ins>) -> i64 {
    let mut executed = HashSet::new();
    let mut pos = 0i64;
    let mut result = 0i64;
    loop {
        if executed.contains(&pos) {
            break;
        }
        executed.insert(pos);
        match prog[pos as usize] {
            Ins::Nop(..) => {
                pos += 1;
            }
            Ins::Acc(num) => {
                result += num;
                pos += 1;
            }

            Ins::Jmp(num) => pos += num,
        }
    }
    result
}

fn solve2(mut prog: Vec<Ins>) -> i64 {
    for i in 0..prog.len() {
        match prog[i] {
            Ins::Nop(num) => prog[i] = Ins::Jmp(num),
            Ins::Jmp(num) => prog[i] = Ins::Nop(num),
            _ => {
                // Nothing changed, no need to try
                continue;
            }
        }

        if let Some(result) = run(&prog) {
            return result;
        }

        match prog[i] {
            Ins::Nop(num) => prog[i] = Ins::Jmp(num),
            Ins::Jmp(num) => prog[i] = Ins::Nop(num),
            _ => {}
        }
    }
    unreachable!("Should be possible");
}

fn run(prog: &Vec<Ins>) -> Option<i64> {
    let mut executed = HashSet::new();
    let mut pos = 0i64;
    let mut result = 0i64;
    loop {
        if executed.contains(&pos) {
            return None;
        }
        executed.insert(pos);
        match prog[pos as usize] {
            Ins::Nop(..) => {
                pos += 1;
            }
            Ins::Acc(num) => {
                result += num;
                pos += 1;
            }

            Ins::Jmp(num) => pos += num,
        }
        if pos == prog.len() as i64 {
            return Some(result);
        }
    }
}

enum Ins {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

fn parse(line: &str) -> Ins {
    let mut parts = line.split(" ");
    let ins = parts.next().unwrap();
    let num: i64 = parts.next().unwrap().parse().unwrap();
    match ins {
        "nop" => Ins::Nop(num),
        "acc" => Ins::Acc(num),
        "jmp" => Ins::Jmp(num),
        _ => panic!("Unknown op: {}", line),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let prog: Vec<_> = input.lines().map(|l| parse(l)).collect();
        assert_eq!(solve(&prog), 5);
        assert_eq!(solve2(prog), 8);
    }
}
