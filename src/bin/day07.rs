//! https://adventofcode.com/2020/day/7

use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/2020/day07.txt");
    let rules = parse_rules(input);
    println!("One: {}", solve(&rules, "shiny gold"));

    let rules = parse_rules2(input);
    println!("Two: {}", solve2(&rules, "shiny gold", 1) - 1);
}

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    color: String,
    count: u64,
}

fn parse_rules(rules: &str) -> HashMap<String, Vec<Bag>> {
    let mut result = HashMap::new();
    for line in rules.lines() {
        let mut parts = line.split(" bags contain ");
        let container = parts.next().unwrap().to_string();
        let inside = parts.next().unwrap();
        for bag in inside.split(", ") {
            if bag.contains("no other bags") {
                continue;
            }

            let s = bag.split(" bag").next().unwrap().to_string();
            let mut parts = s.splitn(2, " ");
            let count: u64 = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap().to_string();
            let bag = Bag {
                color: container.clone(),
                count,
            };
            result.entry(color).or_insert(Vec::new()).push(bag);
        }
    }
    result
}

fn parse_rules2(rules: &str) -> HashMap<String, Vec<Bag>> {
    let mut result = HashMap::new();
    for line in rules.lines() {
        let mut parts = line.split(" bags contain ");
        let container = parts.next().unwrap().to_string();
        let inside = parts.next().unwrap();
        for bag in inside.split(", ") {
            if bag.contains("no other bags") {
                continue;
            }

            let s = bag.split(" bag").next().unwrap().to_string();
            let mut parts = s.splitn(2, " ");
            let count: u64 = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap().to_string();
            let bag = Bag { color, count };
            result
                .entry(container.clone())
                .or_insert(Vec::new())
                .push(bag);
        }
    }
    result
}

fn solve(rules: &HashMap<String, Vec<Bag>>, start: &str) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start.to_string());

    let mut work = Vec::new();
    work.push(start.to_string());

    while let Some(s) = work.pop() {
        if let Some(containers) = rules.get(&s) {
            for container in containers {
                if !visited.contains(&container.color) {
                    visited.insert(container.color.clone());
                    work.push(container.color.clone());
                }
            }
        }
    }
    visited.len() - 1
}

fn solve2(rules: &HashMap<String, Vec<Bag>>, start: &str, num: u64) -> u64 {
    let mut count = num;
    if let Some(bags) = rules.get(start) {
        for bag in bags {
            count += num * solve2(rules, &bag.color, bag.count);
        }
    }
    count
}

/// Alternative solution using a graph library instead of constructing two different
/// `HashMap<String, Vec<Bag>>` for part 1 and 2.
fn parse_rules_graph(rules: &str) -> (HashMap<String, usize>, DiGraphMap<usize, u64>) {
    let mut names = HashMap::new();
    let mut graph = DiGraphMap::new();
    for line in rules.lines() {
        let mut parts = line.split(" bags contain ");

        let container = parts.next().unwrap().to_string();
        let len = names.len();
        let container_index = *names.entry(container).or_insert(len);

        let inside = parts.next().unwrap();
        for bag in inside.split(", ") {
            if bag.contains("no other bags") {
                continue;
            }

            let s = bag.split(" bag").next().unwrap().to_string();
            let mut parts = s.splitn(2, " ");
            let count: u64 = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap().to_string();
            let len = names.len();
            let color_index = *names.entry(color).or_insert(len);

            graph.add_node(container_index);
            graph.add_node(color_index);
            graph.add_edge(container_index, color_index, count);
        }
    }
    (names, graph)
}

fn solve_graph(rules: &DiGraphMap<usize, u64>, start: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut work = Vec::new();
    work.push(start);

    while let Some(s) = work.pop() {
        for node in rules.neighbors_directed(s, Direction::Incoming) {
            if !visited.contains(&node) {
                visited.insert(node);
                work.push(node);
            }
        }
    }
    visited.len() - 1
}

fn solve_graph2(rules: &DiGraphMap<usize, u64>, start: usize, num: u64) -> u64 {
    rules
        .neighbors(start)
        .map(|bag| {
            let bag_count = rules.edge_weight(start, bag).map_or(0, |c| *c);
            num * solve_graph2(rules, bag, bag_count)
        })
        .sum::<u64>()
        + num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let rules = parse_rules(s);
        assert_eq!(solve(&rules, "shiny gold"), 4);

        let (names, graph) = parse_rules_graph(s);
        assert_eq!(solve_graph(&graph, names["shiny gold"]), 4);
    }

    #[test]
    fn test_example_2() {
        let s = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

        let rules = parse_rules2(s);
        assert_eq!(solve2(&rules, "shiny gold", 1) - 1, 126);

        let (names, graph) = parse_rules_graph(s);
        assert_eq!(solve_graph2(&graph, names["shiny gold"], 1) - 1, 126);
    }
}
