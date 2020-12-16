//! https://adventofcode.com/2020/day/16

use reformation::Reformation;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/2020/day16.txt");
    let puzzle = parse(input);

    println!("One: {}", solve(&puzzle));
    println!("Two: {}", solve2(&puzzle));
}

struct Puzzle {
    conditions: Vec<Condition>,
    your: Vec<u64>,
    nearby: Vec<Vec<u64>>,
}

struct Condition {
    name: String,
    ranges: Vec<FromTo>,
}

#[derive(Reformation, Debug)]
#[reformation(r"{from}-{to}")]
struct FromTo {
    from: u64,
    to: u64,
}

enum State {
    Cond,
    Your,
    Nearby,
}

fn parse(input: &str) -> Puzzle {
    let mut conditions = Vec::new();
    let mut tickets = Vec::new();
    let mut your = Vec::new();
    let mut state = State::Cond;
    for line in input.lines() {
        if line == "your ticket:" {
            state = State::Your;
        } else if line == "nearby tickets:" {
            state = State::Nearby;
        } else if !line.is_empty() {
            match state {
                State::Cond => {
                    let mut parts = line.split(": ");
                    let name = parts.next().unwrap().to_string();
                    let ranges = parts
                        .next()
                        .unwrap()
                        .split(" or ")
                        .map(|r| FromTo::parse(r).unwrap())
                        .collect();
                    conditions.push(Condition { name, ranges });
                }
                State::Your => {
                    your = line.split(",").map(|v| v.parse::<u64>().unwrap()).collect();
                }
                State::Nearby => {
                    tickets.push(line.split(",").map(|v| v.parse::<u64>().unwrap()).collect())
                }
            }
        }
    }
    Puzzle {
        conditions,
        your,
        nearby: tickets,
    }
}

fn solve(puzzle: &Puzzle) -> u64 {
    let mut valid = HashSet::new();

    for condition in &puzzle.conditions {
        for range in &condition.ranges {
            for i in range.from..=range.to {
                valid.insert(i);
            }
        }
    }

    let mut error = 0;
    for ticket in &puzzle.nearby {
        for value in ticket {
            if !valid.contains(&value) {
                error += value;
            }
        }
    }
    error
}

fn solve2(puzzle: &Puzzle) -> u64 {
    let mut value_to_conditions = HashMap::new();

    for condition in &puzzle.conditions {
        for range in &condition.ranges {
            for i in range.from..=range.to {
                value_to_conditions
                    .entry(i)
                    .or_insert(HashSet::new())
                    .insert(condition.name.clone());
            }
        }
    }

    let mut valid = Vec::new();
    for ticket in &puzzle.nearby {
        if ticket
            .iter()
            .all(|value| value_to_conditions.contains_key(&value))
        {
            valid.push(ticket.clone());
        }
    }

    let mut departure_values = Vec::new();
    let mut name_to_col = HashMap::new();

    while name_to_col.len() != puzzle.conditions.len() {
        for col in 0..puzzle.conditions.len() {
            let mut possible: HashSet<String> = HashSet::new();

            for (i, ticket) in valid.iter().enumerate() {
                if let Some(conditions) = value_to_conditions.get(&ticket[col]) {
                    if i == 0 {
                        for cond in conditions {
                            possible.insert(cond.clone());
                        }
                    } else {
                        possible.retain(|v| conditions.contains(v));
                    }
                }
            }

            if possible.len() == 1 {
                let col_name = possible.iter().next().unwrap();
                println!("Found column: {} for index {}", col_name, col);
                name_to_col.insert(col_name.clone(), col);

                for (_k, v) in &mut value_to_conditions {
                    v.remove(col_name);
                }

                if col_name.starts_with("departure") {
                    departure_values.push(puzzle.your[col]);
                }
            }
        }
    }

    departure_values.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let puzzle = parse(input);
        assert_eq!(solve(&puzzle), 71);
    }
}
