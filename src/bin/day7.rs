use std::collections::HashMap;

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
        use Cell::*;
        let start = grid
            .iter()
            .find_map(|(coord, cell)| matches!(cell, Start).then_some(*coord))
            .unwrap();
        // simulation: each iter will move the current particle down one spot.
        // if it reaches a Splitter, it will simulate starting from the two points.
        // the current timeline will "end", being subsumed by the other two timelines.
        fn simulate(grid: &Grid<Cell>, start: Coord) -> usize {
            let mut cur = start;
            loop {
                cur = cur.south(1);
                match grid.at(cur) {
                    None => {
                        return 1;
                    }
                    Some(Splitter) => {
                        return simulate(grid, cur.west(1)) + simulate(grid, cur.east(1));
                    }
                    Some(_) => (),
                }
            }
        }
        simulate(&grid, start)
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
