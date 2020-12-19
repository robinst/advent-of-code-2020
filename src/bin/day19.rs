//! https://adventofcode.com/2020/day/19

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2020/day19.txt");

    let puzzle = parse(input);
    println!("One: {}", solve(&puzzle));
    println!("Two: {}", solve2(&puzzle));
}

#[derive(Clone)]
struct Puzzle {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

#[derive(Clone, Debug)]
enum Rule {
    Char(char),
    Alt(Vec<Rule>),
    List(Vec<usize>),
}

fn parse(input: &str) -> Puzzle {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    let mut parse_messages = false;

    for line in input.lines() {
        if line == "" {
            parse_messages = true;
            continue;
        }

        if parse_messages {
            messages.push(line.to_string());
            continue;
        }

        let mut parts = line.split(": ");
        let num = parts.next().unwrap().parse::<usize>().unwrap();
        let desc = parts.next().unwrap();

        if desc.starts_with('"') {
            rules.insert(num, Rule::Char(desc.chars().nth(1).unwrap()));
        } else {
            let mut alts: Vec<_> = desc
                .split(" | ")
                .map(|alt| {
                    Rule::List(
                        alt.split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .collect();
            if alts.len() == 1 {
                rules.insert(num, alts.pop().unwrap());
            } else {
                rules.insert(num, Rule::Alt(alts));
            }
        }
    }

    Puzzle { rules, messages }
}

fn solve(puzzle: &Puzzle) -> usize {
    let regex_str = format!("^{}$", build_regex(&puzzle.rules[&0], &puzzle.rules));
    let regex = Regex::new(&regex_str).unwrap();

    puzzle.messages.iter().filter(|m| regex.is_match(m)).count()
}

fn solve2(puzzle: &Puzzle) -> usize {
    // Modified top-level rules:
    // 0: 8 11
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    //
    // -> potential matches:
    // 42 [..] 42 31
    // 42 [..] 42 42 31 31
    // 42 [..] 42 42 42 31 31 31
    // For rule 11, the number of matches for 42 has to be equal to number of matches for 31.
    // That can't be expressed in a normal regex, so expand it for a fixed number of times.
    // It means this is not a general solution (it would fail for longer inputs), but yeah.

    let a = build_regex(&puzzle.rules[&42], &puzzle.rules);
    let b = build_regex(&puzzle.rules[&31], &puzzle.rules);
    let eleven = (1..10)
        .map(|n| format!("({a}){{{n}}}({b}){{{n}}}", a = a, b = b, n = n))
        .join("|");
    let regex = Regex::new(&format!("^({a})+({eleven})$", a = a, eleven = eleven)).unwrap();
    puzzle.messages.iter().filter(|m| regex.is_match(m)).count()
}

fn build_regex(rule: &Rule, rules: &HashMap<usize, Rule>) -> String {
    match rule {
        Rule::Char(c) => c.to_string(),
        Rule::Alt(alts) => format!(
            "({})",
            alts.iter().map(|alt| build_regex(alt, rules)).join("|")
        ),
        Rule::List(indexes) => indexes
            .iter()
            .map(|&index| build_regex(&rules[&index], rules))
            .join(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let puzzle = parse(input);
        assert_eq!(solve(&puzzle), 2);
    }

    #[test]
    fn test_example_part2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        let puzzle = parse(input);
        assert_eq!(solve2(&puzzle), 12);
    }
}
