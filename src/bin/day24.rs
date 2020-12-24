//! https://adventofcode.com/2020/day/24

use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/2020/day24.txt");

    let puzzle = parse(input);

    println!("One: {}", solve(&puzzle));
    println!("Two: {}", solve2(&puzzle));
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

struct Instruction {
    directions: Vec<Direction>,
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse(input: &str) -> Puzzle {
    let instructions: Vec<_> = input.lines().map(|line| parse_line(line)).collect();
    Puzzle { instructions }
}

fn parse_line(line: &str) -> Instruction {
    let mut s = line;
    let mut directions = Vec::new();
    while !s.is_empty() {
        if s.starts_with("e") {
            directions.push(Direction::East);
            s = &s[1..];
        } else if s.starts_with("w") {
            directions.push(Direction::West);
            s = &s[1..];
        } else if s.starts_with("se") {
            directions.push(Direction::SouthEast);
            s = &s[2..];
        } else if s.starts_with("sw") {
            directions.push(Direction::SouthWest);
            s = &s[2..];
        } else if s.starts_with("ne") {
            directions.push(Direction::NorthEast);
            s = &s[2..];
        } else if s.starts_with("nw") {
            directions.push(Direction::NorthWest);
            s = &s[2..];
        } else {
            panic!("Unknown direction: {}", s);
        }
    }
    Instruction { directions }
}

fn solve(puzzle: &Puzzle) -> usize {
    flip(puzzle).len()
}

fn solve2(puzzle: &Puzzle) -> usize {
    let mut floor = flip(puzzle);

    for _ in 0..100 {
        let mut new_floor = HashSet::new();
        let mut white_to_check = HashSet::new();
        for black_tile in &floor {
            let (blacks, whites): (Vec<_>, Vec<_>) = neighbors(&black_tile)
                .iter()
                .partition(|t| floor.contains(t));
            if !(blacks.len() == 0 || blacks.len() > 2) {
                new_floor.insert(black_tile.clone());
            }

            for white in whites {
                white_to_check.insert(white);
            }
        }

        for white_tile in white_to_check {
            if neighbors(&white_tile)
                .iter()
                .filter(|t| floor.contains(t))
                .count()
                == 2
            {
                new_floor.insert(white_tile);
            }
        }

        floor = new_floor;
    }

    floor.len()
}

fn neighbors(&(x, y): &(i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (x - 2, y),
        (x + 2, y),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ]
}

fn flip(puzzle: &Puzzle) -> HashSet<(i64, i64)> {
    let mut flipped = HashSet::new();
    for instruction in &puzzle.instructions {
        let mut x = 0i64;
        let mut y = 0i64;
        for direction in &instruction.directions {
            match direction {
                Direction::East => x += 2,
                Direction::SouthEast => {
                    x += 1;
                    y += 1;
                }
                Direction::SouthWest => {
                    x -= 1;
                    y += 1;
                }
                Direction::West => {
                    x -= 2;
                }
                Direction::NorthWest => {
                    x -= 1;
                    y -= 1;
                }
                Direction::NorthEast => {
                    x += 1;
                    y -= 1;
                }
            }
        }
        if !flipped.insert((x, y)) {
            flipped.remove(&(x, y));
        }
    }
    flipped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

        let puzzle = parse(input);
        assert_eq!(solve(&puzzle), 10);
        assert_eq!(solve2(&puzzle), 2208);
    }
}
