//! https://adventofcode.com/2020/day/11

use reformation::Reformation;

fn main() {
    let input = include_str!("../../input/2020/day11.txt");
    let seats = parse(input);

    println!("One: {}", solve(seats.clone()));
    println!("Two: {}", solve2(seats));
}

#[derive(Reformation, Clone, Debug)]
enum Seat {
    #[reformation("L")]
    Empty,
    #[reformation("#")]
    Occupied,
    #[reformation(".")]
    Floor,
}

fn parse(input: &str) -> Vec<Vec<Seat>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| Seat::parse(&String::from(ch)).unwrap())
                .collect()
        })
        .collect()
}

fn solve(mut grid: Vec<Vec<Seat>>) -> usize {
    let mut changed = true;
    while changed {
        changed = false;

        let mut new_grid = Vec::new();

        for row in 0i32..grid.len() as i32 {
            let mut new_row = Vec::new();
            for col in 0i32..grid[row as usize].len() as i32 {
                let new_seat = match grid[row as usize][col as usize] {
                    Seat::Empty => {
                        if adjacent_occupied(&grid, row, col) == 0 {
                            changed = true;
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if adjacent_occupied(&grid, row, col) >= 4 {
                            changed = true;
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                    Seat::Floor => Seat::Floor,
                };
                new_row.push(new_seat);
            }
            new_grid.push(new_row);
        }

        grid = new_grid;
    }

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count()
        })
        .sum()
}

fn solve2(mut grid: Vec<Vec<Seat>>) -> usize {
    let mut changed = true;
    while changed {
        changed = false;

        let mut new_grid = Vec::new();

        for row in 0i32..grid.len() as i32 {
            let mut new_row = Vec::new();
            for col in 0i32..grid[row as usize].len() as i32 {
                let new_seat = match grid[row as usize][col as usize] {
                    Seat::Empty => {
                        if see_occupied(&grid, row, col) == 0 {
                            changed = true;
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if see_occupied(&grid, row, col) >= 5 {
                            changed = true;
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                    Seat::Floor => Seat::Floor,
                };
                new_row.push(new_seat);
            }
            new_grid.push(new_row);
        }

        grid = new_grid;
    }

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count()
        })
        .sum()
}

fn see_occupied(grid: &Vec<Vec<Seat>>, row: i32, col: i32) -> usize {
    check_direction(grid, row, col, -1, -1)
        + check_direction(grid, row, col, -1, 0)
        + check_direction(grid, row, col, -1, 1)
        + check_direction(grid, row, col, 0, -1)
        + check_direction(grid, row, col, 0, 1)
        + check_direction(grid, row, col, 1, -1)
        + check_direction(grid, row, col, 1, 0)
        + check_direction(grid, row, col, 1, 1)
}

fn check_direction(
    grid: &Vec<Vec<Seat>>,
    mut row: i32,
    mut col: i32,
    row_dir: i32,
    col_dir: i32,
) -> usize {
    loop {
        row += row_dir;
        col += col_dir;
        if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32 {
            return 0;
        }

        match grid
            .get(row as usize)
            .unwrap_or(&Vec::new())
            .get(col as usize)
        {
            Some(Seat::Occupied) => return 1,
            Some(Seat::Empty) => return 0,
            _ => {}
        }
    }
}

fn adjacent_occupied(grid: &Vec<Vec<Seat>>, row: i32, col: i32) -> usize {
    occupied(&grid, row - 1, col - 1)
        + occupied(&grid, row - 1, col)
        + occupied(&grid, row - 1, col + 1)
        + occupied(&grid, row, col - 1)
        + occupied(&grid, row, col + 1)
        + occupied(&grid, row + 1, col - 1)
        + occupied(&grid, row + 1, col)
        + occupied(&grid, row + 1, col + 1)
}

fn occupied(grid: &Vec<Vec<Seat>>, row: i32, col: i32) -> usize {
    if row < 0 || col < 0 {
        return 0;
    }
    match grid
        .get(row as usize)
        .unwrap_or(&Vec::new())
        .get(col as usize)
    {
        Some(Seat::Occupied) => 1,
        _ => 0,
    }
}

fn print(grid: &Vec<Vec<Seat>>) {
    for row in grid {
        for seat in row {
            match seat {
                Seat::Occupied => print!("#"),
                Seat::Empty => print!("L"),
                Seat::Floor => print!("."),
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

        let parsed = parse(input);
        assert_eq!(solve(parsed.clone()), 37);
        assert_eq!(solve2(parsed), 26);
    }
}
