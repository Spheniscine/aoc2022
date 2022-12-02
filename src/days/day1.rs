use crate::aoc_base::Day;

pub struct Day1;

impl Day for Day1 {
    type Parsed = Vec<Vec<i64>>;

    type Output1 = i64;

    type Output2 = i64;

    fn num() -> usize {
        1
    }

    fn title() -> &'static str {
        "Calorie Counting"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.split("\n\n").map(|sec| 
            sec.lines().map(|v| v.parse().unwrap()).collect()
        ).collect()
    }

    fn part1(data: &Self::Parsed) -> Self::Output1 {
        data.iter().map(|s| s.iter().sum()).max().unwrap()
    }

    fn part2(data: &Self::Parsed) -> Self::Output2 {
        let mut a = data.iter().map(|s| s.iter().sum::<i64>()).collect::<Vec<_>>();
        a.sort();
        a.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day1::test(InputSource::gmail(1), Some(70613), Some(205805))
    }
}