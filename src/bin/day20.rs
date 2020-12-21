//! https://adventofcode.com/2020/day/20

use itertools::Itertools;
use nom::lib::std::fmt::Formatter;
use num::integer::sqrt;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;

const TILE_LENGTH: usize = 10;

fn main() {
    let input = include_str!("../../input/2020/day20.txt");

    let puzzle = parse(input);

    println!("One: {}", solve(&puzzle));
}

#[derive(Clone, Debug)]
struct Puzzle {
    tiles: HashMap<u64, Tile>,
}

#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    content: HashMap<(usize, usize), bool>,
    sides: Vec<u32>,
}

impl Tile {
    fn new(id: u64, content: HashMap<(usize, usize), bool>) -> Tile {
        let mut sides = Vec::new();
        // Clockwise sides
        sides.push(num((0..TILE_LENGTH).map(|x| content[&(x, 0)]).collect()));
        sides.push(num((0..TILE_LENGTH).map(|y| content[&(9, y)]).collect()));
        sides.push(num((0..TILE_LENGTH)
            .rev()
            .map(|x| content[&(x, 9)])
            .collect()));
        sides.push(num((0..TILE_LENGTH)
            .rev()
            .map(|y| content[&(0, y)])
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
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..TILE_LENGTH {
            for x in 0..TILE_LENGTH {
                write!(f, "{}", if self.content[&(x, y)] { '#' } else { '.' })?
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

        let mut content = HashMap::new();
        for (row, line) in rest.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                content.insert((col, row), c == '#');
            }
        }

        tiles.insert(id, Tile::new(id, content));
    }

    Puzzle { tiles }
}

fn solve(puzzle: &Puzzle) -> u64 {
    let side_length = sqrt(puzzle.tiles.len());

    // let mut tiles: HashMap<u64, Tile> = puzzle.tiles.iter().map(|t| (t.id, t)).collect();
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

    grid[&(0, 0)].id
        * grid[&(0, side_length - 1)].id
        * grid[&(side_length - 1, 0)].id
        * grid[&(side_length - 1, side_length - 1)].id
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
        assert_eq!(tile.side(1, 0, trune), 498);
        assert_eq!(tile.side(2, 0, true), 924);
        assert_eq!(tile.side(3, 0, true), 89);
        // first: Tile 1951 fits at coord (0, 0) with rotation 0 and flip true
        assert_eq!(solve(&puzzle), 20899048083289);
    }
}
