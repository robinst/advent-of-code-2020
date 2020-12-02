//! https://adventofcode.com/2020/day/2

fn main() {
    let input = include_str!("../../input/2020/day02.txt");
    let inputs: Vec<_> = input.lines().map(|line| parse(line)).collect();

    let mut first = 0;
    let mut second = 0;

    for (c, from, to, pw) in inputs {
        if (from..=to).contains(&pw.chars().filter(|ch| ch == &c).count()) {
            first += 1;
        }

        if (pw.as_bytes()[from - 1] == c as u8) ^ (pw.as_bytes()[to - 1] == c as u8) {
            second += 1;
        }
    }
    println!("First: {}", first);
    println!("Second: {}", second);
}

// 5-6 s: zssmssbsms
fn parse(line: &str) -> (char, usize, usize, String) {
    let parts: Vec<_> = line.split_whitespace().collect();
    let nums: Vec<_> = parts[0].split("-").collect();
    let from: usize = nums[0].parse().unwrap();
    let to: usize = nums[1].parse().unwrap();
    let c = parts[1].chars().next().unwrap();
    let password = parts[2].to_string();
    (c, from, to, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("5-6 s: zssmssbsms"),
            ('s', 5, 6, "zssmssbsms".to_string())
        );
    }
}
