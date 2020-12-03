//! https://adventofcode.com/2020/day/3

fn main() {
    let input = include_str!("../../input/2020/day03.txt");
    let lines: Vec<_> = input.lines().map(|line| line.to_string()).collect();

    let first = trees(&lines, 3, 1);
    let second = trees(&lines, 1, 1)
        * trees(&lines, 3, 1)
        * trees(&lines, 5, 1)
        * trees(&lines, 7, 1)
        * trees(&lines, 1, 2);

    println!("First: {}", first);
    println!("Second: {}", second);
}

fn trees(lines: &[String], right: usize, down: usize) -> i64 {
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;
    while y < lines.len() {
        let line = &lines[y];
        if line.as_bytes()[x % line.len()] == '#' as u8 {
            count += 1;
        }
        x += right;
        y += down;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let lines: Vec<_> = input.lines().map(|line| line.to_string()).collect();
        assert_eq!(trees(&lines, 3, 1), 7);
    }
}
