//! https://adventofcode.com/2020/day/5

fn main() {
    let input = include_str!("../../input/2020/day05.txt");
    let strings: Vec<_> = input.lines().map(|l| l.to_string()).collect();

    let mut ids: Vec<_> = strings.iter().map(|s| seat(s)).collect();
    if let Some(max) = ids.iter().max() {
        println!("Max: {}", max);
    }

    ids.sort();
    for i in 0..ids.len() - 1 {
        if ids[i] + 1 != ids[i + 1] {
            println!("Missing: {}", ids[i] + 1);
        }
    }
}

/// FBFBBFFRLR -> 357
fn seat(s: &str) -> u32 {
    let mut low = 0;
    let mut high = 127;

    for c in s[0..7].chars() {
        match c {
            'F' => {
                high = low + ((high - low) / 2);
            }
            'B' => {
                low = low + ((high - low) / 2) + 1;
            }
            _ => panic!("Unknown row code {}", c),
        }
    }
    let row = low;

    let mut low = 0;
    let mut high = 7;
    for c in s[7..].chars() {
        match c {
            'L' => {
                high = low + ((high - low) / 2);
            }
            'R' => {
                low = low + ((high - low) / 2) + 1;
            }
            _ => panic!("Unknown column code {}", c),
        }
    }
    let column = low;

    row * 8 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(seat("FBFBBFFRLR"), 357);
        assert_eq!(seat("BFFFBBFRRR"), 567);
        assert_eq!(seat("FFFBBBFRRR"), 119);
        assert_eq!(seat("BBFFBBFRLL"), 820);
    }
}
