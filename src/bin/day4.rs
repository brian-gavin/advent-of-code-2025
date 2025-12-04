use aoc::{
    Solution,
    grid::{Coord, Grid},
};

struct Day4;

type PaperGrid = Grid<bool>;

impl Solution<usize> for Day4 {
    type Input = PaperGrid;

    fn parse_input() -> Self::Input {
        Grid::from_input_fn(include_str!("./4.txt"), |c| c == '@')
    }

    fn part1(input: Self::Input) -> usize {
        input
            .iter()
            .filter(|(c, p)| paper_can_be_removed(&input, c, **p))
            .count()
    }

    fn part2(mut input: Self::Input) -> usize {
        let mut sum = 0;
        loop {
            let candidates = input
                .iter()
                .filter_map(|(c, p)| paper_can_be_removed(&input, c, *p).then_some(c))
                .cloned()
                .collect::<Vec<_>>();
            if candidates.is_empty() {
                return sum;
            }
            sum += candidates.len();
            candidates.into_iter().for_each(|c| {
                let has_paper = input.at_mut(c).unwrap();
                *has_paper = false;
            })
        }
    }
}

fn paper_can_be_removed(grid: &PaperGrid, coord: &Coord, has_paper: bool) -> bool {
    has_paper
        && [
            coord.north(1),
            coord.northeast(1),
            coord.east(1),
            coord.southeast(1),
            coord.south(1),
            coord.southwest(1),
            coord.west(1),
            coord.northwest(1),
        ]
        .iter()
        .filter(|c| grid.at(**c).cloned().unwrap_or_default())
        .count()
        .lt(&4)
}

fn main() {
    Day4::run();
}
