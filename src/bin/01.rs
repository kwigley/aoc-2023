use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let chars = &line.chars();
            let left = chars.clone().find(|char| char.is_numeric());
            let right = chars.clone().rfind(|char| char.is_numeric());
            let formatted = format!("{}{}", left.unwrap_or_default(), right.unwrap_or_default());
            formatted.parse::<u32>().unwrap_or_default()
        })
        .sum::<u32>()
        .into()
}
const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let chars = &line.chars();
            let left = chars
                .clone()
                .fold_while("".to_owned(), |acc, c| {
                    if let Some(n) = NUMBERS.iter().position(|n| acc.as_str().contains(n)) {
                        Done((n + 1).to_string())
                    } else if c.is_numeric() {
                        Done(c.to_string())
                    } else {
                        Continue(format!("{}{}", acc, c))
                    }
                })
                .into_inner();
            let right = chars
                .clone()
                .rev()
                .fold_while("".to_owned(), |acc, c| {
                    if let Some(n) = NUMBERS.iter().position(|n| acc.as_str().contains(n)) {
                        Done((n + 1).to_string())
                    } else if c.is_numeric() {
                        Done(c.to_string())
                    } else {
                        Continue(format!("{}{}", c, acc))
                    }
                })
                .into_inner();
            let formatted = format!("{}{}", left, right);
            formatted.parse::<u32>().unwrap_or_default()
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
