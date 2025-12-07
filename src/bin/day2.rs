use std::ops::RangeInclusive;

use aoc::Solution;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Day2;

impl Solution<usize> for Day2 {
    type Input = Vec<RangeInclusive<usize>>;

    fn parse_input() -> Self::Input {
        include_str!("./2.txt")
            .trim()
            .split(',')
            .map(|l| {
                l.split_once('-')
                    .map(|(start, end)| start.parse().unwrap()..=end.parse().unwrap())
                    .unwrap()
            })
            .collect()
    }

    fn part1(input: Self::Input) -> usize {
        solve(input, |id| {
            let id = id.as_bytes();
            let (first, second) = id.split_at(id.len() / 2);
            first == second
        })
    }

    fn part2(input: Self::Input) -> usize {
        solve(input, |id| {
            let id = id.as_bytes();
            for size in 1..=id.len() / 2 {
                let chunks = id.chunks_exact(size);
                if chunks.remainder().is_empty() && chunks.unique().count() == 1 {
                    return true;
                }
            }
            false
        })
    }
}

fn solve<F>(input: <Day2 as Solution<usize>>::Input, is_invalid_id: F) -> usize
where
    F: Fn(&str) -> bool + Sync + Send,
{
    input
        .into_par_iter()
        .flat_map(|r| {
            r.into_par_iter().filter(|id| {
                let id = id.to_string();
                is_invalid_id(&id)
            })
        })
        .sum()
}

fn main() {
    Day2::run();
}
