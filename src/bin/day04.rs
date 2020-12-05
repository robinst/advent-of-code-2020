//! https://adventofcode.com/2020/day/4

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/2020/day04.txt");
    let strings = input.split("\n\n");

    let mut passports = Vec::new();
    for s in strings {
        let fields = s.split_whitespace();
        let mut values = HashMap::new();
        for field in fields {
            let mut parts = field.split(":");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            values.insert(key, value);
        }

        passports.push(values);
    }

    let valids: Vec<_> = passports
        .iter()
        .filter(|p| {
            vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|key| p.contains_key(*key))
        })
        .collect();
    println!("First: {}", valids.len());

    let second = valids.iter().filter(|p| is_valid(p)).count();
    println!("Second: {}", second);
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn is_valid(passport: &HashMap<String, String>) -> bool {
    let year = passport["byr"].parse::<u32>().unwrap_or(0);
    if year < 1920 || year > 2002 {
        return false;
    }
    let year = passport["iyr"].parse::<u32>().unwrap_or(0);
    if year < 2010 || year > 2020 {
        return false;
    }
    let year = passport["eyr"].parse::<u32>().unwrap_or(0);
    if year < 2020 || year > 2030 {
        return false;
    }
    let hgt = &passport["hgt"];
    if hgt.ends_with("cm") {
        let cm = hgt.strip_suffix("cm").unwrap().parse::<u32>().unwrap_or(0);
        if cm < 150 || cm > 193 {
            return false;
        }
    } else if hgt.ends_with("in") {
        let inches = hgt.strip_suffix("in").unwrap().parse::<u32>().unwrap_or(0);
        if inches < 59 || inches > 76 {
            return false;
        }
    } else {
        return false;
    }

    let hcl = &passport["hcl"];
    if !hcl.starts_with("#") || hcl.len() != 7 {
        return false;
    }
    if !hcl[1..].chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
        return false;
    }

    let ecl = &passport["ecl"];
    if !(ecl == "amb"
        || ecl == "blu"
        || ecl == "brn"
        || ecl == "gry"
        || ecl == "grn"
        || ecl == "hzl"
        || ecl == "oth")
    {
        return false;
    }

    let pid = &passport["pid"];
    if pid.len() != 9 {
        return false;
    }
    if !pid.chars().all(|c| matches!(c, '0'..='9')) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {}
}
