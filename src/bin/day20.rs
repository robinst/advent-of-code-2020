//! https://adventofcode.com/2020/day/20

use itertools::Itertools;
use nom::lib::std::fmt::Formatter;
use num::integer::sqrt;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::{fmt, mem};

const TILE_LENGTH: usize = 10;

fn main() {
    let input = include_str!("../../input/2020/day20.txt");

    let puzzle = parse(input);

    let part1 = solve(&puzzle);
    println!("One: {}", part1.0);
    println!("Two: {}", solve2(&part1.1, part1.2));
}

fn parse(input: &str) -> Puzzle {
    let mut tiles = HashMap::new();

    for tile_str in input.split("\n\n") {
        let mut parts = tile_str.splitn(2, "\n");
        let header = parts.next().unwrap();
        let rest = parts.next().unwrap();

        let id = header
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let mut content = HashSet::new();
        for (row, line) in rest.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    content.insert((col, row));
                }
            }
        }

        tiles.insert(id, Tile::new(id, content));
    }

    Puzzle { tiles }
}

fn solve(puzzle: &Puzzle) -> (u64, HashSet<(usize, usize)>, usize) {
    let side_length = sqrt(puzzle.tiles.len());

    let mut grid: HashMap<(usize, usize), ArrangedTile> = HashMap::new();
    let mut candidates: HashSet<u64> = puzzle.tiles.keys().copied().collect();

    let coords: Vec<(usize, usize)> = (0..side_length).cartesian_product(0..side_length).collect();
    assert_eq!(coords.len(), puzzle.tiles.len());
    assert!(arrange(
        &mut candidates,
        &mut grid,
        0,
        &coords,
        &puzzle.tiles,
    ));

    let result = grid[&(0, 0)].id
        * grid[&(0, side_length - 1)].id
        * grid[&(side_length - 1, 0)].id
        * grid[&(side_length - 1, side_length - 1)].id;

    let mut set = HashSet::new();
    for y in 0..side_length {
        for x in 0..side_length {
            let ArrangedTile { id, rotation, flip } = grid[&(x, y)];
            let content = puzzle.tiles[&id].content(rotation, flip);
            for (ny, ty) in (1..TILE_LENGTH - 1).enumerate() {
                for (nx, tx) in (1..TILE_LENGTH - 1).enumerate() {
                    if content.contains(&(tx, ty)) {
                        set.insert((x * 8 + nx, y * 8 + ny));
                    }
                }
            }
        }
    }

    (result, set, side_length * 8)
}

fn arrange(
    candidates: &mut HashSet<u64>,
    grid: &mut HashMap<(usize, usize), ArrangedTile>,
    coord: usize,
    coords: &Vec<(usize, usize)>,
    tiles: &HashMap<u64, Tile>,
) -> bool {
    if coord >= coords.len() {
        return true;
    }

    let (y, x) = coords[coord];
    let ids: Vec<u64> = candidates.iter().copied().collect();
    for id in ids {
        let tile = &tiles[&id];
        for &flip in &[false, true] {
            for rotation in 0..4 {
                if y != 0 {
                    let top = tile.side(0, rotation, flip);
                    let ArrangedTile {
                        id: other,
                        rotation: r,
                        flip: f,
                    } = grid[&(x, y - 1)];
                    if tiles[&other].side(2, r, f) != top {
                        continue;
                    }
                }
                if x != 0 {
                    let left = tile.side(3, rotation, flip);
                    let ArrangedTile {
                        id: other,
                        rotation: r,
                        flip: f,
                    } = grid[&(x - 1, y)];
                    if tiles[&other].side(1, r, f) != left {
                        continue;
                    }
                }
                // println!(
                //     "Tile {} fits at coord ({}, {}) with rotation {} and flip {}:\n{}",
                //     id, x, y, rotation, flip, tile
                // );
                candidates.remove(&id);
                grid.insert((x, y), ArrangedTile { id, rotation, flip });
                if arrange(candidates, grid, coord + 1, coords, tiles) {
                    return true;
                }
                grid.remove(&(x, y));
                candidates.insert(id);
            }
        }
    }
    false
}

fn solve2(image: &HashSet<(usize, usize)>, side_length: usize) -> usize {
    for &flip in &[false, true] {
        for rotation in 0..4 {
            let transformed = transform(image, side_length, rotation, flip);
            let monsters = find_monsters(&transformed, side_length);
            if monsters > 0 {
                return image.len() - monsters * 15;
            }
        }
    }
    0
}

/// Monster:
///
/// ```
///                   #
/// #    ##    ##    ###
///  #  #  #  #  #  #
/// ```
fn find_monsters(image: &HashSet<(usize, usize)>, side_length: usize) -> usize {
    let mut count = 0;
    for y in 1..side_length - 1 {
        for x in 0..side_length - 20 {
            if image.contains(&(x, y)) {
                // Could be the tail, check other parts
                if [
                    // Top
                    (x + 18, y - 1),
                    // Middle
                    (x + 5, y),
                    (x + 6, y),
                    (x + 11, y),
                    (x + 12, y),
                    (x + 17, y),
                    (x + 17, y),
                    (x + 18, y),
                    (x + 19, y),
                    // Bottom
                    (x + 1, y + 1),
                    (x + 4, y + 1),
                    (x + 7, y + 1),
                    (x + 10, y + 1),
                    (x + 13, y + 1),
                    (x + 16, y + 1),
                ]
                .iter()
                .all(|p| image.contains(p))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

#[derive(Clone, Debug)]
struct Puzzle {
    tiles: HashMap<u64, Tile>,
}

#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    content: HashSet<(usize, usize)>,
    sides: Vec<u32>,
}

impl Tile {
    fn new(id: u64, content: HashSet<(usize, usize)>) -> Tile {
        let mut sides = Vec::new();
        // Clockwise sides
        sides.push(num((0..TILE_LENGTH)
            .map(|x| content.contains(&(x, 0)))
            .collect()));
        sides.push(num((0..TILE_LENGTH)
            .map(|y| content.contains(&(9, y)))
            .collect()));
        sides.push(num((0..TILE_LENGTH)
            .rev()
            .map(|x| content.contains(&(x, 9)))
            .collect()));
        sides.push(num((0..TILE_LENGTH)
            .rev()
            .map(|y| content.contains(&(0, y)))
            .collect()));
        Tile { id, content, sides }
    }

    fn side(&self, side: u8, rotation: u8, mut flip: bool) -> u32 {
        let index = if flip {
            ((8 - side - rotation) % 4) as usize
        } else {
            ((side + rotation) % 4) as usize
        };
        let mut num = self.sides[index];
        if side == 2 || side == 3 {
            // For e.g. the bottom side, we want the number to read from left to right too, so we
            // can compare with a top side. Because we store the sides in clockwise reading order,
            // we need to flip the bottom and left side.
            flip ^= true;
        }
        if !flip {
            num
        } else {
            let mut rev = 0;
            for _ in 0..TILE_LENGTH {
                rev <<= 1;
                if num & 1 == 1 {
                    rev ^= 1;
                }
                num >>= 1;
            }
            rev
        }
    }

    fn content(&self, rotation: u8, flip: bool) -> HashSet<(usize, usize)> {
        transform(&self.content, TILE_LENGTH, rotation, flip)
    }
}

fn transform(
    positions: &HashSet<(usize, usize)>,
    side_length: usize,
    rotation: u8,
    flip: bool,
) -> HashSet<(usize, usize)> {
    let (invert_x, invert_y, swap) = match (rotation, flip) {
        (0, false) => (false, false, false),
        (1, false) => (true, false, true),
        (2, false) => (true, true, false),
        (3, false) => (false, true, true),
        (0, true) => (true, false, false),
        (1, true) => (false, false, true),
        (2, true) => (false, true, false),
        (3, true) => (true, true, true),
        (r, _) => panic!("Unknown rotation {}", r),
    };

    let mut result = HashSet::new();
    for (mut x, mut y) in positions {
        if invert_x {
            x = side_length - 1 - x;
        }
        if invert_y {
            y = side_length - 1 - y;
        }
        if swap {
            mem::swap(&mut x, &mut y);
        }
        result.insert((x, y));
    }
    result
}

fn print(positions: &HashSet<(usize, usize)>, side_length: usize) {
    for y in 0..side_length {
        for x in 0..side_length {
            let dot = if positions.contains(&(x, y)) {
                "#"
            } else {
                "."
            };
            print!("{}", dot);
        }
        println!();
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..TILE_LENGTH {
            for x in 0..TILE_LENGTH {
                let c = if self.content.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

struct ArrangedTile {
    id: u64,
    rotation: u8,
    flip: bool,
}

fn num(bits: Vec<bool>) -> u32 {
    let mut num = 0;
    for bit in bits {
        num <<= 1;
        num += if bit { 1 } else { 0 };
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"#;

        let puzzle = parse(input);

        // Tile 2311:
        // ..##.#..#.
        // ##..#.....
        // #...##..#.
        // ####.#...#
        // ##.##.###.
        // ##...#.###
        // .#.#.#..##
        // ..#....#..
        // ###...#.#.
        // ..###..###

        let tile = &puzzle.tiles[&2311];
        assert_eq!(tile.sides, vec![210, 89, 924, 318]);
        assert_eq!(tile.side(0, 0, false), 210);
        assert_eq!(tile.side(1, 0, false), 89);
        assert_eq!(tile.side(2, 0, false), 231);
        assert_eq!(tile.side(3, 0, false), 498);

        assert_eq!(tile.side(0, 1, false), 89);
        assert_eq!(tile.side(1, 1, false), 924);
        assert_eq!(tile.side(2, 1, false), 498);
        assert_eq!(tile.side(3, 1, false), 300);

        assert_eq!(tile.side(0, 0, true), 300);
        assert_eq!(tile.side(1, 0, true), 498);
        assert_eq!(tile.side(2, 0, true), 924);
        assert_eq!(tile.side(3, 0, true), 89);
        // first: Tile 1951 fits at coord (0, 0) with rotation 0 and flip true
        let result = solve(&puzzle);
        assert_eq!(result.0, 20899048083289);

        assert_eq!(solve2(&result.1, result.2), 273);
    }
}
