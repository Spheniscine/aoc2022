use crate::aoc_base::Day;

pub struct DayNUM;

impl Day for DayNUM {
    type Parsed = ();

    type Output1 = &'static str;

    type Output2 = &'static str;

    fn num() -> usize {
        NUM
    }

    fn title() -> &'static str {
        ""
    }

    fn parse(input: &str) -> Self::Parsed {
    }

    fn part1(data: &Self::Parsed) -> Self::Output1 {
        "TODO"
    }

    fn part2(data: &Self::Parsed) -> Self::Output2 {
        "TODO"
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        //DayNUM::test(InputSource::gmail(NUM), Some(1553), Some(1597))
    }
}