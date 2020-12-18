//! https://adventofcode.com/2020/day/18

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};

use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/2020/day18.txt");

    let mut one = 0;
    let mut two = 0;
    for line in input.lines() {
        one += solve(line);
        two += solve2(line);
    }

    println!("One: {}", one);
    println!("Two: {}", two);
}

fn solve(input: &str) -> i64 {
    expr(input).unwrap().1
}

// See https://github.com/Geal/nom/blob/master/tests/arithmetic.rs
fn expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = operand(i)?;

    fold_many0(
        pair(alt((char('+'), char('*'))), operand),
        init,
        |acc, (op, val): (char, i64)| {
            if op == '+' {
                acc + val
            } else {
                acc * val
            }
        },
    )(i)
}

fn operand(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens,
    ))(i)
}

fn parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
}

fn solve2(input: &str) -> i64 {
    times(input).unwrap().1
}

fn times(i: &str) -> IResult<&str, i64> {
    let (i, init) = plus(i)?;

    fold_many0(
        pair(char('*'), plus),
        init,
        |acc, (_op, val): (char, i64)| acc * val,
    )(i)
}

fn plus(i: &str) -> IResult<&str, i64> {
    let (i, init) = operand2(i)?;

    fold_many0(
        pair(char('+'), operand2),
        init,
        |acc, (_op, val): (char, i64)| acc + val,
    )(i)
}

fn operand2(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens2,
    ))(i)
}

fn parens2(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), times, tag(")")), space)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(solve("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve("2 * 3 + (4 * 5)"), 26);
        assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );

        assert_eq!(solve2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(solve2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(solve2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            solve2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
