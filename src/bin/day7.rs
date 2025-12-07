use std::collections::{HashMap, HashSet};

use aoc::{
    Solution,
    grid::{Coord, Grid},
};
use itertools::Itertools;
struct Day7;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Start,
    Splitter,
    Empty,
    Beam,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            '.' => Cell::Empty,
            '|' => Cell::Beam,
            c => unreachable!("{}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Path {
    Left,
    Right,
}

impl Solution<usize> for Day7 {
    type Input = Grid<Cell>;

    fn parse_input() -> Self::Input {
        Grid::from_input(include_str!("./7.txt"))
    }

    fn part1(mut grid: Self::Input) -> usize {
        use Cell::*;
        let start = grid
            .iter()
            .find_map(|(coord, cell)| matches!(cell, Start).then_some(*coord))
            .unwrap();
        let mut beam_ends = vec![start];
        let mut splits = 0;
        // simulation: each iter will move the beam down one spot.
        // it stops if the beams can't move anymore.
        loop {
            // dbg!(splits, &beam_ends);
            beam_ends = beam_ends
                .into_iter()
                .flat_map(|start| {
                    let end = start.south(1);
                    match grid.at(end).cloned() {
                        None => vec![].into_iter(),
                        Some(Splitter) => {
                            let ends = split_at(&mut grid, end);
                            if !ends.is_empty() {
                                splits += 1;
                            }
                            ends.into_iter()
                        }
                        Some(_) => {
                            grid.insert(end, Beam);
                            vec![end].into_iter()
                        }
                    }
                })
                .collect();
            if beam_ends.is_empty() {
                break;
            }
        }
        splits
    }

    fn part2(grid: Self::Input) -> usize {
        fn next_end(grid: &Grid<Cell>, start: Coord) -> (Coord, bool) {
            let mut cur = start;
            loop {
                cur = cur.south(1);
                match grid.at(cur) {
                    Some(Cell::Splitter) => return (cur, true),
                    None => return (cur, false),
                    Some(_) => (),
                }
            }
        }
        fn fill_paths(
            paths: &mut HashMap<(Coord, Path), Coord>,
            grid: &Grid<Cell>,
            start: Coord,
            path: Path,
            (end, is_splitter): (Coord, bool),
        ) {
            if paths.contains_key(&(start, path)) {
                return;
            }
            paths.insert((start, path), end);
            if is_splitter {
                fill_paths(paths, grid, end, Path::Left, next_end(grid, end.west(1)));
                fill_paths(paths, grid, end, Path::Right, next_end(grid, end.east(1)));
            }
        }
        let start = grid
            .iter()
            .find_map(|(coord, cell)| matches!(cell, Cell::Start).then_some(*coord))
            .unwrap();
        let first_split = next_end(&grid, start).0;
        let mut paths = HashMap::new();
        fill_paths(
            &mut paths,
            &grid,
            first_split,
            Path::Left,
            next_end(&grid, first_split.west(1)),
        );
        fill_paths(
            &mut paths,
            &grid,
            first_split,
            Path::Right,
            next_end(&grid, first_split.east(1)),
        );
        let mut splits = HashMap::new();
        paths
            .values()
            .sorted()
            .rev()
            .cloned()
            .chain(vec![first_split])
            .for_each(|c| {
                let left_end = paths.get(&(c, Path::Left));
                let right_end = paths.get(&(c, Path::Right));
                match (left_end, right_end) {
                    (None, None) => {
                        splits.insert(start, 0usize);
                    }
                    (Some(left_end), Some(right_end)) => {
                        let left = splits.get(left_end).cloned().unwrap_or_default();
                        let right = splits.get(right_end).cloned().unwrap_or_default();
                        splits.insert(c, 1usize + left + right);
                    }
                    v => unreachable!("{:?}", v),
                }
            });
        1 + *splits.get(&first_split).unwrap()
    }
}

fn split_at(grid: &mut Grid<Cell>, coord: Coord) -> Vec<Coord> {
    use Cell::{Beam, Empty};
    let (left, right) = (coord.west(1), coord.east(1));
    match (grid.at(left).cloned(), grid.at(right).cloned()) {
        (Some(l), Some(r)) => match (l, r) {
            (Empty, Empty) => {
                grid.insert(left, Beam);
                grid.insert(right, Beam);
                vec![left, right]
            }
            (Empty, Beam) => {
                grid.insert(left, Beam);
                vec![left]
            }
            (Beam, Empty) => {
                grid.insert(right, Beam);
                vec![right]
            }
            (Beam, Beam) => vec![],
            _ => unreachable!("{:?}: weird combination", (l, r)),
        },
        (l, r) => unreachable!("{:?} oopsie?", (l, r)),
    }
}

fn main() {
    Day7::run();
}
