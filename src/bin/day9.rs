use aoc::Solution;
use itertools::Itertools;

struct Day9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s
            .split(",")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect_tuple()
            .unwrap();
        Point { x, y }
    }
}

impl Point {
    fn rec_area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl Solution<u64> for Day9 {
    type Input = Vec<Point>;

    fn parse_input() -> Self::Input {
        include_str!("./9.txt").lines().map(Point::from).collect()
    }

    fn part1(input: Self::Input) -> u64 {
        input
            .into_iter()
            .tuple_combinations()
            .map(|(p1, p2)| p1.rec_area(&p2))
            .max()
            .unwrap()
    }

    fn part2(input: Self::Input) -> u64 {
        todo!()
    }
}

fn main() {
    Day9::run();
}
