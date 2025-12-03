use aoc::Solution;

struct Day3;

impl Solution<u64> for Day3 {
    type Input = Vec<Vec<u64>>;

    fn parse_input() -> Self::Input {
        include_str!("./3.txt")
            .trim()
            .lines()
            .map(parse_bank)
            .collect()
    }

    fn part1(input: Self::Input) -> u64 {
        input.into_iter().map(|bank| max_joltage::<2>(&bank)).sum()
    }

    fn part2(input: Self::Input) -> u64 {
        input.into_iter().map(|bank| max_joltage::<12>(&bank)).sum()
    }
}

fn parse_bank(s: &str) -> Vec<u64> {
    s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
}

fn max_joltage<const DIGITS: usize>(bank: &[u64]) -> u64 {
    fn next_max(bank: &[u64], n: usize, accum: &mut [u64]) {
        let (i, digit) = bank
            // make sure we pick from a slice that will have enough digits for the next
            .split_at(bank.len().checked_sub(n).unwrap_or_default())
            .0
            .iter()
            .cloned()
            .enumerate()
            .rev()
            .max_by(|(_, n1), (_, n2)| n1.cmp(n2))
            .unwrap();
        accum[n] = digit;
        if n == 0 {
            return;
        }
        next_max(bank.split_at(i + 1).1, n - 1, accum);
    }
    let digits = &mut [0u64; DIGITS];
    next_max(bank, DIGITS - 1, digits);
    digits
        .iter()
        .enumerate()
        .map(|(i, n)| n * 10u64.pow(i as u32))
        .sum()
}

fn main() {
    Day3::run()
}

#[cfg(test)]
mod tests {
    use crate::{max_joltage, parse_bank};

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage::<2>(&[1, 2, 3, 4, 5]), 45);
        assert_eq!(max_joltage::<2>(&[5, 4, 3, 2, 1]), 54);
        assert_eq!(max_joltage::<2>(&parse_bank("55")), 55);
        assert_eq!(max_joltage::<2>(&parse_bank("987654321111111")), 98);
        assert_eq!(max_joltage::<2>(&parse_bank("811111111111119")), 89);
        assert_eq!(max_joltage::<2>(&parse_bank("234234234234278")), 78);
        assert_eq!(max_joltage::<2>(&parse_bank("818181911112111")), 92);
    }
}
