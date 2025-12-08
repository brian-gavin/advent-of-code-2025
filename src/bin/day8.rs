use std::collections::BinaryHeap;

use aoc::Solution;
use itertools::Itertools;
use petgraph::{
    algo::{connected_components, scc::tarjan_scc},
    prelude::*,
};

struct Day8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> Distance {
        let (j1, j2) = (*self, *other);
        let d = ((j1.x as i64 - j2.x as i64).pow(2)
            + (j1.y as i64 - j2.y as i64).pow(2)
            + (j1.z as i64 - j2.z as i64).pow(2))
        .isqrt();
        Distance { j1, j2, d }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Distance {
    j1: JunctionBox,
    j2: JunctionBox,
    d: i64,
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.d.partial_cmp(&other.d).map(|o| o.reverse())
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution<usize> for Day8 {
    type Input = Vec<JunctionBox>;

    fn parse_input() -> Self::Input {
        include_str!("./8.txt")
            .lines()
            .map(|l| {
                let mut nums = l.split(",").filter_map(|n| n.parse().ok());
                JunctionBox {
                    x: nums.next().unwrap(),
                    y: nums.next().unwrap(),
                    z: nums.next().unwrap(),
                }
            })
            .collect()
    }

    fn part1(mut junction_boxes: Self::Input) -> usize {
        const SAMPLE: bool = false;
        const N: usize = if SAMPLE { 10 } else { 1000 };

        junction_boxes.sort();
        let mut g = UnGraphMap::<JunctionBox, i64>::with_capacity(junction_boxes.len(), N);
        junction_boxes.iter().for_each(|j| {
            g.add_node(*j);
        });
        let mut distances: BinaryHeap<Distance> = junction_boxes
            .iter()
            .tuple_combinations()
            .map(|(j1, j2)| j1.distance(j2))
            .collect();
        for _ in 0..N {
            let Distance { j1, j2, d } = distances.pop().unwrap();
            g.add_edge(j1, j2, d);
        }
        tarjan_scc(&g)
            .into_iter()
            .map(|g| g.len())
            .k_largest(3)
            .product()
    }

    fn part2(mut junction_boxes: Self::Input) -> usize {
        junction_boxes.sort();
        let mut distances: BinaryHeap<Distance> = junction_boxes
            .iter()
            .tuple_combinations()
            .map(|(j1, j2)| j1.distance(j2))
            .collect();
        let mut g =
            UnGraphMap::<JunctionBox, i64>::with_capacity(junction_boxes.len(), distances.len());
        junction_boxes.iter().for_each(|j| {
            g.add_node(*j);
        });
        loop {
            let Distance { j1, j2, d } = distances.pop().unwrap();
            g.add_edge(j1, j2, d);
            if connected_components(&g) == 1 {
                return j1.x * j2.x;
            }
        }
    }
}

fn main() {
    Day8::run();
}
