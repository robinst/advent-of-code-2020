//! https://adventofcode.com/2020/day/12

use reformation::Reformation;

fn main() {
    let input = include_str!("../../input/2020/day12.txt");
    let actions = parse(input);

    println!("One: {}", solve(&actions));
    println!("Two: {}", solve2(&actions));
}

#[derive(Reformation, Clone, Debug)]
enum Action {
    #[reformation("N{}")]
    N(i64),
    #[reformation("S{}")]
    S(i64),
    #[reformation("E{}")]
    E(i64),
    #[reformation("W{}")]
    W(i64),
    #[reformation("L{}")]
    L(i64),
    #[reformation("R{}")]
    R(i64),
    #[reformation("F{}")]
    F(i64),
}

fn parse(input: &str) -> Vec<Action> {
    input.lines().map(|l| Action::parse(l).unwrap()).collect()
}

fn solve(actions: &Vec<Action>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 90;
    for action in actions {
        match action {
            Action::N(n) => {
                y += n;
            }
            Action::S(n) => {
                y -= n;
            }
            Action::E(n) => {
                x += n;
            }
            Action::W(n) => {
                x -= n;
            }
            Action::L(deg) => {
                direction = (direction - deg + 360) % 360;
            }
            Action::R(deg) => {
                direction = (direction + deg) % 360;
            }
            Action::F(n) => match direction {
                0 => y += n,
                90 => x += n,
                180 => y -= n,
                270 => x -= n,
                _ => panic!("Unknown direction {}", direction),
            },
        }
    }
    return x.abs() + y.abs();
}

fn solve2(actions: &Vec<Action>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut way_x = 10;
    let mut way_y = 1;

    for action in actions {
        match action {
            Action::N(n) => {
                way_y += n;
            }
            Action::S(n) => {
                way_y -= n;
            }
            Action::E(n) => {
                way_x += n;
            }
            Action::W(n) => {
                way_x -= n;
            }
            Action::L(deg) => {
                for _ in 0..(deg / 90) {
                    let new_x = -way_y;
                    way_y = way_x;
                    way_x = new_x;
                }
            }
            Action::R(deg) => {
                for _ in 0..(deg / 90) {
                    let new_y = -way_x;
                    way_x = way_y;
                    way_y = new_y;
                }
            }
            Action::F(n) => {
                x += way_x * n;
                y += way_y * n;
            }
        }
    }
    return x.abs() + y.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "F10
N3
F7
R90
F11
";

        let actions = parse(input);
        assert_eq!(solve(&actions), 25);
        assert_eq!(solve2(&actions), 286);
    }
}
