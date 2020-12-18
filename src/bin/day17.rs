//! https://adventofcode.com/2020/day/17

use reformation::Reformation;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/2020/day17.txt");
    let puzzle = parse(input);

    println!("One: {}", solve(&puzzle));
    println!("Two: {}", solve2(&puzzle));
}

struct Puzzle {
    map: HashMap<(i64, i64, i64), State>,
    map4: HashMap<(i64, i64, i64, i64), State>,
}

#[derive(Reformation, Clone, Debug)]
enum State {
    #[reformation("#")]
    Active,
    #[reformation(".")]
    Inactive,
}

fn parse(input: &str) -> Puzzle {
    let mut map = HashMap::new();
    let mut map4 = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            map.insert(
                (x as i64, y as i64, 0i64),
                State::parse(&c.to_string()).unwrap(),
            );
            map4.insert(
                (x as i64, y as i64, 0i64, 0i64),
                State::parse(&c.to_string()).unwrap(),
            );
        }
    }
    Puzzle { map, map4 }
}

fn solve(puzzle: &Puzzle) -> usize {
    let mut map = puzzle.map.clone();

    for _cycle in 0..6 {
        let mut new_map = HashMap::new();
        let mut also_check = Vec::new();
        for (pos, state) in &map {
            match state {
                State::Active => {
                    let active = active_neighbors(pos, &map);
                    if active == 2 || active == 3 {
                        new_map.insert(pos.clone(), State::Active);
                    }

                    for neighbor in neighbor_positions(pos) {
                        if !map.contains_key(&neighbor) {
                            also_check.push(neighbor);
                        }
                    }
                }
                State::Inactive => {
                    if active_neighbors(pos, &map) == 3 {
                        new_map.insert(pos.clone(), State::Active);
                    }
                }
            }
        }

        for pos in also_check {
            if active_neighbors(&pos, &map) == 3 {
                new_map.insert(pos, State::Active);
            }
        }

        map = new_map;
    }

    map.values().filter(|s| matches!(s, State::Active)).count()
}

fn neighbor_positions((x, y, z): &(i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut neighbors = Vec::new();
    for xd in vec![-1, 0, 1] {
        for yd in vec![-1, 0, 1] {
            for zd in vec![-1, 0, 1] {
                if xd == 0 && yd == 0 && zd == 0 {
                    continue;
                }
                neighbors.push((x + xd, y + yd, z + zd));
            }
        }
    }
    neighbors
}

fn active_neighbors(pos: &(i64, i64, i64), map: &HashMap<(i64, i64, i64), State>) -> usize {
    neighbor_positions(pos)
        .iter()
        .filter(|p| map.get(p).map_or(false, |s| matches!(s, State::Active)))
        .count()
}

fn solve2(puzzle: &Puzzle) -> usize {
    let diffs = calculate_diffs();

    let mut map = puzzle.map4.clone();

    for _cycle in 0..6 {
        let mut new_map = HashMap::new();
        let mut inactive_neighbors = HashSet::new();
        for (pos, state) in &map {
            match state {
                State::Active => {
                    let active = active_neighbors4(pos, &map, &diffs);
                    if active == 2 || active == 3 {
                        new_map.insert(pos.clone(), State::Active);
                    }

                    for diff in &diffs {
                        let p = (
                            pos.0 + diff.0,
                            pos.1 + diff.1,
                            pos.2 + diff.2,
                            pos.3 + diff.3,
                        );
                        if !map.contains_key(&p) {
                            inactive_neighbors.insert(p);
                        }
                    }
                }
                State::Inactive => {
                    if active_neighbors4(pos, &map, &diffs) == 3 {
                        new_map.insert(pos.clone(), State::Active);
                    }
                }
            }
        }

        for pos in inactive_neighbors {
            if active_neighbors4(&pos, &map, &diffs) == 3 {
                new_map.insert(pos, State::Active);
            }
        }

        map = new_map;
    }

    map.values().filter(|s| matches!(s, State::Active)).count()
}

fn calculate_diffs() -> Vec<(i64, i64, i64, i64)> {
    let mut diffs = Vec::new();
    for xd in vec![-1, 0, 1] {
        for yd in vec![-1, 0, 1] {
            for zd in vec![-1, 0, 1] {
                for ad in vec![-1, 0, 1] {
                    if xd == 0 && yd == 0 && zd == 0 && ad == 0 {
                        continue;
                    }
                    diffs.push((xd, yd, zd, ad));
                }
            }
        }
    }
    diffs
}

fn active_neighbors4(
    pos: &(i64, i64, i64, i64),
    map: &HashMap<(i64, i64, i64, i64), State>,
    diffs: &Vec<(i64, i64, i64, i64)>,
) -> usize {
    diffs
        .iter()
        .map(|d| (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2, pos.3 + d.3))
        .filter(|p| map.get(&p).map_or(false, |s| matches!(s, State::Active)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = ".#.
..#
###
";
        let puzzle = parse(input);
        assert_eq!(solve(&puzzle), 112);
        assert_eq!(solve2(&puzzle), 848);
    }
}
