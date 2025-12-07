use std::collections::HashSet;

use aoc::{
    Solution,
    grid::{Coord, Grid},
};
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

impl Solution<usize> for Day7 {
    type Input = Grid<Cell>;

    fn parse_input() -> Self::Input {
        Grid::from_input(include_str!("./7.sample.txt"))
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
            paths: &mut HashSet<(Coord, Coord)>,
            grid: &Grid<Cell>,
            start: Coord,
            (end, is_splitter): (Coord, bool),
        ) {
            if paths.contains(&(start, end)) {
                return;
            }
            paths.insert((start, end));
            if is_splitter {
                fill_paths(paths, grid, end, next_end(grid, end.west(1)));
                fill_paths(paths, grid, end, next_end(grid, end.east(1)));
            }
        }
        let start = grid
            .iter()
            .find_map(|(coord, cell)| matches!(cell, Cell::Start).then_some(*coord))
            .unwrap();
        let first_split = next_end(&grid, start).0;
        let mut paths = HashSet::new();
        fill_paths(
            &mut paths,
            &grid,
            first_split,
            next_end(&grid, first_split.west(1)),
        );
        fill_paths(
            &mut paths,
            &grid,
            first_split,
            next_end(&grid, first_split.east(1)),
        );
        paths.len()
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
