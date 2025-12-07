use std::ops::RangeInclusive;

use aoc::Solution;

struct Day5;

impl Solution<usize> for Day5 {
    type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);

    fn parse_input() -> Self::Input {
        let mut lines = include_str!("./5.txt").lines();
        let ranges = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (start, end) = l.split_once('-').unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .collect();
        let ids = lines.map(|l| l.parse().unwrap()).collect();
        (ranges, ids)
    }

    fn part1(input: Self::Input) -> usize {
        let (ranges, ids) = input;
        ids.into_iter()
            .filter(|id| ranges.iter().any(|r| r.contains(id)))
            .count()
    }

    fn part2((mut ranges, _): Self::Input) -> usize {
        ranges.sort_by_key(|r| *r.start());
        let (first, ranges) = ranges.split_first().unwrap();
        ranges
            .iter()
            .fold(vec![first.clone()], |mut v, r2| {
                let r1 = v.last_mut().unwrap();
                if r1.contains(r2.start()) {
                    *r1 = *r1.start()..=*r2.end().max(r1.end());
                    v
                } else {
                    v.push(r2.clone());
                    v
                }
            })
            .into_iter()
            .map(|r| r.end().abs_diff(*r.start()) + 1)
            .sum()
    }
}

fn main() {
    Day5::run();
}
