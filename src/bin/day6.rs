use std::collections::VecDeque;

use aoc::Solution;

struct Day6;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Add,
    Mul,
    Number(u64),
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        use Cell::*;
        match value {
            "+" => Add,
            "*" => Mul,
            s => Number(s.parse().unwrap()),
        }
    }
}

impl Solution<u64> for Day6 {
    type Input = &'static str;

    fn parse_input() -> Self::Input {
        include_str!("6.txt")
    }

    fn part1(input: Self::Input) -> u64 {
        let mut lines = input.lines();
        let mut v: Vec<VecDeque<Cell>> = lines
            .by_ref()
            .nth(0)
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|s| VecDeque::from([s.into()]))
                    .collect()
            })
            .unwrap();
        for line in lines {
            line.split_ascii_whitespace()
                .enumerate()
                .for_each(|(i, s)| {
                    v[i].push_front(s.into());
                });
        }
        v.into_iter()
            .flat_map(|mut v| {
                let op = match v.pop_front().unwrap() {
                    Cell::Add => u64::wrapping_add,
                    Cell::Mul => u64::wrapping_mul,
                    Cell::Number(_) => unreachable!(),
                };
                v.into_iter()
                    .map(|c| match c {
                        Cell::Number(n) => n,
                        _ => unreachable!(),
                    })
                    .reduce(op)
            })
            .sum()
    }

    fn part2(input: Self::Input) -> u64 {
        // parse the input into vectors of columns
        let cols = {
            let mut lines = input.lines();
            let mut cols: Vec<String> = lines
                .by_ref()
                .nth(0)
                .map(|l| l.chars().map(|c| String::from(c)).collect())
                .unwrap();
            lines.for_each(|l| l.char_indices().for_each(|(i, c)| cols[i].push(c)));
            cols
        };
        // split each column into groups of math separated by a column of all space
        let groups = {
            let (mut groups, last) =
                cols.into_iter()
                    .fold((vec![], vec![]), |(mut groups, mut cur), s| {
                        if s.chars().all(|c| c.is_whitespace()) {
                            groups.push(cur);
                            (groups, vec![])
                        } else {
                            cur.push(s);
                            (groups, cur)
                        }
                    });
            groups.push(last);
            groups
        };
        groups
            .into_iter()
            .map(|problem| -> (Vec<u64>, Option<fn(u64, u64) -> u64>) {
                // turn the problem into its numbers and its operator
                problem
                    .into_iter()
                    .fold((vec![], None), |(mut nums, op), s| {
                        let (num, found_op) = s.chars().fold((0, None), |(n, op), c| match c {
                            '0'..='9' => (n * 10 + c.to_digit(10).unwrap() as u64, op),
                            '+' => (n, Some(u64::wrapping_add as fn(u64, u64) -> u64)),
                            '*' => (n, Some(u64::wrapping_mul as fn(u64, u64) -> u64)),
                            _ => (n, op),
                        });
                        nums.push(num);
                        (nums, op.or(found_op))
                    })
            })
            .map(|(nums, op)| (nums, op.unwrap()))
            .filter_map(|(nums, op)| nums.into_iter().reduce(op))
            .sum()
    }
}

fn main() {
    Day6::run();
}
