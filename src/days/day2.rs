use std::cmp::Ordering;

use crate::aoc_base::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock, Paper, Scissors
}
use Hand::*;
use Ordering::*;

impl Hand {
    fn cmp(self, b: Hand) -> Ordering {
        if self == b { return Equal; }
        if b == match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        } {Greater} else {Less}
    }

    fn score(self, b: Hand) -> i32 {
        let shape_score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let outcome_score = match self.cmp(b) {
            Less => 0,
            Equal => 3,
            Greater => 6,
        };
        shape_score + outcome_score
    }
}

pub struct Day2;

impl Day for Day2 {
    type Parsed = Vec<(char, char)>;

    type Output1 = i32;

    type Output2 = i32;

    fn num() -> usize {
        2
    }

    fn title() -> &'static str {
        "Rock Paper Scissors"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.split("\n").map(|s| (s.as_bytes()[0] as char, s.as_bytes()[2] as char)).collect()
    }

    fn part1(data: &Self::Parsed) -> Self::Output1 {
        data.iter().map(|&(x, y)| {
            let a = match x {
                'A' => Rock,
                'B' => Paper,
                _ => Scissors,
            };
            let b = match y {
                'X' => Rock,
                'Y' => Paper,
                _ => Scissors,
            };
            b.score(a)
        }).sum()
    }

    fn part2(data: &Self::Parsed) -> Self::Output2 {
        data.iter().map(|&(x, y)| {
            let a = match x {
                'A' => Rock,
                'B' => Paper,
                _ => Scissors,
            };
            let b = match y {
                'X' => match a {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                },
                'Y' => a,
                _ => match a {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                },
            };
            b.score(a)
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day2::test(InputSource::gmail(2), Some(12855), Some(13726))
    }
}