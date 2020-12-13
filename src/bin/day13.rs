//! https://adventofcode.com/2020/day/13

use num::integer::lcm;

fn main() {
    let input = include_str!("../../input/2020/day13.txt");
    let (departure, buses) = parse(input);
    println!("One: {}", solve(departure, buses));

    let buses = parse2(input);
    dbg!(&buses);
    println!("Two: {}", solve2(buses, 100000000000000));
}

fn parse(input: &str) -> (u64, Vec<u64>) {
    let lines: Vec<_> = input.lines().collect();
    let departure = lines[0].parse().unwrap();
    let buses: Vec<_> = lines[1]
        .split(",")
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    (departure, buses)
}

fn solve(departure: u64, buses: Vec<u64>) -> u64 {
    let (minutes, bus) = buses
        .iter()
        .map(|b| (b - (departure % b), b))
        .min()
        .unwrap();
    minutes * bus
}

#[derive(Debug)]
struct Bus {
    minutes: u64,
    offset: u64,
}

fn parse2(input: &str) -> Vec<Bus> {
    let lines: Vec<_> = input.lines().collect();
    let mut buses = Vec::new();
    for (i, s) in lines[1].split(",").enumerate() {
        if s != "x" {
            let minutes = s.parse::<u64>().unwrap();
            buses.push(Bus {
                minutes,
                offset: i as u64,
            });
        }
    }
    buses
}

/// Brute force solution that takes a couple of minutes on the actual input. Didn't know about
/// Chinese remainder, heh.
fn solve2(buses: Vec<Bus>, start: u64) -> u64 {
    // The magic bus is the one with an offset the same as the first bus's minutes. In the example,
    // that's bus 19 with offset 7, which matches bus 7.
    let magic = buses.iter().find(|b| b.offset == buses[0].minutes).unwrap();
    // Instead of incrementing one by one, we can jump by the LCM now.
    let incr = lcm(magic.minutes, magic.offset);

    let mut t = start + (incr - (start % incr)) - magic.offset;
    dbg!(t);
    dbg!(incr);
    loop {
        // Don't need to check the first bus, because we're already jumping directly to a departure.
        if buses[1..].iter().all(|b| (t + b.offset) % b.minutes == 0) {
            return t;
        }
        t += incr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "939
7,13,x,x,59,x,31,19";

        let (departure, buses) = parse(input);
        assert_eq!(solve(departure, buses), 295);
        let buses = parse2(input);
        assert_eq!(solve2(buses, 1), 1068781);
    }
}
